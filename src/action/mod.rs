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
//use super::parse::{Var, Vars};

use super::parse;
use super::parse::ast;
//use self::scrape::url_to_str;
use self::contact::{post_email,pushjet};

use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::Write;
//use std::fs::File;
use std::error::Error;

use std::hash::{Hash, Hasher, SipHasher};

extern crate hyper;
extern crate chrono;
use self::chrono::{DateTime, Local};

const PUSHJET_PRIORITY: u8 = 3;



/* Define the 'Action' type, which stores contact info and metadata
 */

pub enum Action {
    //New form of `ast::Contact` that includes the values it depends on
    //Makes it harder for a un-definition to slip through the cracks
    //TODO: use string slices instead?? it would reduce repetition
    //TODO: make `Dir` a mandatory element of every option?
    Email {
        domain: String,
        secret: String,
        recip:  String,
    },
    Pushjet {
        secret: String,
        url:    hyper::Url,
    },
    LogLast,
    LogAll,
}

fn extract<'a>(target: ast::VarType, 
               vars: &'a parse::Vars) -> Result<&'a str,String>{
    //shorthand for extracting a var from Vars with a string error message
    //use `try!(extract(..))` and it'll throw helpful error messages
    //helper for Action::extrapolate
    match vars.get(&target) {
        Some(v) => Ok(v),
        None => Err(format!("Necessary variable not defined: {:?}", target)),
    }
}

