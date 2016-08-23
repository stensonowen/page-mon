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


use parse::ast::Contact;
use event::calendar::ValidSet;

extern crate hyper;

struct Time {
    //store time as a collection of valid options
    //more efficient to store in HashSet (or BTreeSet or whatever)
    // than iterating through an arbitrarily complex definition every 
    // iteration (which might be every minute).
    //Better to use a little bit more memory that at least has an upper bound
    // rather than committing an arbitrary amount of cpu power every minute
    minute: ValidSet,
    hour:   ValidSet,
    date:   ValidSet,
    month:  ValidSet,
    weekday:ValidSet,
}

struct Job {
    time:   Time,
    url:    hyper::Url,
    via:    Contact,
}

