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
use std::path::PathBuf;

use self::hyper::header::*;
use self::select::predicate::Name;


//use a descriptive user agent? or a generic one?
const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
//What qualifies as html that's too long for LCS, 
//which should instead be compared tag-by-tag
const DIFF_THRESHOLD: usize = 10_000;

/*
 * TODO List: To make diffs better:
 *   * Add context to changes
 *   * Add smarter way to decide whether to decompose or diff the whole thing
 *      e.g. JSON
 *   * Add more relevant html tags?
 */  


fn rhs_of_diff(l: &str, r: &str) -> String {
    //`diff` `l` and `r`, and append all the unique parts of `r` 
    // to `buffer`
    //TODO: maybe include context? A dozen characters before/after?
    let differences = diff::chars(&l, &r);
    let mut rhs = String::new();
    let mut continuous = false;
    for diff in differences {
        if let diff::Result::Right(change) = diff {
            rhs.push(change);
            continuous = true;
        } else if continuous {
            //add newlines if `Both` or `Left` separates two `Right`s
            rhs.push('\n');
            continuous = false;
        }
    }
    rhs
}

fn decompose_and_diff(old: &str, new: &str) -> String {
    //use html5ever to extract text, then only diff the content
    //shouldn't care about (certain) formatting or javascript
    //only use content tags like `p`, `hN`, ... ?
    //`title`, `p`, `h1..h6`, `div/span`?, `ol`, `ul`, `li`?
    //This needs to append a lot, so it can't really use a buffer (right?)
    // Most `read_to_string` calls like `to_vec` and modifies that
    // Should I try to do something like that?
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
        for (old, new) in elems {
            let diff_buf = rhs_of_diff(&old.text(), &new.text());
            diff.push_str(&diff_buf);
            if diff_buf.is_empty() == false {
                diff.push('\n');
            }
        }
    }
    diff
}


/*
pub fn compare(url: hyper::Url, path: &str) -> Result<String,String> {
    //`old` has been converted to bytes, is that a problem?
    //TODO: verify old cache file exists before this
    //let filename = url_to_str(&url);
    //create buffers for the `old` and `new` htmls, and attempt to load them
    //BUG: we need the full difference also
    //DEPRECATED
    let mut old_txt = String::new();
    if let Err(e) = get_cache(path, &mut old_txt) {
        return Err(format!("Failed to open cache: {}", e))
    }
    let mut new_txt = String::new();
    if let Err(e) = get_url(url, &mut new_txt) {
        return Err(format!("Failed to fetch new page: {}", e))
    }

    //decide which method to use to diff text
    //TODO: maybe recognize JSON and treat it differently?
    let diff_fn = match new_txt.len() > DIFF_THRESHOLD {
        true  => decompose_and_diff,
        false => rhs_of_diff,
    };
    Ok(diff_fn(&old_txt, &new_txt))
}*/

pub fn diff(old: &str, new: &str) -> String {
    //master differ
    //decide which diff fn to use and use it
    //TODO: better way to decide which way to diff
    let diff_fn = match new.len() > DIFF_THRESHOLD {
        true  => decompose_and_diff,
        false => rhs_of_diff,
    };
    diff_fn(&old, &new)
}


/*
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
*/

/* NOTE:
 *  Should `get_cache` and `get_url` be public? Or wrapped?
 *  Job::fire() needs to have possession of both the diff and the `new`,
 *   so a wrapper would have to return a tuple of results or something
 */

//pub fn get_cache(filename: &str, buffer: &mut String) -> Result<usize,String> {
pub fn get_cache(filename: &PathBuf, buffer: &mut String) -> Result<usize,String> {
    //open cached version of a page. Return the html or an error message
    //let filename = url_to_str(url);
    let mut file = match File::open(filename) {
        Err(e) => return Err(format!("File open error: {}",
                                     e.description()
                                     .to_string())),
        Ok(f)  => f,
    };
    match file.read_to_string(buffer) {
        Err(e) => Err(format!("File open error: {}",
                              e.description() 
                              .to_string())),
        Ok(_)  => Ok(buffer.len()),
    }
}

pub fn get_url(url: &hyper::Url, buffer: &mut String) -> Result<usize,String> {
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
    let res = client.get(url.clone())
                    .headers(headers)
                    .send();
    match res {
        Err(e) => Err(format!("Request error: {}", 
                              e.description().to_string())),
        Ok(mut r)  => {
            //let mut text = String::new();
            match r.read_to_string(buffer) {
                Err(e) => Err(format!("Read error: {}", 
                                      e.description().to_string())),
                Ok(_)  => Ok(buffer.len()),
            }
        }
    }
}
