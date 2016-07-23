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
//use std::time;
use std::{ops, cmp};
use ast::*;
//mod crontime;


pub trait HasNext {
    fn next(&self, current: u8, range: ops::Range<u8>) -> u8;
    fn verify(&self, valid: ops::Range<u8>) -> bool;
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
