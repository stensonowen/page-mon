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

use std::ops;
use std::collections::BTreeSet;

extern crate chrono;
use self::chrono::{TimeZone, Local, Datelike};

//pub mod value_itr;
use event::value_itr::*;
use ast::{Time, Value};


const MINUTE_RANGE: ops::Range<u8> = 0..60;
const HOUR_RANGE:   ops::Range<u8> = 0..24;
const DATE_RANGE:   ops::Range<u8> = 1..32;
const MONTH_RANGE:  ops::Range<u8> = 1..13;
const WEEKDAY_RANGE:ops::Range<u8> = 0.. 8;


pub type ValidSet = BTreeSet<u8>;

#[derive(Debug)]
pub struct Calendar {
    mn:     ValidSet,
    hr:     ValidSet,
    dt:     ValidSet,
    mon:    ValidSet,
    dow:    ValidSet,
}


const MONTH_LENS: [u8;12] = [31,0,31,30,31,30,31,31,30,31,30,31];
const WEEK_LEN: u8 = 7;
impl Calendar {
    pub fn from_time(time: &mut Time) -> Calendar {
        Calendar {
            mn:     collect_vals(&mut time.minute,  MINUTE_RANGE),
            hr:     collect_vals(&mut time.hour,    HOUR_RANGE),
            dt:     collect_vals(&mut time.date,    DATE_RANGE),
            mon:    collect_vals(&mut time.month,   MONTH_RANGE),
            dow:    collect_vals(&mut time.weekday, WEEKDAY_RANGE),
        }
    }

    fn days_in_feb(year: i32) -> u8 {
        if      year %   4 != 0     { 28 } 
        else if year % 100 != 0     { 29 } 
        else if year % 400 != 0     { 28 } 
        else                        { 29 }
    }
    fn days_in_month(year: i32, month: u8) -> u8 {
        //input: starting from 1, e.g. December is 12
        if month == 2 {
            Calendar::days_in_feb(year)
        } else {
            let index = (month - 1) as usize;
            MONTH_LENS[index]
        }
    }

    pub fn get_month(&self, year: i32, month: u8) -> ValidSet {
        //TODO: unit tests
        let mut month_set = self.mon.clone();
        let ref dow_set = self.dow;
        //remove days not in this month:
        let days_in_month = Calendar::days_in_month(year, month) + 1;
        for i in days_in_month .. 32u8 {
            month_set.remove(&i);
        }
        //get the first of the month's day of the week. 0 = sunday
        let first_day = Local.ymd(year, month as u32, 1).weekday().num_days_from_sunday();
        let offset = (WEEK_LEN - first_day as u8) % WEEK_LEN;
        for weekday in dow_set.iter() {
            let mut mult = 0;
            loop {
                let guess = weekday + mult * WEEK_LEN;
                if guess >= days_in_month {
                    break;
                } else {
                    month_set.insert(guess);
                }
                mult += 1;
            }
        }
        month_set
    }
}


pub fn collect_vals(vals: &mut Vec<Value>, range: ops::Range<u8>) -> ValidSet {
    let mut cal = ValidSet::new();
    for mut value in vals.into_iter() {
        let val_itr = ValueItr::new(&mut value, &range);
        for possibility in val_itr.into_iter() {
            cal.insert(possibility);
        }
    }
    cal
}

