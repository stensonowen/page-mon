//All character info taken from https://en.wikipedia.org/wiki/Cron
//Shoutout https://crontab.guru for being interesting and semi-relevant
//Excludes only `%`, because I'm pretty sure it's irrelevant here
use std::fmt::{Debug, Formatter, Error};


pub enum Line {
    //separated by newlines
    Comment,
    Cmd(Command),
}

#[derive(Debug)]
pub struct Command {
    pub minute:     Entry,
    pub hour:       Entry,
    pub date:       Entry,
    pub month:      Entry,
    pub weekday:    Entry,
    pub url:        Option<String>,
}

pub type Entry = Vec<Value>;
//separated by spaces

#[derive(Debug)]
pub enum Value {
    //separated by commas
    Skip(ContVal, u8),
    CV(ContVal),
}

#[derive(Debug)]
pub enum ContVal {
    //Values are contiguous, e.g. 4 or 1-5 or * but not */2
    Asterisk,
    Constant(u8),
    Range(u8,u8),
}

impl Debug for Line {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if let Line::Cmd(ref cmd) = *self {
            try!(write!(fmt, "{:?}\n", cmd.url));
            try!(write!(fmt, "\tminute:\t\t{:?}\n", cmd.minute));
            try!(write!(fmt, "\thour:\t\t{:?}\n",   cmd.hour));
            try!(write!(fmt, "\tdate:\t\t{:?}\n",   cmd.date));
            try!(write!(fmt, "\tmonth:\t\t{:?}\n",  cmd.month));
            write!(fmt, "\tweekday:\t{:?}\n", cmd.weekday)
        } else {
            write!(fmt, "#Comment")
        }
    }
}
