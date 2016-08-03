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

//use std::env;
//use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};
use self::hyper::header::*;

//use a descriptive user agent? or a generic one?
const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";

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
        Err(e) => {
            //no `old` file to compare to; (try to) create one and throw an Err
            println!("About to create file: '{}'", filename);
            let mut file = match File::create(filename) {
                Ok(f)  => f,
                Err(r) => return Err(format!("Failed to open cache ({}), and failed to create cache: {}",
                                             e, r.description().to_string())),
            };
            return match file.write_all(new.as_bytes()) {
                Ok(_)  => Err(format!("Cache made because none was found ({})",
                                      e)),
                Err(r) => Err(format!("Failed to open cache ({}), and failed to write cache: {}",
                                      e, r.description().to_string())),
            }
        }
    };
    let delta = diff::lines(&old, &new);
    let mut recent = String::new();
    for diff in delta {
        if let diff::Result::Right(change) = diff {
            recent.push('\n');
            recent.push_str(change);
        }
    }
    if recent.is_empty() {
        Err("No change".to_string())
    } else {
        Ok(recent)
    }
}

pub fn url_to_str(url: &hyper::Url) -> String {
    //try to make descriptive name out of url to use for file cache
    //can't just use the domain, because there could be collisions
    //if we can't, just use the url itself (without the forward slashes)
    let split = url.path_segments();
    if let Some(s) = split {
        let parts: Vec<&str> = s.into_iter().collect();
        let sum = parts.join("_");
        if !sum.is_empty() {
            return sum
        }
    } 
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

/*
pub fn contact(pushjet_url: hyper::Url, secret: &str, message: &str, 
               title: &str, level: u8, link: &str) -> Result<String,String> {
    //on failure: error description wrapped in Err()
    //on success: return response (for logging?) in Ok()
    //  could just return ()? does anyone care? Will there be logging?
    let url = pushjet_url.join("message").unwrap();
    let client = hyper::Client::new();

    //serialize data
    let payload: String = Serializer::new(String::new())
                    .append_pair("secret",  secret)
                    .append_pair("message", message)
                    .append_pair("title",   title)
                    .append_pair("level",   level.to_string().as_str())
                    .append_pair("link",    link)
                    .finish();

    //set up and make request
    let res = client.post(url)
                    .header(ContentType::form_url_encoded())
                    .body(payload.as_bytes())
                    .send();

    //return status code or error message
    match res {
        Err(e) => Err(e.description().to_string()),
        Ok(t)  => Ok(t
                     .status
                     .canonical_reason()
                     .unwrap_or("Success: reason unknown")
                     .to_string()),
    }
}*/
