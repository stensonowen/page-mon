//All character info taken from https://en.wikipedia.org/wiki/Cron
//Shoutout https://crontab.guru for being interesting and semi-relevant
//Excludes only `%`, because I'm pretty sure it's irrelevant here

#[derive(Clone)]
pub enum Entry {    //comma-delineated options for one 'place', e.g. minute
    Constant(u8),   //should never exceed 59
    Special(Special),// *, L, W, ?
    Hash(u8),       //`#2`  ↔ the second X-day of the month
    Slash(u8),      //`*/4` ↔ all values that are multiples of 4 
    Range(u8,u8),   // (lower bound, upper bound)
}

#[derive(Clone)]
pub enum Special {
    Asterisk,
    L,
    W,
    Question,
}

pub type Entries = Vec<Entry>;  //One value; e.g. `*` in the `minute` field

#[derive(Clone)]
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
