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
 *
 *
 *
 */

extern crate hyper;
extern crate chrono;
//use std::time;
use std::{ops, cmp};
use ast::*;
//use event::chrono::Datelike;
//mod crontime;


pub trait HasNext {
    fn next(&self, current: u8, range: ops::Range<u8>) -> u8;
    //get next value to trigger an event given its current value 
    // (which did not trigger it) and the valid range (which 
    // varied depending on which field this is.
    //If the result is ≤ current, then this field overflowed by 1.
    //Can be buggy on the month field when the max varies. Might
    // not overflow predictably.
    //
    //So I spent a full day trying to implement the next_month() corner
    //case because not all months have the same number of days before
    //realizing I didn't need to, so I'm going to explain myself.
    //The way next() works is by checking the minimum value that's in
    //range and higher than `current`, and if no such number exists it 
    //returns the minimum valid value. Ordinarily, next() doesn't need
    //to track overflow because next() overflowed by 1 if next<=current
    //and 0 otherwise. However, because some months have fewer than 31 
    //days, sometimes a Value (e.g. 30) is only valid for certain months.
    //Thus not only would next() have to check with a variable `max`, 
    //it would also have to keep track of the number of times it over-
    //flowed. However, the Gregorian calendar taketh away, but it also
    //giveth: there are no consecutive months that both do not have the
    //maximum number of days (31). That means this bug will never happen,
    //because if the current month has 31 days ...
    //dammit
    //Scratch that. If we have `@monthly` and it's Jan 30, then next() 
    //yields Feb 30, and correcting that results in an overflow by 2.
    //So.
    //next() is almost always right. It can be wrong if the month after 
    //the `current` month has fewer days than the upper limit of the range. 
    //This is easier to fix. Instead of writing a new function to generate
    //the next, we can take the final date with all relevant fields 
    //updated and verify that it's valid. If it's invalid, we call next()
    //on the date field at most 3 more times until the date is valid.
    //The worst case scenario (next()ing 3 times) is when `current` is 
    //January and `range.end` is 31 (and `range.start`<29): then next()
    //gives us Feb 29, which is invalid (usually), and next() gives 
    //Feb 30, Feb 31, and finally Mar N. Mar N is guaranteed to be valid 
    //because March has 31 days and each next() overflows by at most 1.
    //Christ.

    //fn next_month(&self, current: u8) -> (u8,u8);
    //fn next_month(&self, current_month: u8, current_year: u16) -> (u8,u8);
    //ACUTALLY... February has 28-29 days but March has 31. So 
    // there's no way `next` would skip Feb AND Mar unless it 
    // were invalid.

    //similar to `next`, but takes into account the variable 
    // ranges. Also returns both the `next` month value and the
    // number of times it overflowed (this is the only field 
    // where that can be ≥ 2 or cannot be predicted as `next` does.

    fn verify(&self, valid: ops::Range<u8>) -> bool;
    //returns whether this value is valid. 
    //Examples of invalid values include constants outside the 
    // the valid range (e.g. 100) and ranges like 5..2.
}

/*fn is_leap_year(year: u16) -> bool {
    // https://en.wikipedia.org/wiki/Leap_year#Algorithm
    if year % 4 != 0 {
        false 
    } else if year % 100 != 0 {
        true 
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}*/

//const MONTH_LENS: [u8;12] = [31,0,31,30,31,30,31,31,30,31,30,31];//good placeholder?


impl HasNext for ContVal {
    fn next(&self, current: u8, range: ops::Range<u8>) -> u8 {
        //safe to assume current \in range
        match *self {
            ContVal::Asterisk       => {
                let guess = current + 1;
                if guess < range.end {
                    guess
                } else {
                    range.start
                }
            }
            ContVal::Range(min,max) => {
                //not safe to assume that min <= current <= max
                //TODO: verify min < max
                //TODO: increment max by one: cron ranges are inclusive
                //TODO: verify min and max both in range
                let guess = current + 1;
                if guess >= min && guess < max {
                    guess
                } else {
                    min
                }
            }
        }
    }
    /*
    fn next_month(&self, current_month: u8, current_year: u16) -> (u8,u8) {
        //`max` depends on which month and year it is. `current`
        // should always refer to the month the program is executing
        //30: sep april june november
        let month_len: u8 = {
            if current_month != 2 {
                MONTH_LENS[current_month as usize]
            } else {
                //let year:i32 = chrono::Local::today().year();
                if is_leap_year(current_year)   { 29 }
                else                            { 28 }
            }

        };
        match *self {
            ContVal::Asterisk   => {
                let next = self.next(current_month, 1..month_len);
                let overflow = match next <= current_month {
                    true  => 1,
                    false => 0,
                };
                (next,overflow)
            },
            ContVal::Range(min,max) => {
                //it's okay if this bit is uninspired, it's such a
                // niche corner case who cares.
                let mut overflow = 0;
                for i in 0..12 {
                    let next = 
                }
            }
        }
    }
    */
    fn verify(&self, valid: ops::Range<u8>) -> bool {
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
    //fn next_month(&self, current_month: u8, current_year: u16) -> (u8,u8) {
    //    (0,0)
    //}
    fn next(&self, current: u8, range: ops::Range<u8>) -> u8 {
        match *self {
            Value::CV(ref cv)   => cv.next(current, range),
            Value::Constant(c)  => c,
            Value::Skip(ref cv, mult) => {
                let start = match *cv {
                    ContVal::Asterisk => {
                        // `*/mult`
                        //need a multiple of n that is greater than current
                        //by as small a margin as possible
                        //TODO: verify mult ≠ 0
                        current
                    },
                    ContVal::Range(min,max) => {
                        // `min-max/mult`
                        //event wasn't necessarily fired at `current` time
                        //TODO: verify min and max are in range and min < max
                        if current >= max {
                            min-1
                        } else {
                            cmp::max(current,min-1)
                        }
                    },
                };
                //hard to predict whether `guess` will exceed the hard max
                //there's probably a better way to do this?
                let guess = (start / mult + 1) * mult;
                if guess >= range.end {
                    self.next(0, range)
                } else {
                    guess
                }

            }
        }
    }
    fn verify(&self, valid: ops::Range<u8>) -> bool {
        match *self {
            Value::CV(ref cv)   => cv.verify(valid),
            Value::Constant(c)  => valid.start<=c && c<valid.end,
            Value::Skip(ref cv, mult) => 
                mult != 0 && mult < valid.end && cv.verify(valid), 
        }
    }
}


/*
pub struct Field {
    //e.g. Minute, Month
    range:  ops::Range<u8>,
    valid:  Vec<Special>,
}

#[allow(dead_code)]
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
*/

/*
#[allow(dead_code)]
pub struct Event {
    url:    hyper::Url,
    period: time::Duration,

}


pub fn foo() {
    let ct = crontime::CronTime::from_string("a b c d e");
    println!("{:?}", ct);
}
*/
