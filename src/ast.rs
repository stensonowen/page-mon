//All character info taken from https://en.wikipedia.org/wiki/Cron
//Shoutout https://crontab.guru for being interesting and semi-relevant
//Excludes only `%`, because I'm pretty sure it's irrelevant here
use std::fmt::{Debug, Formatter, Error};

#[derive(Debug)]
pub enum Entry {    //comma-delineated options for one 'place', e.g. minute
    //Each of these options is always allowed; 
    // constant, range, and (at least) asterisk are always valid
    Constant(u8),   //should never exceed 59
    Special(Special),// *, L, W, ?
    Range(u8,u8),   // (lower bound, upper bound)
}

#[derive(Debug)]
pub enum Special {
    //Not necessarily always allowed; everything but Asterisk 
    Asterisk,
    L,
    W,
    Question,
    Hash(u8),       //`#2`  ↔ the second X-day of the month
    Slash(u8),      //`*/4` ↔ all values that are multiples of 4 
}

pub type Entries = Vec<Entry>;  //One value; e.g. `*` in the `minute` field

pub struct Line {
    //though there are slight differences between these fields,
    // they will be treated as identical until later.
    // verifying the logic of the entry should not be part of the parser
    pub minute:     Entries,
    pub hour:       Entries,
    pub date:       Entries,
    pub month:      Entries,
    pub weekday:    Entries,
    pub url:        String,
}


impl Debug for Line {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(write!(fmt, "{}\n", self.url));
        try!(write!(fmt, "\tminute:\t\t{:?}\n",   self.minute));
        try!(write!(fmt, "\thour:\t\t{:?}\n",     self.hour));
        try!(write!(fmt, "\tdate:\t\t{:?}\n",     self.date));
        try!(write!(fmt, "\tmonth:\t\t{:?}\n",    self.month));
        write!(fmt, "\tweekday:\t{:?}\n", self.weekday)
    }
}
