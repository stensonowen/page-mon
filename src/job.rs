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


//use parse::ast;//::Contact;
use parse;
//use event::calendar::ValidSet;
use event::calendar;//::Calendar;
//use parse::Vars;
//use action::Action;
use action;

extern crate hyper;


pub struct Job {
    time:   calendar::Calendar,
    url:    hyper::Url,
    //via:    Contact,    //the one above, NOT the one in ast.rs
    via:    action::Action,
}
//TODO: should `job` store just `Contact` type and assume all data 
// are supplied in variable? 


impl Job {
    pub fn from(cmd: parse::ast::Command, vars: &parse::Vars) -> Result<Self,String> {
        let mut time = cmd.time;
        let cal = calendar::Calendar::from_time(&mut time);
        let url = match hyper::Url::parse(&cmd.act.url) {
            Ok(u)  => u,
            Err(e) => return Err(format!("Failed to parse job url ({}) into url: {:?}"
                                         , cmd.act.url, e))
        };
        let contact = try!(action::Action::extrapolate(cmd.act.contact, vars));
        Ok(Job { time: cal, url: url, via: contact })
    }
}

