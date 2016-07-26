/*  Periodically crawl web pages and alert the user of changes
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
 *  More information in the enclosed `LICENSE' file
 */

//Based on http://linux.die.net/man/5/crontab
//
//It would be interesting to try to use pseudo-polymorphism instead of enums.
//You can impl an enum but not its variant, but it would be nice to call 
// `.next()` on, say, Value::ContVal::Asterisk. Entry would have to become
// a generic vector which stores anything implementing the `Next` trait.
//This has the drawback of I'm not sure if it's possible. Also apparently
//generic typedefs are a no-no. I'll leave it alone for now.

//TODO: don't forget that 0 = 7 = Sunday

use std::fmt::{Debug, Formatter, Error};


pub enum Line {
    //separated by newlines
    Comment,
    Cmd(Command),
}

#[derive(Debug)]
pub struct Command {
    pub time:   Time,
    pub url:    String,
}

//#[derive(Debug)]
pub struct Time {
    pub minute:     Entry,
    pub hour:       Entry,
    pub date:       Entry,
    pub month:      Entry,
    pub weekday:    Entry,
}

impl Time {
    pub fn from(min: Value, hr: Value, date: Value, 
                mon: Value, wd: Value) -> Time {
        //shortcut to quickly construct a Time object
        //useful for nicknames in croncfg.lalrpop
        Time {
            minute:     vec![min],
            hour:       vec![hr],
            date:       vec![date],
            month:      vec![mon],
            weekday:    vec![wd],
        }
    }
}

pub type Entry = Vec<Value>;
//separated by spaces

#[derive(Debug)]
pub enum Value {
    //separated by commas
    Skip(ContVal, u8),
    CV(ContVal),
    Constant(u8),
}

#[derive(Debug)]
pub enum ContVal {
    //Values are contiguous, e.g. 1-5 or * but not */2 or 4
    Asterisk,
    Range(u8,u8),
}

//for convenient setting
pub const ZERO: Value = Value::Constant(0);
pub const ONE:  Value = Value::Constant(1);
pub const STAR: Value = Value::CV(ContVal::Asterisk);

impl Debug for Time {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
            try!(write!(fmt, "Time Object:\n"));
            try!(write!(fmt, "\tminute:\t\t{:?}\n", self.minute));
            try!(write!(fmt, "\thour:\t\t{:?}\n",   self.hour));
            try!(write!(fmt, "\tdate:\t\t{:?}\n",   self.date));
            try!(write!(fmt, "\tmonth:\t\t{:?}\n",  self.month));
            write!(fmt, "\tweekday:\t{:?}\n", self.weekday)
    }
}

impl Debug for Line {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if let Line::Cmd(ref cmd) = *self {
            try!(write!(fmt, "{:?}\n", cmd.url));
            try!(write!(fmt, "\tminute:\t\t{:?}\n", cmd.time.minute));
            try!(write!(fmt, "\thour:\t\t{:?}\n",   cmd.time.hour));
            try!(write!(fmt, "\tdate:\t\t{:?}\n",   cmd.time.date));
            try!(write!(fmt, "\tmonth:\t\t{:?}\n",  cmd.time.month));
            write!(fmt, "\tweekday:\t{:?}\n", cmd.time.weekday)
        } else {
            write!(fmt, "#Comment")
        }
    }
}
