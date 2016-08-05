/*  Periodically crawl web pages and alert the user of changes 
 *  Copyright (C) 2016  Owen Stenson
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>. 
 *
 *  More information in the enclosed `LICENSE' file
 */

//Retrieve data from web page and compare it to cached version

extern crate hyper;
extern crate diff;
extern crate select;

use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};

use self::hyper::header::*;
use self::select::predicate::Name;
use self::select::document::Document;

//use a descriptive user agent? or a generic one?
const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
//What qualifies as html that's too long for LCS, 
//which should instead be compared tag-by-tag
const DIFF_THRESHOLD: usize = 10_000;

pub fn diff_text(old: &str, new: &str) -> String {
    //even w/ dynamic programming, this doesn't scale great
    //a 521kb String took >3 minutes w/ 100% cpu on 1 core
    //takes about 1/2 a second for a 1.4k String,
    // about as long as a 13-byte String
    let delta = diff::chars(&old, &new);
    rhs_of_diff(&old, &new)
    //let mut recent = String::new();
    //for diff in delta {
    //    if let diff::Result::Right(change) = diff {
    //        recent.push('\n');
    //        recent.push_str(change);
    //    }
    //}
    //recent
}

pub fn rhs_of_diff(l: &str, r: &str) -> String {
    //TODO: maybe include context?
    let mut rhs = String::new();
    let differences = diff::chars(&l, &r);
    let mut continuous = true;
    for diff in differences {
        if let diff::Result::Right(change) = diff {
            rhs.push(change)
        } else if continuous {
            //add newlines if `Both` or `Left` separates two `Right`s
            rhs.push('\n');
            continuous = false;
        }
    }
    rhs

}

pub fn decompose_and_diff(old: &str, new: &str) -> String {
    //use html5ever to extract text, then only diff the content
    //shouldn't care about (certain) formatting or javascript
    //only use content tags like `p`, `hN`, ... ?
    //`title`, `p`, `h1..h6`, `div/span`?, `ol`, `ul`, `li`?
    let tags = vec![Name("title"), Name("h1"), 
                    Name("h2"), Name("h3"), Name("h4"), 
                    Name("h5"), Name("h6"), Name("p")];
    let old_html = select::document::Document::from(old);
    let new_html = select::document::Document::from(new);
    let mut diff = String::new();
    for tag in tags {
        let old_elems = old_html.find(tag);
        let new_elems = new_html.find(tag);
        let elems = old_elems.iter().zip(new_elems.iter());
        for (old_elem, new_elem) in elems {
            let diff_elem = rhs_of_diff(&old_elem.text(),
                                        &new_elem.text());
            diff.push_str(&diff_elem);
            diff.push('\n');
            //let deltas = diff::chars(&old_elem.text(), &new_elem.text());
            //for delta in deltas {
            //    if let diff::Result::Right(change) = delta { 
            //        diff.push(change);
            //    }
            //}
            ////only sometimes
            //diff.push('\n');
        }
    }
    diff
}


pub fn compare(url: hyper::Url) -> Result<String,String> {
    //`old` has been converted to bytes, is that a problem?
    let filename = url_to_str(&url);
    let old = get_cache(&filename);
    //try to download new content
    let new = match get_url(url) {
        Ok(n)  => n,
        Err(e) => return Err(format!("Failed to fetch new page: {}", e)),
    };
    //try to look up cached content
    let old = match old {
        Ok(o)  => o,
        Err(e) => 
            //no `old` file to compare to; (try to) create one and throw an Err
            return match set_cache(&filename, &new) {
                Ok(_)  => Err(format!("Cache absent ({}); new one made", 
                                      e.to_string())),
                Err(f) => Err(format!("Cache absent ({});creation failed: {}", 
                                      e, f).to_string()),
            }
    };

    //decide which method to use to diff text
    //TODO: maybe recognize JSON and treat it differently?
    println!("Using `decompose_and_diff`: {}", new.len() > DIFF_THRESHOLD);
    let diff_fn = match new.len() > DIFF_THRESHOLD {
        true  => decompose_and_diff,
        false => diff_text,
    };
    let diff = diff_fn(&old, &new);
    if diff.is_empty() {
        Err("No change".to_string())
    } else {
        //new != old
        match set_cache(&filename, &new) {
            Ok(_)  => Ok(diff),
            Err(e) => Err(format!("Page changed, but couldn't be cached: {}", 
                                  e).to_string()),
        }
    }
}

pub fn set_cache(filename: &str, contents: &str) -> Result<(),String> {
    //create or replace old file
    let mut file = match File::create(filename) {
        Ok(f)  => f,
        Err(e) => return Err(format!("Failed to create cache: {}", 
                                     e.description().to_string())),
    };
    match file.write_all(contents.as_bytes()) {
        Ok(_)  => Ok(()),
        Err(e) => Err(format!("Failed to write cache: {}", 
                              e.description().to_string())),
    }
}

pub fn url_to_str(url: &hyper::Url) -> String {
    //try to make descriptive name out of url to use for file cache
    //can't just use the domain, because there could be collisions
    //if we can't, just use the url itself (without the forward slashes)
    //TODO: use domain in fromt of path segments
    //TODO: make sure title is at least like 1 or 2 parts long
    //TODO: add `directory` part to config grammar?
    //TODO: collisions are still not uncommon. Maybe use base64 of a 
    // hash of the url and Time object?
    let split = url.path_segments();
    if let Some(s) = split {
        if let Some(d) = url.domain() {
            let parts: Vec<&str> = s.into_iter().collect();
            let mut sum = d.to_owned();
            sum.push_str(&parts.join("_"));
            if parts.len() > 1 {
                return sum;
            }
        }
    } 
    //"/" and \0 are the only invalid characters in a filename
    url.as_str().replace("/", "_")
}

pub fn get_cache(filename: &str) -> Result<String,String> {
    //open cached version of a page. Return the html or an error message
    //let filename = url_to_str(url);
    let mut file = match File::open(filename) {
        Err(e) => return Err(format!("File open error: {}",
                                     e.description()
                                     .to_string())),
        Ok(f)  => f,
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(e) => Err(format!("File open error: {}",
                              e.description() 
                              .to_string())),
        Ok(_)  => Ok(text),
    }
}

pub fn get_url(url: hyper::Url) -> Result<String,String> {
    //perform GET request on url. Return the html or an error message
    let client = hyper::Client::new();
    let mut headers = Headers::new();
    //headers.set(ContentType::form_url_encoded());
    //TODO: learn more about headers
    //Accept?
    //Accept-Language?
    //Accept-Encoding?
    //Connection keep-alive?
    headers.set(UserAgent(USER_AGENT.to_string()));
    let res = client.get(url)
                    .headers(headers)
                    .send();
    match res {
        Err(e) => Err(format!("Request error: {}", 
                              e.description().to_string())),
        Ok(mut r)  => {
            let mut text = String::new();
            match r.read_to_string(&mut text) {
                Err(e) => Err(format!("Read error: {}", 
                                      e.description().to_string())),
                Ok(_)  => Ok(text),
            }
        }
    }
}
