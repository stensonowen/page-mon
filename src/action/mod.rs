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
use super::parse::Vars;
use super::parse::ast::VarType;

use super::parse::ast;
use self::scrape::url_to_str;
use std::collections::HashMap;

extern crate hyper;
extern crate chrono;
use self::chrono::{DateTime, Local};

const PUSHJET_PRIORITY: u8 = 3;

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
            let res = contact::post_email(secret.to_string(), domain, to, 
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

}
