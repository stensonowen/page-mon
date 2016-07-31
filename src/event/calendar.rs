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

//pub mod value_item;
use event::value_item::*;
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
}


pub fn collect_vals(vals: &mut Vec<Value>, range: ops::Range<u8>) -> ValidSet {
    let mut cal = ValidSet::new();
    for mut value in vals.into_iter() {
        let val_item = ValueItem::new(&mut value, &range);
        for possibility in val_item.into_iter() {
            cal.insert(possibility);
        }
    }
    cal
}

