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
use git::Repo;

extern crate hyper;
extern crate chrono;
use self::chrono::{DateTime,Local};

use std::path::Path;
use std::hash::{Hash, Hasher, SipHasher};
use std::error::Error;

#[derive(Hash)]
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

    //pub fn fire(&self, dir: &str, timestamp: &DateTime<Local>) -> Result<(),String> {
    pub fn fire(&self, repo_path: &str, timestamp: &DateTime<Local>) -> Result<(),String> {
        //Do this in a different order so at most one Error is returned
        //0. open the repo (and fetch oid)
        //1. save page contents to a file
        //2. add the file to the repo
        //3. commit the file to the repo (and fetch new oid)
        //4. diff the file with the saved version
        //NOTE: some functions return Ok(Data), which might occasionally be helpful
        // for logging purposes but which we ignore here
        //let filename = action::url_to_file(&self.url);
        let filename = self.filename();
        //let path = Path::new(repo_path).join(filename);
        //open repo
        let repo = match Repo::open(repo_path) {
            Ok(r)  => r,
            Err(e) => return Err(format!("Failed to open git repo: {}", e.description()))
        };
        //and get its oid
        let oid_old = match repo.get_oid() {
            Ok(o)  => o,
            Err(e) => return Err(format!("Failed to get oid: {}", e.description()))
        };

        if let Err(e) = action::scrape::fetch_page(&self.url, &filename) {
            //nothing to update if there's no new info
            return Err(format!("Failed to get/save page: {}", e))
        }

        if let Err(e) = repo.add_file(&filename) {
            return Err(format!("Failed to add page to repo: {}", e.description()));
        }
        
        let commit_msg = format!("Updated '{}' at {}", filename, timestamp);
        let oid_new = match repo.commit(&commit_msg, oid_old) {
            Ok(o)  => o,
            Err(e) => return Err(format!("Failed to commit to repo: {}", e.description()))
        };

        let diff = match repo.diff(oid_old, oid_new) {
            Ok(d)  => d,
            Err(e) => return Err(format!("Failed to diff files: {}", e.description()))
        };

        match self.via.contact(&self.url, &diff, timestamp) {
            Ok(_)  => Ok(()),
            Err(e) => Err(format!("Failed to contact user: {}", e))
        }
        //if everything above has worked, 
        //let diff = action::scrape::diff(&cache, &html);
        //if diff.is_empty() {
        //    return Ok(());
        //}
        //send the message and return the result
        //Ok(())
        //match self.via.contact(&self.url, &diff, timestamp) {
        //    Ok(_)  => Ok(()),
        //    Err(e) => Err(format!("Failed to contact user: {}", e))
        //}
    }

    pub fn filename(&self) -> String {
        //hash self to get unique filename
        let suffix = ".cache";

        let mut prefix = {
            if let (Some(s), Some(d)) = (self.url.path_segments(), self.url.domain()) {
                let mut sum = d.to_owned();
                sum.push('~');
                s.into_iter().fold(sum, |mut acc, part| {acc.push_str(part); acc})
            } else {
                self.url.as_str().replace("/", "_")
            }
        };

        let mut hasher = SipHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();
        let uid = format!("{:09}", hash % 1_000_000_000);

        prefix.push('~');
        prefix.push_str(&uid);
        prefix.push_str(suffix);
        prefix

    }
}


