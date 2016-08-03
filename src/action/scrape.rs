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

//use std::env;
//use std::path::PathBuf;
//use std::fs::File;
use std::error::Error;
use std::io::Read;
use self::hyper::header::*;

const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";

pub fn get_url(url: hyper::Url) -> Result<String,String> {
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
                              e.description()
                              .to_string())),
        Ok(mut r)  => {
            let mut text = String::new();
            match r.read_to_string(&mut text) {
                Err(e) => Err(format!("Read error: {}", 
                                      e.description()
                                      .to_string())),
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
