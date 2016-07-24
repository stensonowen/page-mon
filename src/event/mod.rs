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
use event::chrono::Datelike;
//mod crontime;


pub trait HasNext {
    fn next(&self, current: u8, range: ops::Range<u8>) -> u8;
    //get next value to trigger an event given its current value 
    // (which did not trigger it) and the valid range (which 
    // varied depending on which field this is.
    //If the result is ≤ current, then this field overflowed by 1.
    //Can be buggy on the month field when the max varies. Might
    // not overflow predictably.

    fn next_month(&self, current: u8) -> (u8,u8);
    //similar to `next`, but takes into account the variable 
    // ranges. Also returns both the `next` month value and the
    // number of times it overflowed (this is the only field 
    // where that can be ≥ 2 or cannot be predicted as `next` does.

    fn verify(&self, valid: ops::Range<u8>) -> bool;
    //returns whether this value is valid. 
    //Examples of invalid values include constants outside the 
    // the valid range (e.g. 100) and ranges like 5..2.
}

fn is_leap_year(year: i32) -> bool {
    // https://en.wikipedia.org/wiki/Leap_year#Algorithm
    // chrono's `year` is an i32 but only needs to be u16
    if year % 4 != 0 {
        false 
    } else if year % 100 != 0 {
        true 
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}


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
    fn next_month(&self, current: u8) -> (u8,u8) {
        //`max` depends on which month and year it is. `current`
        // should always refer to the month the program is executing
        let month_lens: [u8;12] = [31,0,31,30,31,30,31,31,30,31,30,31];
        //30: sep april june november
        let max: u8 = {
            if current != 2 {
                month_lens[current as usize]
            } else {
                let year = chrono::Local::today().year();
                if is_leap_year(year) {
                    29
                } else {
                    29
                }
            }

        };
        match *self {
            ContVal::Asterisk   => {
                let next = self.next(current, 1..2);
                let overflow = match next <= current {
                    true  => 1,
                    false => 0,
                };
                (next,overflow)
            },
            _ => (0,0),
        }
    }
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
    fn next_month(&self, current: u8) -> (u8,u8) {
        (0,0)
    }
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
