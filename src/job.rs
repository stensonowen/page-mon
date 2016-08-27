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

use parse;
use event::calendar;
use action;

extern crate hyper;
extern crate chrono;
use self::chrono::{DateTime,Local};

use std::path::Path;


pub struct Job {
    time:   calendar::Calendar,
    pub url:    hyper::Url,
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
    pub fn fire_if_match(&self, dir: &str, timestamp: &DateTime<Local>) -> Result<(),String> {
        //wraps Calendar::fire_now
        if self.time.fire_now(timestamp) {
            self.fire(dir, timestamp)
        } else {
            Ok(())
        }
    }
    pub fn matches_time(&self, timestamp: &DateTime<Local>) -> bool {
        self.time.fire_now(timestamp)
    }

    pub fn fire(&self, dir: &str, timestamp: &DateTime<Local>) -> Result<(),String> {
        //Do this in a different order so at most one Error is returned
        // 1. get page contents
        // 2. open cache contents
        // 3. diff them
        // 4. contact the user
        // 5. update the cache
        //NOTE: some functions return Ok(Data), which might occasionally be helpful
        // for logging purposes but which we ignore here
        let filename = action::url_to_file(&self.url);
        let path = Path::new(dir).join(filename);
        let mut cache = String::new();
        let mut html  = String::new();

        //if `get_cache` fails, we assume there is no cache and this is the first run
        //TODO: maybe rework this so an error is handles?
        //      or so get_cache doesn't return a result?
        action::scrape::get_cache(&path, &mut cache);

        if let Err(e) = action::scrape::get_url(&self.url, &mut html) {
            //nothing to update if there's no new info
            return Err(format!("Failed to download page: {}", e));
        }

        //update the cache
        if let Err(e) = self.via.log(&path, &html, timestamp) {
            //if the cache cannot be updated there's something pretty wrong
            //there's also a chance our info is screwed up
            //not returning here though would be problematic, as we want to
            // complain that the cache couldn't be updated, but there could be
            // other errors below; we'd have to return a Vec<String>
            return Err(format!("Failed to update cache: {}", e));
        }

        //if everything above has worked, 
        let diff = action::scrape::diff(&cache, &html);
        if diff.is_empty() {
            return Ok(());
        }
        //send the message and return the result
        match self.via.contact(&self.url, &diff, timestamp) {
            Ok(_)  => Ok(()),
            Err(e) => Err(format!("Failed to contact user: {}", e))
        }
    }
}

