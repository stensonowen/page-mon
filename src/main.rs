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

//use std::time::Duration;
use std::{thread,time};
//use std::thread::sleep;
//extern crate chrono;
//use chrono::{DateTime,Local,Duration,Timelike};

fn main() {
    //TODO: replace this with a constant
    let input_file = Path::new("/home/owen/page-mon/config");

    let (cmds, vars) = parse::parse(input_file).unwrap();
    let jobs: Vec<job::Job> = cmds.into_iter().map(|c| job::Job::from(c, &vars).unwrap()).collect();
    //panic! if a job is invalid
    
    //let mut dt = Local::now();
    //let mut dur = time_to_next_minute(&dt).to_std();
    //thread::sleep(dur);
    //wait for the minute to start
    let mut now = time::Instant::now();
    let mut dur = time_to_next_minute(&now);
    thread::sleep(dur);


    loop {
        now = time::Instant::now();
        //iterate through the jobs, executing those for which it is time
        for j in &jobs {

        }
        dur = time_to_next_minute(&now);
        thread::sleep(dur);
    }
}

//fn time_to_next_minute(last_run: &DateTime<Local>) -> Duration {
fn time_to_next_minute(last_run: &time::Instant) -> time::Duration {
    //compute the amount of time program should wait before checking again
    //if cycling through everything took more than 1 minute, then it'll miss 
    // the next minute (TODO?)
    //It's okay to wait a little too long, but do not wait too short
    
    // 20 -> 40
    // 61 -> 59
    let seconds_since = last_run.elapsed().as_secs() as i64;
    //is there a more succinct way to do this that is actually readable?
    let mut seconds_to_next = (60 - seconds_since) % 60;
    if seconds_to_next <= 0 {
        seconds_to_next += 60;
    }
    time::Duration::from_secs(seconds_to_next as u64)
}
