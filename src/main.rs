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

/* main.rs is a mess
 *  mostly used to test stuff before it goes in lib.rs as a unit test.
 *  pardon the sloppiness
 * NOTE: future default location: /var/cache
 */

mod parse;
mod event;
mod job;
mod action;

use std::path::Path;
use std::fs::DirBuilder;

use std::{thread,time};
extern crate chrono;
use chrono::{DateTime,Local,Duration,Timelike};

const LOG_FILE: &'static str = "/home/owen/page-mon/log";
//const CFG_FILE: &'static str = "/home/owen/page-mon/config_";
const CFG_FILE: &'static str = "/home/owen/shared/code/rust/page-mon/config";

fn main() {
    //TODO: start threads for each tasks
    //TODO: replace vec with map to futures?
    
    let config = Path::new(CFG_FILE);

    //parse input; panic if parsing fails
    let (cmds, vars) = match parse::parse(config) {
        Ok((c,v)) => (c,v),
        Err(e)    => panic!(e.join("\n")),
    };
    //panic! if a job is invalid
    let jobs: Vec<job::Job> = cmds.into_iter()
                                  .map(|c| job::Job::from(c, &vars).unwrap())
                                  .collect();
    
    //create cache directory if it's not there
    let cache_path = parse::get_dir(&vars).expect("No `DIR` variable set in config");
    if Path::new(cache_path).is_dir() == false {
        DirBuilder::new().recursive(true).create(cache_path).expect("Failed to create cache dir");
    }
    println!("Cache path: {:?}", cache_path);
    
    let mut now = Local::now();
    thread::sleep(time_to_next_minute(&now));

    loop {
        //iterate through the jobs, executing those for which it is time
        for j in &jobs {
            println!("Starting job {}", j.url);
            if let Err(e) = j.fire_if_match(cache_path, &now) {
                println!("Error in job {}: `{}`", j.url, e);
            }
        }
        println!("\t{}", now);
        now = Local::now();
        thread::sleep(time_to_next_minute(&now));
    }
}

fn time_to_next_minute(last_run: &DateTime<Local>) -> time::Duration {
    //compute the amount of time program should wait before checking again
    //if cycling through everything took more than 1 minute, then it'll miss 
    // the next minute (TODO?)
    //It's okay to wait a little too long, but do not wait too short
    
    let sec = last_run.second() as i64; // 0 <= sec <= 60
    //TODO: will there ever be an instance when unwrap fails?
    //DateTime::second() should never exceed 60, right?
    Duration::seconds(60i64 - sec).to_std().unwrap()
}
