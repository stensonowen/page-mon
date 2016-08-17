/*	Periodically crawl web pages and alert the user of changes 
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
 * 	More information in the enclosed `LICENSE' file
 */

/* mod.rs
 * Organize calls to scrape, save, compare, and send data
 */


pub mod scrape;
pub mod contact;
//use super::parse::job;
use super::job;
use super::parse::{Var, Vars};
use self::scrape::url_to_str;
use std::collections::HashMap;
use std::path::Path;
use std::fs::OpenOptions;
use std::error::Error;
use std::io::Write;

extern crate hyper;
extern crate chrono;
use self::chrono::{Local, DateTime};

const PUSHJET_PRIORITY: u8 = 3;   //1-5


pub fn act(delta: &str, url: hyper::Url, method: job::Contact, 
           vars: &Vars, now: &DateTime<Local>) -> Result<(),String> {
    //contact the user via `method` (email/pushjet)
    let url_domain = match url.domain() {
        Some(d) => d,
        None => url.as_str(),
    };
    let subject = format!("Update from `{}` at `{}`", url_domain, now.to_string()); 
    if method == job::Contact::Text {
        let secret = match vars.get(&Var::PushjetSecret) {
            Some(s) => s,
            None    => return Err("No PushjetSecret value defined".to_string()),
        };
        //should definitely contain Url, because it was added in 
        // parse/mod.rs:insert_variable_default_values()
        //TODO: verify supplied url is valid & parsable
        let pushjet_url = vars.get(&Var::PushjetUrl).unwrap();
        let pushjet_url = hyper::Url::parse(pushjet_url).unwrap();
        let page_url = url.as_str();
        let res = contact::pushjet(pushjet_url, secret, delta, 
                                   &subject, PUSHJET_PRIORITY, page_url);
        if let Err(e) = res {
            return Err(format!("Failed to contact via pushjet: {}", e))
        }
    } else if method == job::Contact::Email {
        let secret = match vars.get(&Var::EmailSecret) {
            Some(s) => s,
            None    => return Err("No EmailSecret value defined".to_string())
        };
        let domain = match vars.get(&Var::EmailDomain) {
            Some(d) => d,
            None    => return Err("No EmailDomain value defined".to_string())
        };
        let to = match vars.get(&Var::EmailRecip) {
            Some(t) => t,
            None    => return Err("No EmailRecipient value defined".to_string()),
        };
        let res = contact::post_email(secret.to_string(), domain, to, 
                                      &subject, delta);
        if let Err(e) = res {
            return Err(format!("Failed to contact via email: {}", e))
        }
    } 
    //log data independent of method of contact
    let dir = vars.get(&Var::Dir).unwrap();
    let log_type = match method {
        job::Contact::LogAll => LogType::Append,
        _                    => LogType::Create,
    };
    let res = log(&url, dir, log_type, delta);
    if let Err(e) = res {
        Err(format!("Failed to log data: {}", e))
    } else {
        Ok(())
    }
}

enum LogType {
    Create,
    Append,
}

fn log(url: &hyper::Url, dir: &str, mut log_type: LogType, content: &str) -> Result<(),String> {
    //replace or append
    //open file
    let filename = url_to_str(url);
    let filepath = Path::new(dir).join(filename);
    //if file is absent, create it
    //i.e., act as if we're in Create mode even if we aren't
    if filepath.is_file() == false {
        log_type = LogType::Create;
    }
    let file = match log_type {
        LogType::Create => OpenOptions::new().write( true).open(filepath),
        LogType::Append => OpenOptions::new().append(true).open(filepath),
    };
    let mut file = match file {
        Ok(f)  => f,
        Err(e) => return Err(format!("Error opening cache: {}", e.description())),
    };
    match file.write(content.as_bytes()) {
        Ok(_)  => Ok(()),
        Err(e) => Err(format!("Error writing to cache: {}", e.description())),
    }
}
