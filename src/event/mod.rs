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
 *
 *
 *
 */

extern crate hyper;

extern crate chrono;
use self::chrono::{Local, datetime, TimeZone};
use event::chrono::{Timelike, Datelike};

use std::{ops, cmp, u8};

mod next;
use event::next::Next;

use ast::*;
//use event::chrono::Datelike;
//mod crontime;


pub trait HasNext {
    fn next(&self, current: u8, range: &ops::Range<u8>) -> u8;
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
    //https://en.wikipedia.org/wiki/International_Fixed_Calendar

    fn verify(&self, valid: &ops::Range<u8>) -> bool;
    //returns whether this value is valid. 
    //Examples of invalid values include constants outside the 
    // the valid range (e.g. 100) and ranges like 5..2.
}


impl HasNext for ContVal {
    fn next(&self, current: u8, range: &ops::Range<u8>) -> u8 {
        //safe to assume current \in range
        //TODO: track overflow. field overflowed by 1 if 
        // next<=current or 0 otherwise.
        //TODO: verify resulting date is valid. May need to call 
        // next() up to three more times* to result in a valid date
        // in the case of `current:2`, `range:29..32`.
        // And remember to track the overflow again.
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
                //DONE: verify min < max
                //TODO: increment max by one: cron ranges are inclusive
                //DONE: verify min and max both in range
                let guess = current + 1;
                if guess >= min && guess < max {
                    guess
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
                let start = match *cv {
                    ContVal::Asterisk => {
                        // `*/mult`
                        //need a multiple of n that is greater than current
                        //by as small a margin as possible
                        //DONE: verify mult ≠ 0
                        current
                    },
                    ContVal::Range(min,max) => {
                        // `min-max/mult`
                        //event wasn't necessarily fired at `current` time
                        //DONE: verify min and max are in range and min < max
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
                    self.next(0, &range)
                } else {
                    guess
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
}


//Valid ranges for each field
//Lower bound included, upper bound excluded
const MINUTE_RANGE: ops::Range<u8> = 0..60;
const HOUR_RANGE:   ops::Range<u8> = 0..24;
const DATE_RANGE:   ops::Range<u8> = 1..32;
const MONTH_RANGE:  ops::Range<u8> = 1..13;
const WEEKDAY_RANGE:ops::Range<u8> = 0.. 8;


fn increment(field: &Entry, current: u32, range: &ops::Range<u8>) -> Next { 
    let current = current as u8;
    let mut best: Next = Next::worst();
    let mut tmp:  Next;
    for opt in field.iter() {
        let val = opt.next(current, &range);
        tmp = Next::new(val, val<=current);
        if tmp < best {
            best = tmp;
        }
    }
    best
}

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
        let now = Local::now();
        //increment by date or weekday, whichever is less
        //unless one is Asterisk, then increment by the other
        //unless both are Asterisk, in which case it doesn't matter

        let data = vec![(&self.minute,  now.minute(),   MINUTE_RANGE),
                        (&self.hour,    now.hour(),     HOUR_RANGE),
                        (&self.date,    now.day(),      DATE_RANGE),
                        (&self.month,   now.month(),    MONTH_RANGE),
                        (&self.weekday, now.weekday().num_days_from_sunday(),  
                                                        WEEKDAY_RANGE)];
        //store current values, to be replaced as applicable
        
        //`year` field shenanigans: `year` is the only field than can 
        // overflow a u8. But we technically need to store the year
        // to use this general solution for tracking overflow.
        // So pretend the year is 0 in `result` and add the current 
        // year back (to either 0 or 1) in the dt_opt ymd assignment
        let mut result: [Next; 5] = [Next::from_n(now.minute()),
                    Next::from_n(now.hour()),  Next::from_n(now.day()),
                    //Next::from_n(now.month()), Next::from_n(now.year())];
                    Next::from_n(now.month()), Next::from_n(0)];
                    
        for (i, &(field, current, ref range)) in data.iter().enumerate() {
            result[i] = increment(field, current, range);
            if result[i].overflowed() == false {
                break;
            }
        }

        let dt_opt = Local.ymd_opt(result[4].as_u32() as i32 + now.year(), 
                                   result[3].as_u32(),
                                   result[2].as_u32());
        if dt_opt == chrono::offset::LocalResult::None {
            //invalid dates can arise because field.next() treats all 
            //months as if they have 31 days. At most this will need to
            //be called thrice, i.e. Feb 29 -> Feb 30 -> Feb 31 -> Mar N,
            //where Mar N must be valid because March has 31 days.
            self.next()
        } else {
            dt_opt.unwrap().and_hms(result[1].as_u32(),
                                    result[0].as_u32(),
                                    0)
        }
    }
}

