/*	Periodically crawl web pages and alert the user of changes
 *
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

/*
 * Contains info from `ast.rs` and defines the `job` structure
 * `ast.rs` defines the structure that was parsed,
 * `job.rs` defines the structure that is stored
 */


use parse::ast;//::Contact;
use parse;
//use event::calendar::ValidSet;
use event::calendar;//::Calendar;
//use parse::Vars;

extern crate hyper;

enum Contact {
    //New form of `ast::Contact` that includes the values it depends on
    //Makes it harder for a un-definition to slip through the cracks
    //TODO: use string slices instead?? it would reduce repetition
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

fn extract<'a>(target: ast::VarType, vars: &'a parse::Vars) -> Result<&'a str,String> {
    //shorthand for extracting a var from Vars with a string error message
    match vars.get(&target) {
        Some(v) => Ok(v),
        None => Err(format!("Necessary variable not defined: {:?}", target)),
    }
}


impl Contact {
    pub fn extrapolate(base: ast::Contact, vars: &parse::Vars) -> Result<Self,String> {
        //converts the basic information from ast::Contact into a fuller version
        // by extracting the relevant variables from `vars`
        match base {
            ast::Contact::LogAll  => Ok(Contact::LogAll),
            ast::Contact::LogLast => Ok(Contact::LogLast),
            ast::Contact::Text    => {
                let secret = try!(extract(ast::VarType::PjSecret, vars));
                let urlstr = try!(extract(ast::VarType::PjUrl, vars));
                let url = match hyper::Url::parse(urlstr) {
                    Ok(u)  => u,
                    Err(e) => return Err(format!("Failed to parse PjUrl into url: {:?}", e)),
                };
                Ok(Contact::Pushjet { secret: secret.to_string(), url: url})
            },
            ast::Contact::Email    => {
                let domain = try!(extract(ast::VarType::EmailDomain, vars));
                let secret = try!(extract(ast::VarType::EmailSecret, vars));
                let recip  = try!(extract(ast::VarType::EmailRecip,  vars));
                //Ok(Contact::Email { domain: domain, secret: secret, recip: recip})
                Ok(Contact::Email { domain: domain.to_string(), 
                                    secret: secret.to_string(), 
                                    recip:   recip.to_string()})
            },
        }
    }
}

pub struct Job {
    time:   calendar::Calendar,
    url:    hyper::Url,
    via:    ast::Contact,
}
//TODO: should `job` store just `Contact` type and assume all data 
// are supplied in variable 


impl Job {
    pub fn from(cmd: &ast::Command, vars: &parse::Vars) -> Result<Self,String> {
        Err(String::new())
    }

}

