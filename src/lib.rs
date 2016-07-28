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

//Test Suite:
//Note: `cargo test` will say most of these traits are unused.
//  This is false. They are necessary.

mod event;
pub mod croncfg;
pub mod ast;
extern crate chrono;

#[cfg(test)]
mod tests {
    use super::*;
    use event::HasNext;
    use chrono::{Local, TimeZone};

    #[test]
    #[ignore]
    fn parse0() {
        //verify lines can be parsed correctly
        assert!(croncfg::parse_Line("* * * * * http://www.google.com").is_ok());
        assert!(croncfg::parse_Line("1 2 3 4 5 https://google.com").is_ok());
        assert!(croncfg::parse_Line("1/1 2 3 4 5 https://google.com").is_err());
    }

    #[test]
    #[ignore]
    fn parse1() {
        //verify valid entries can be recognized
        let   valid_cmd = croncfg::parse_Command("10-20/7 2 3-5 */5 7 https://valid.com");
        assert!(valid_cmd.is_ok());
        let vt = valid_cmd.unwrap().time;
        for entry in &vt.minute { assert!(entry.verify(&(0..60))); }
        for entry in &vt.hour   { assert!(entry.verify(&(0..24))); }
        for entry in &vt.date   { assert!(entry.verify(&(1..32))); } //variable
        for entry in &vt.month  { assert!(entry.verify(&(1..13))); }
        for entry in &vt.weekday{ assert!(entry.verify(&(0.. 8))); } //0 = 7 = SUN
        assert!(vt.verify());
    }

    #[test]
    #[ignore]
    fn parse2() {
        //verify invalid entries can be recognized
        let invalid_cmd = croncfg::parse_Command("60 */0 0 2-1 5-9 https://invalid.com");
        assert!(invalid_cmd.is_ok());
        let it = invalid_cmd.unwrap().time;
        for entry in &it.minute  { assert!(!entry.verify(&(0..60))); }
        for entry in &it.hour    { assert!(!entry.verify(&(0..24))); }
        for entry in &it.date    { assert!(!entry.verify(&(1..32))); } //variable
        for entry in &it.month   { assert!(!entry.verify(&(1..13))); }
        for entry in &it.weekday { assert!(!entry.verify(&(0.. 8))); } //0 = 7 = SUN
        assert!(!it.verify());
    }    


    #[test]
    #[ignore]
    fn next0() {
        //verify .next() works on entries
        let valid_cmd = croncfg::parse_Command("10-20/7 2 3-5 */5 7 https://valid.com");
        let vt = valid_cmd.unwrap().time;
        assert_eq!(vt.minute[0] .next(0, &(0..60)), 14);
        assert_eq!(vt.hour[0]   .next(3, &(0..24)),  2);
        assert_eq!(vt.date[0]   .next(5, &(1..32)),  3);
        assert_eq!(vt.month[0]  .next(12,&(1..13)),  5);
        assert_eq!(vt.weekday[0].next(7, &(0.. 8)),  7);
    }

    #[test]
    #[ignore]
    fn next1() {
        //verify .next() works on Time object
        //trigger at 23:35 on some even day between the 13th and the 27th of any month
        //starting at Jan 1, 1970 00:00:00  -> next is Jan 14, 1970 23:35:00
        let test = croncfg::parse_Command("35 23 13-27/2 * * https://test.com").unwrap().time;
        assert_eq!(Local.ymd(1970, 01, 14).and_hms(23, 35, 00), 
                   test.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));
    }

    #[test]
    #[ignore]
    fn next2() {
        //verify .next() works on Time object
        //trigger at the 0,10,20,30,40,50th minute of hour 0,7,14,21 on Jan 13,14,15
        //starting at Jan 1, 1970 00:00:00  -> next is Jan 13, 1970 00:00:00
        let test2 = croncfg::parse_Command("*/10 */7 13-15 01 * https://test.com").unwrap().time;
        assert_eq!(Local.ymd(1970, 01, 13).and_hms(00, 00, 00), 
                   test2.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));
    }    

    #[test]
    fn next3() {
        //verify .next() works on Time object
        //23:59 on Dec 31
        //starting at Jan 1, 1970 00:00:00  -> next is Dec 31, 23:59
        let test = croncfg::parse_Command("59 23 31 12 * https://test.com").unwrap().time;
        assert_eq!(Local.ymd(1970, 12, 31).and_hms(23, 59, 00), 
                   test.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));
    }

    #[test]
    fn next4() {
        //verify .next() works on Time object around leap years
        //imdnight on the 29th-31st of each month
        //starting at Jan 13, 1970 00:00:00  -> Mar 29, 1970 00:00
        let test = croncfg::parse_Command("0 0 29-32 * * https://test.com").unwrap().time;
        assert_eq!(Local.ymd(1970, 03, 29).and_hms(00, 00, 00), 
                   test.next_after_time(Local.ymd(1970, 02, 01).and_hms(00, 00, 00)));
    }

    #[test]
    fn next5() {
        //verify .next() works on Time object with weekdays
        //trigger every minute of every Monday of february
        let test = croncfg::parse_Command("* * * 2 MON https://test.com").unwrap().time;
        assert_eq!(Local.ymd(1970, 02, 02).and_hms(00, 00, 00), 
                   test.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));
    }

    #[test]
    #[ignore]
    fn next6() {
        //verify .next() works on Time object with weekdays
        //trigger every minute of every Monday of february
        //starting at Jan 1, 1970 00:00:00  -> next is Jan 14, 1970 23:35:00
        let test = croncfg::parse_Command("* * * 2 1 https://test.com").unwrap().time;
        assert_eq!(Local.ymd(1970, 02, 01).and_hms(00, 00, 00), 
                   test.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));
    }
}
