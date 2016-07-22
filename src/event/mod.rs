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

extern crate hyper;
use std::{time, ops};
use ast::{Line, Special};
mod crontime;

pub struct Field {
    //e.g. Minute, Month
    range:  ops::Range<u8>,
    valid:  Vec<Special>,
}


pub fn sanity_check(value: &Line) -> bool {
    //see https://en.wikipedia.org/wiki/Cron#Format
    let minute  = Field { range: 0..60, valid: vec![Special::Asterisk, Special::Slash(0)] };
    let hour    = Field { range: 0..24, valid: vec![Special::Asterisk, Special::Slash(0)] };
    let date    = Field { range: 1..32, valid: vec![Special::Asterisk, Special::Slash(0),
                            Special::Question, Special::W, Special::L] };
    let month   = Field { range: 1..13, valid: vec![Special::Asterisk, Special::Slash(0)] };
    let weekday = Field { range: 0.. 7, valid: vec![Special::Asterisk, Special::Slash(0),
                            Special::Question, Special::L, Special::Hash(0)] }; //?

    false 
}

#[allow(dead_code)]
pub struct Event {
    url:    hyper::Url,
    period: time::Duration,

}


pub fn foo() {
    let ct = crontime::CronTime::from_string("a b c d e");
    println!("{:?}", ct);
}
