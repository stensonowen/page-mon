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
        // next<current or 0 otherwise.
        //TODO: verify resulting date is valid. May need to call 
        // next() up to three more times* to result in a valid date
        // in the case of `current:2`, `range:29..32`.
        // And remember to track the overflow again.
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
                //DONE: verify min < max
                //TODO: increment max by one: cron ranges are inclusive
                //DONE: verify min and max both in range
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
                //let guess = ((start-1)/mult+1)*mult;
                //let guess = (((start.wrapping_sub(1))/mult)
                //             .wrapping_add(1)).wrapping_mul(mult);
                let mult_ = mult as i16;
                let start_ = start as i16;
                let guess: i16 = ((start_-1)/mult_+1)*mult_;
                let guess = guess as u8;
                if guess < max {
                    //can't be < range.start
                    //in range: this is the answer
                    guess
                } else {
                    //the smallest multiple of `skip` that
                    //exceeds `current` also exceeds `end`,
                    //so it overflowed.
                    //0
                    //TODO: verify that there is at least one valid 
                    //answer. e.g. nothing like `20-25/9`
                    //((min-1)/mult+1)*mult
                    //(((min.wrapping_sub(1))/mult).wrapping_add(1)).wrapping_mul(mult)
                    (((min as i16 -1)/mult_+1)*mult_) as u8
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
    //return the soonest valid time by calling .next() on all 
    // comma-delimited Values
    let current = current as u8;
    let mut best: Next = Next::worst();
    let mut tmp:  Next;
    for opt in field.iter() {
        let val = opt.next(current, &range);
        //tmp = Next::new(val, val<=current);
        tmp = Next::new(val, val<current);
        //shit. is this a problem?
        //overflow is calculated by checking `val<current`.
        //The new .next() system returns `current` if it's valid
        //So if minute=Const(42) and current=42, it'll return now
        // which looks like an overflow (like it means NextHour:42)
        // but it won't.
        //BUT we call it on current_time + 1 minute
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
        //it is useful to be able to specify the `current` time for
        //testing porpoises. otherwise eventually all tests that 
        //compare against a hardcoded date would break :P
        self.next_given_time(Local::now())
    }

    pub fn next_given_time(&self, 
                           now: datetime::DateTime<Local>) 
                            -> datetime::DateTime<Local> {
        //call .next() on all fields because `current` might not be valid.
        //now if `current` is valid, .next() will return it.
        //if a field overflows, that means .next() must be called once more.
        
        //The Plus One problem:
        //We can't call next() on `current_time` because it is useless to 
        //us if the current time is valid, and field.next() is now inclusive.
        //So we increment the `minute` field by 1 so that the earliest Next
        //can be is one minute in the future. 
        //Is it fine to increment now.minute() by 1, as in data[0][1]?
        //Or should we call `let now = Local::now().with_minute(1)`?
        //There shouldn't be a difference, right? Except the former relies
        //on my code and the latter relies on chrono?
        
        //let now = Local::now();
        
        //TODO: add weekday part
        //increment by date or weekday, whichever is less
        //unless one is Asterisk, then increment by the other
        //unless both are Asterisk, in which case it doesn't matter

        //need to increment now.minute by 1, otherwise if `current` is
        //valid it'll just return it (which we don't want)
        let data = vec![(&self.minute,  now.minute()+1, MINUTE_RANGE),
                        (&self.hour,    now.hour(),     HOUR_RANGE),
                        (&self.date,    now.day(),      DATE_RANGE),
                        (&self.month,   now.month(),    MONTH_RANGE)];
                        //(&self.weekday, now.weekday().num_days_from_sunday(),  
                        //                                WEEKDAY_RANGE)];
        //store current values, to be replaced as applicable
        
        //`year` field shenanigans: `year` is the only field than can 
        // overflow a u8. But we technically need to store the year
        // to use this general solution for tracking overflow.
        // So pretend the year is 0 in `result` and add the current 
        // year back (to either 0 or 1) in the dt_opt ymd assignment
        let mut result: [Next; 4] = [Next::from_n(now.minute()),
                    Next::from_n(now.hour()),  Next::from_n(now.day()),
                    //Next::from_n(now.month()), Next::from_n(now.year())];
                    Next::from_n(now.month())];//, Next::from_n(0)];
                    
        let mut overflowed = true;

        for (i, &(field, current, ref range)) in data.iter().enumerate() {
            result[i] = increment(field, current, range);
            //overflowed &= result[i].overflowed();
            if overflowed {
                //everything from `minute` through i-1 overflowed, so 
                // even though result[i] is valid, find the *next* value
                result[i] = increment(field, result[i].as_u32(), range);
            }
            overflowed = result[i].overflowed();
        }
        //let year_overflow = result[3].overflowed() as i32;
        let year_overflow = overflowed as i32;//??
        //pretty sure this should work. result[3] can only be overflowed k

        let mut dt_opt = Local.ymd_opt(now.year() + year_overflow, 
                                       result[3].as_u32(),
                                       result[2].as_u32());

        while dt_opt == chrono::offset::LocalResult::None {
            //invalid date, e.g. Feb 30. call next() at most 3 times
            //invalid dates can arise because field.next() treats all 
            //months as if they have 31 days. At most this will need to
            //be called thrice, i.e. Feb 29 -> Feb 30 -> Feb 31 -> Mar N,
            //where Mar N must be valid because March has 31 days.
            result[2] = increment(&self.date, result[2].as_u32(), &DATE_RANGE);
            if result[2].overflowed() {
                result[3] = increment(&self.month, result[3].as_u32(), &MONTH_RANGE);
                //Dec has 31 days, so this will never overflow into the year field
            }
            dt_opt = Local.ymd_opt(now.year() + year_overflow, 
                                   result[3].as_u32(),
                                   result[2].as_u32());
        }

        //hms fields shouldn't ever overwloe
        dt_opt.unwrap().and_hms(result[1].as_u32(),
                                result[0].as_u32(),
                                0)
    }
}

