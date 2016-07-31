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

/* BS:
 *  The biggest thing that a Command object needs to do is, given its 
 *  internal data and the current time, return the next time it should run.
 *  There seem to be two ways of approaching this.
 *  1. each vector of Values is consolidated into a hash set of valid times:
 *      this would be slightly costly in terms of initial construction and 
 *      memory usage, but fetching the 'next' valid value would be very fast.
 *      5 hash tables large enough to store `*****` is 263 bytes (134 u8s)
 *  2. fetch the 'next' valid value for each Value in the vector. that would
 *      require implementing 'next' for Asterisk, Range, Constant, and Skip.
 *      This would require less memory but maybe a little more cpu to find 
 *      'next' an arbitrary (though effectively like 1 or 2) number of times.
 *      It might be in our best interest to consolidate values also (e.g. 
 *      `2,1-3` and `1-3,3-4` are redundant), but I'm not sure how I'd do that.
 *  (1) would probably be simpler, cleaner, and less interesting. 
 */

//extern crate hyper;

//extern crate chrono;
//use self::chrono::{Local, datetime, TimeZone, Duration, Weekday};
//use event::chrono::{Timelike, Datelike};

//use std::{ops, cmp, u8};
use std::ops;
use std::collections::BTreeSet;

pub mod value_item;
use event::value_item::*;

use ast::*;


pub type Calendar = BTreeSet<u8>;

pub fn cal_from_vals(vals: &mut Vec<Value>, range: ops::Range<u8>) -> Calendar {
    //let max_size = (range.end - range.start) as usize;
    let mut cal = Calendar::new();
    for mut value in vals.into_iter() {
        let val_item = ValueItem::new(&mut value, &range);
        for possibility in val_item.into_iter() {
            cal.insert(possibility);
        }
    }
    cal
}




/*
pub trait HasNext {
    //fn next(&self, current: u8, range: &ops::Range<u8>) -> u8;
    
    fn verify(&self, valid: &ops::Range<u8>) -> bool;
    //returns whether this value is valid. 
    //Examples of invalid values include constants outside the 
    // the valid range (e.g. 100) and ranges like 5..2.
}
*/


/*
impl HasNext for ContVal {
    fn next(&self, current: u8, range: &ops::Range<u8>) -> u8 {
        //safe to assume current \in range
        match *self {
            ContVal::Asterisk       => {
                if current < range.end {
                    current  
                } else {
                    range.start
                }
            }
            ContVal::Range(min,max) => {
                //not safe to assume that min <= current <= max
                //TODO: increment max by one: cron ranges are inclusive
                if current >= min && current < max {
                    current
                } else {
                    min
                }
            }
        }
    }

    fn verify(&self, valid: &ops::Range<u8>) -> bool {
        match *self {
            ContVal::Asterisk => true,
            ContVal::Range(min,max) => 
                min >= valid.start
                && max < valid.end
                && min < max
        }
    }

}

impl HasNext for Value {
    fn next(&self, current: u8, range: &ops::Range<u8>) -> u8 {
        match *self {
            Value::CV(ref cv)   => cv.next(current, &range),
            Value::Constant(c)  => c,
            Value::Skip(ref cv, mult) => {
                let (min, max) = match *cv {
                    ContVal::Asterisk => (range.start, range.end),
                    ContVal::Range(min,max) => (min, max),
                };
                let start = cmp::max(current, min);
                if start == 0 {
                    return 0;
                }
                let guess = ((start-1)/mult+1)*mult;
                if guess < max {
                    guess
                } else {
                    //the smallest multiple of `skip` that
                    //exceeds `current` also exceeds `end`,
                    //so it overflowed.
                    //TODO: verify that there is at least one valid 
                    //answer. e.g. nothing like `20-25/9`
                    ((min-1)/mult+1)*mult
                }
            }
        }
    }
    fn verify(&self, valid: &ops::Range<u8>) -> bool {
        match *self {
            Value::CV(ref cv)   => cv.verify(valid),
            Value::Constant(c)  => valid.start<=c && c<valid.end,
            Value::Skip(ref cv, mult) => 
                mult != 0 && mult < valid.end && cv.verify(valid), 
        }
    }
}*/


/*
//Valid ranges for each field
//Lower bound included, upper bound excluded
const MINUTE_RANGE: ops::Range<u8> = 0..60;
const HOUR_RANGE:   ops::Range<u8> = 0..24;
const DATE_RANGE:   ops::Range<u8> = 1..32;
const MONTH_RANGE:  ops::Range<u8> = 1..13;
//const WEEKDAY_RANGE:ops::Range<u8> = 0.. 8;
const WEEKDAY_RANGE:ops::Range<u8> = 1.. 8;
*/


/*
fn increment(field: &Entry, current: u32, range: &ops::Range<u8>) -> Next { 
    //return the soonest valid time by calling .next() on all 
    // comma-delimited Values
    let current = current as u8;
    let mut best: Next = Next::worst();
    let mut tmp:  Next;
    for opt in field.iter() {
        let val = opt.next(current, &range);
        tmp = Next::new(val, val<current);
        if tmp < best {
            best = tmp;
        }
    }
    best
}

fn increment_from_start(field: &Entry, range: &ops::Range<u8>) -> Next { 
    increment(field, range.start as u32, range)
}
*/