impl Action {
    pub fn extrapolate(base: ast::Contact, 
                       vars: &parse::Vars) -> Result<Self,String>{
        //converts the basic information from ast::Contact into a fuller version
        // by extracting the relevant variables from `vars` and storing them
        match base {
            ast::Contact::LogAll  => Ok(Action::LogAll),
            ast::Contact::LogLast => Ok(Action::LogLast),
            ast::Contact::Text    => {
                let secret = try!(extract(ast::VarType::PjSecret, vars));
                let urlstr = try!(extract(ast::VarType::PjUrl, vars));
                let url = match hyper::Url::parse(urlstr) {
                    Ok(u)  => u,
                    Err(e) => return Err(format!("Failed to parse PjUrl into url: 
                                                 {:?}", e)),
                };
                Ok(Action::Pushjet { secret: secret.to_string(), url: url})
            },
            ast::Contact::Email    => {
                let domain = try!(extract(ast::VarType::EmailDomain, vars));
                let secret = try!(extract(ast::VarType::EmailSecret, vars));
                let recip  = try!(extract(ast::VarType::EmailRecip,  vars));
                //Ok(Contact::Email { domain: domain, secret: secret, recip: recip})
                Ok(Action::Email { domain: domain.to_string(), 
                                    secret: secret.to_string(), 
                                    recip:   recip.to_string()})
            },
        }
    }
    pub fn contact(&self, url: &hyper::Url, delta: &str, 
                   timestamp: &DateTime<Local>) -> Result<(),String> {
        //NOTE: `delta` is ONLY what you want to communicate to the user
        //  (probably just the changes)
        //communicate changes using whatever method
        let url_short = url.domain().unwrap_or(url.as_str());
        let subject = format!("Update from `{}` at `{}`", url_short, timestamp);
        let res = match self {
            &Action::Pushjet { secret: ref s, url: ref pj_u } => 
                pushjet(pj_u.clone(), &s, delta, &subject, 
                        PUSHJET_PRIORITY, url.as_str()),
            &Action::Email { domain: ref d, secret: ref s, recip: ref r } => 
                post_email(s, &d, &r, &subject, delta),
            _ => Ok(String::new())  //TODO: don't do this?
        };
        match res {
            Ok(_)  => Ok(()),
            Err(e) => Err(format!("Failed to convey message: {}", e))
        }
        //should `log` be called here? or right after this function?
    }
    //pub fn log(&self, dir: &str, filename: &str, text: &str, 
    pub fn log(&self, path: &PathBuf, text: &str, 
               timestamp: &DateTime<Local>) -> Result<(),String> {
        //NOTE: `text` is the FULL page html, NOT just the changes
        //store `text` (i.e. a full page) to that page's log file
        //this will be called every time delta.len() > 0 (on every change)
        //somewhat different behavior based on what `self` is:
        //  LogAll:
        //      store full text at every iteration, regardless of changes
        //      append new value of `text` to the file without erasing
        //      e.g. monitor twitch metadata 
        //          e.g. https://api.twitch.tv/kraken/channels/truktruk
        //  Anything Else:
        //      only store if there was some change
        //      rewrite entire file
        //let path = Path::new(dir).join(filename);
        let file = match *self {
            Action::LogAll => OpenOptions::new().append(true).create(true).open(path),
            _               => OpenOptions::new().write(true).create(true).open(path)
        };
        let mut file = match file {
            Ok(f)  => f,
            Err(e) => return Err(format!("Failed to open a file for writing: {}",
                                         e.description()))
        };
        match file.write_fmt(format_args!("\n\t{}\n{}\n", timestamp, text)) {
            Ok(_)  => Ok(()),
            Err(e) => Err(format!("Failed to write to file: {}", e.description()))
        }
    }
    /*
    pub fn fire(&self, dir: &str, timestamp: &DateTime<Local>) -> Result<(),String> {
        TODO: move to job or something
        // 1. get page contents
        // 2. open cache contents
        // 3. diff them
        // 4. contact the user
        // 5. update the cache
        Ok(())

    }
    */
}



pub fn url_to_file(url: &hyper::Url) -> String {
    //try to make descriptive name out of url to use for file cache
    //can't just use the domain, because there could be collisions
    //if we can't, just use the url itself (without the forward slashes)
    //"/" and \0 are the only invalid characters in a filename
    //  in linux, at least. with windows who knows
    //TODO: Hash just `url` or whole `job`? probably just url
    // otherwise with distinct jobs with the same url, user might be 
    //  alerted twice after one change
    let suffix = ".cache";

    let mut prefix = {
        if let (Some(s), Some(d)) = (url.path_segments(), url.domain()) {
            let mut sum = d.to_owned();
            sum.push('~');
            s.into_iter().fold(sum, |mut acc, part| {acc.push_str(part); acc})
        } else {
            url.as_str().replace("/", "_")
        }
    };

    let mut hasher = SipHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();
    let uid = format!("{:09}", hash % 1_000_000_000);

    prefix.push('~');
    prefix.push_str(&uid);
    prefix.push_str(suffix);
    prefix
}


/*
enum LogType {
    Append,
    Create
}

pub fn act(delta: &str, url: hyper::Url, method: ast::Contact, 
           vars: &Vars, now: &DateTime<Local>) -> Result<(),String> {
    //contact the user via `method` (email/pushjet)
    let url_domain = match url.domain() {
        Some(d) => d,
        None => url.as_str(),
    };
    let subject = format!("Update from `{}` at `{}`", url_domain, now.to_string()); 
    match method {
        ast::Contact::Text => {
            let secret = match vars.get(&VarType::PjSecret) {
                Some(s) => s,
                None    => return Err("No PushjetSecret value defined".to_string()),
            };
            //should definitely contain Url, because it was added in 
            // parse/mod.rs:insert_variable_default_values()
            //TODO: verify supplied url is valid & parsable
            let pushjet_url = vars.get(&VarType::PjUrl).unwrap();
            let pushjet_url = hyper::Url::parse(pushjet_url).unwrap();
            let page_url = url.as_str();
            let res = contact::pushjet(pushjet_url, secret, delta, 
                                       &subject, PUSHJET_PRIORITY, page_url);
            if let Err(e) = res {
                return Err(format!("Failed to contact via pushjet: {}", e))
            }
        },
        ast::Contact::Email => {
            let secret = match vars.get(&VarType::EmailSecret) {
                Some(s) => s,
                None    => return Err("No EmailSecret value defined".to_string())
            };
            let domain = match vars.get(&VarType::EmailDomain) {
                Some(d) => d,
                None    => return Err("No EmailDomain value defined".to_string())
            };
            let to = match vars.get(&VarType::EmailRecip) {
                Some(t) => t,
                None    => return Err("No EmailRecipient value defined".to_string()),
            };
            //let res = contact::post_email(secret.to_string(), domain, to, 
            let res = contact::post_email(secret, domain, to, 
                                          &subject, delta);
            if let Err(e) = res {
                return Err(format!("Failed to contact via email: {}", e))
            }
        },

        _ => (),

    }

    Ok(())
}

fn log(url: &hyper::Url, log_type: LogType) -> Result<(),String> {
    //replace or append
    Ok(())

}*/