/*
impl Time {
    pub fn verify(&self) -> bool {
        //test all elements in the minute, hour, date, etc. vectors
        //by calling verify() on all of them and only returning true
        //if all of them do.
        let field_vectors = vec![&self.minute, &self.hour, 
                    &self.date, &self.month, &self.weekday];
        let ranges = vec![MINUTE_RANGE, HOUR_RANGE,
                    DATE_RANGE, MONTH_RANGE, WEEKDAY_RANGE];
        let mut zip = field_vectors.iter().zip(ranges.iter());
        zip.all(|(&fv, ref range)| 
                fv.iter().all(|ref f| f.verify(range)))
    }
    pub fn next(&self) -> datetime::DateTime<Local> {
        //it is useful to be able to specify the `current` time for
        //testing porpoises. otherwise eventually all tests that 
        //compare against a hardcoded date would break :P
        //self.next_date_after_time(Local::now())
        
        //decide which to use:
        //if one is an asterisk, return the other
        //if neither are, return whichever comes first
        let now = Local::now();
        if self.weekday.contains(&STAR) {
            self.next_date_after_time(now)
        } else if self.date.contains(&STAR) {
            self.next_weekday_after_time(now)
        } else {
            let by_date     = self.next_date_after_time(now);
            let by_weekday  = self.next_weekday_after_time(now);
            cmp::min(by_date, by_weekday)
        }
    }

    
    pub fn next_weekday_after_time(&self, 
                                   now: datetime::DateTime<Local>,)  
            -> datetime::DateTime<Local> {
        //weekdays work differently than dates. 
        //If a weekday overflows, the month doesn't necessarily overflow
        //I'm not super certain how to write this function without either
        // duplicating code from next_date_after_time or making spaghetti
        println!("BP A");
        let first_valid_month = increment(&self.month, now.month(), &MONTH_RANGE);
        if first_valid_month.as_u32() != now.month() {
            println!("weekday, overflow");
            //get hour and minute
            //month changed, so they started from 0, so they can't overflow
            let first_valid_minute  = increment_from_start(&self.minute,  &MINUTE_RANGE);
            let first_valid_hour    = increment_from_start(&self.hour,    &HOUR_RANGE);

            //generate date that meets criteria except `weekday`; use the 1st of the month
            let year = now.year() + first_valid_month.overflowed() as i32;
            let mut dt = Local.ymd(year, first_valid_month.as_u32(), 1);

            //get the weekday of the first day of the month, and find how much to 
            //add to it to satisfy the weekday criterion
            let day1_of_week = dt.weekday().num_days_from_sunday();    //from 0
            let first_valid_weekday = increment(&self.weekday, day1_of_week, &WEEKDAY_RANGE);
            let delta = first_valid_weekday.as_u32() + 
                (first_valid_weekday.overflowed() as u32) * 7;

            dt = dt.with_day0(delta).unwrap();
            dt.and_hms(first_valid_hour.as_u32(), first_valid_minute.as_u32(), 0)
            //TODO: TEST
        }
        else {
            println!("Weekday, NOverflow");
            //TODO 
            //TODO: More complicated if fields can overflow 
            //TODO: Support weird Dec/Jan week thing: 
            //  subtract like 14 or 42 days and add them on at the end
            //This should work for both cases
            
            //get hour and minute
            //overflowing is now an option
            let mut first_valid_minute = increment(&self.minute, now.minute(), &MINUTE_RANGE);
            let mut first_valid_hour   = increment(&self.hour,   now.hour(),    &HOUR_RANGE);

            //if the hour changed, reset the minute
            //if the hour didn't change, but the minute overflowed, increase hour by ≥1
            if first_valid_hour.as_u32() != now.hour() {
                first_valid_minute = increment_from_start(&self.minute, &MINUTE_RANGE);
            } else if first_valid_minute.overflowed() {
                first_valid_hour = increment(&self.hour, 
                                             first_valid_hour.as_u32()+1, &HOUR_RANGE);
            }
            //if the minute/hour caused the day to overflow, then `current`
            // is no longer a valid option. So call increment the date
            let mut dt_base = now;
            if first_valid_hour.overflowed() {
                dt_base = dt_base + Duration::days(1);
            }

            //convert date to ISO date (year+week+weekday)
            let (year, week, weekday) = dt_base.isoweekdate();

            let first_valid_weekday = increment(&self.weekday, 
                                                weekday.num_days_from_sunday(), 
                                                &WEEKDAY_RANGE);
            //the "easiest" way to use weeks/weekdays is via chrono's ISO support
            //convert today/tomorrow to a week/weekday, then find the next valid
            //weekday, adjust `week` for any overflow, and recreate the date.
            
            //overflow calculation is harder here. ISO weekday starts with Monday,
            // but rational humans start the week with Sunday.
            //      fuck it

            let _foo = increment(&self.weekday, 3, &WEEKDAY_RANGE);
            println!("TEST: {:?}", _foo);


            let first_valid_week = week + (first_valid_weekday.overflowed() as u32);
            let first_valid_day = match first_valid_weekday.as_u32() {
                //I promise this is way more readable than the other way to do it
                0   => Weekday::Sun,
                1   => Weekday::Mon,
                2   => Weekday::Tue,
                3   => Weekday::Wed,
                4   => Weekday::Thu,
                5   => Weekday::Fri,
                6   => Weekday::Sat,
                7   => Weekday::Sun,
                _   => panic!("Unknown date"),
            };

            //check if this is valid. It may not be if the year overflowed
            //(pretty sure that's the only circumstance, right??)
            let dt_opt = Local.isoywd_opt(year, first_valid_week, first_valid_day);
            let dt = {
                if dt_opt == chrono::LocalResult::None {
                    println!("Recalculating.");
                    //overflowed; increment year and reset week.
                    Local.isoywd(year+1, 1, first_valid_day)
                } else {
                    dt_opt.unwrap()
                }
            };
            
            dt.and_hms(first_valid_hour.as_u32(), first_valid_minute.as_u32(), 0)
        }
    }

    pub fn next_date_after_time(&self, 
                                now: datetime::DateTime<Local>,) 
            -> datetime::DateTime<Local> {
        //TODO: UNFORTUNATELY, THIS IS NOT HOW NUMBERS WORK
        //Need to handle weekdays differently than dates
        
        //let now = now.with_minute(1)
        //store current values, to be replaced as applicable
        //need to increment now.minute by 1, otherwise if `current` is
        //valid it'll just return it (which we don't want)
        let data: [(&Entry, u32, ops::Range<u8>); 4] = 
                       [(&self.minute,  now.minute()+1, MINUTE_RANGE),
                        (&self.hour,    now.hour(),     HOUR_RANGE),
                        (&self.date,    now.day(),      DATE_RANGE),
                        (&self.month,   now.month(),    MONTH_RANGE)];

        let mut result: [Next; 4] = [Next::blank(); 4];
        let mut last_increase: usize = 0;

        //call increment() on all fields once
        for (i, &(field, current, ref range)) in data.iter().enumerate() {
            result[i] = increment(field, current, range);
            if result[i].as_u32() != current {
                last_increase = i;
            }
        }

        //if a significant field changes, all previous fields just reset
        //reset all fields before `last_increase` from min value 
        for i in 0 .. last_increase {
            let (field, _, ref range) = data[i];
            result[i] = increment(field, range.start as u32, range);
        }

        //increment all fields following an overflow
        let mut last_field_overflowed = false;
        for (i, &(field, _, ref range)) in data.iter().enumerate() {
            if last_field_overflowed {
                result[i] = increment(field, result[i].as_u32(), range);
            }
            last_field_overflowed = result[i].overflowed();
        }

        //last_field_overflowed indicates whether year overflowed
        let year_overflow = last_field_overflowed as i32;

        let mut dt_opt = Local.ymd_opt(now.year() + year_overflow, 
                                       result[3].as_u32(),
                                       result[2].as_u32());

        while dt_opt == chrono::LocalResult::None {
            //invalid date, e.g. Feb 30. call next() at most 3 times
            //invalid dates can arise because field.next() treats all 
            //months as if they have 31 days. At most this will need to
            //be called thrice, i.e. Feb 29 -> Feb 30 -> Feb 31 -> Mar N,
            //where Mar N must be valid because March has 31 days.
            //REMEMBER: increment() can return `current` if it's valid
            result[2] = increment(&self.date, result[2].as_u32()+1, &DATE_RANGE);
            if result[2].overflowed() {
                result[3] = increment(&self.month, result[3].as_u32()+1, &MONTH_RANGE);
                //Dec has 31 days, so this will never overflow into the year field
            }
            dt_opt = Local.ymd_opt(now.year() + year_overflow, 
                                   result[3].as_u32(),
                                   result[2].as_u32());
        }

        //hms fields shouldn't ever overwlow
        dt_opt.unwrap().and_hms(result[1].as_u32(),
                                result[0].as_u32(),
                                0)

            //Old but maybe useful comments
        //TODO      FIX THIS LOGIC
        //TODO .next() must be called on all fields to verify they're all valid 
        //TODO If a .next() call overflows, .next() should be called on the 
        //          next largest field again, because it overflowed
        //TODO If .next() is called and changes any field's value (i.e. it's no
        //          longer `current), then all lesser fields should be reset
        //          to their minimum valid values. c.f. 09 -> 10.
        //TODO Figure out a way to do this without making spaghetti
        //
        //All fields must be .next()ed to verify they're valid.
        //If the year  overflowed, reset the month, date, hour, and minute
        //If the month overflowed, reset the        date, hour, and minute
        //...
        //If the minute overflowed, increment the hour
        //If the  hour  overflowed, increment the date
        //...
        //`results`: all fields are valid; fields are Minute, Hour, Day, Month
    }
}
*/
