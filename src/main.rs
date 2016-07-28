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

mod event;
pub mod croncfg;
pub mod ast;
//use ast::*;
use event::HasNext;

extern crate chrono;
use chrono::{Local, TimeZone};

fn main() {

    let   valid_cmd = croncfg::parse_Command("10-20/7 2 3-5 */5 7 https://valid.com");
    assert!(valid_cmd.is_ok());
    let vt = valid_cmd.unwrap().time;
    for entry in &vt.minute { assert!(entry.verify(&(0..60))); }
    for entry in &vt.hour   { assert!(entry.verify(&(0..24))); }
    for entry in &vt.date   { assert!(entry.verify(&(1..32))); } //variable
    for entry in &vt.month  { assert!(entry.verify(&(1..13))); }
    for entry in &vt.weekday{ assert!(entry.verify(&(0.. 8))); } //0 = 7 = SUN
    assert!(vt.verify());
    assert!(vt.minute[0]    .next(0, &(0..60)) == 14);
    assert!(vt.hour[0]      .next(3, &(0..24)) ==  2);
    assert!(vt.date[0]      .next(5, &(1..32)) ==  3);
    assert!(vt.month[0]     .next(12,&(1..13)) ==  5);
    assert!(vt.weekday[0]   .next(7, &(0.. 8)) ==  7);

    let invalid_cmd = croncfg::parse_Command("60 */0 0 2-1 5-9 https://invalid.com");
    assert!(invalid_cmd.is_ok());
    let it = invalid_cmd.unwrap().time;
    for entry in &it.minute  { assert!(!entry.verify(&(0..60))); }
    for entry in &it.hour    { assert!(!entry.verify(&(0..24))); }
    for entry in &it.date    { assert!(!entry.verify(&(1..32))); } //variable
    for entry in &it.month   { assert!(!entry.verify(&(1..13))); }
    for entry in &it.weekday { assert!(!entry.verify(&(0.. 8))); } //0 = 7 = SUN
    assert!(!it.verify());
    

    //let test_cmd = croncfg::parse_Command("59 1 30-31 1 * https://invalid.com");
    //assert!(test_cmd.is_ok());
    //let tm = test_cmd.unwrap().time;
    //println!("Time: {:?}", tm);
    //println!("Now:  {}", chrono::Local::now());
    //println!("Next: {}", tm.next());
    
    //trigger at 23:35 on some even day between the 13th and the 27th of any month
    //starting at Jan 1, 1970 00:00:00  -> next is Jan 14, 1970 23:35:00
    let test1 = croncfg::parse_Command("35 23 13-27/2 * * https://test.com").unwrap().time;
    assert!(Local.ymd(1970, 01, 14).and_hms(23, 35, 00) == 
            test1.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));

    //trigger at the 0,10,20,30,40,50th minute of hour 0,7,14,21 on Jan 13,14,15
    //starting at Jan 1, 1970 00:00:00  -> next is Jan 13, 1970 00:00:00
    //starting at Jan 1, 1970 00:00:00  -> next is Jan 13, 1970 
    let test2 = croncfg::parse_Command("*/10 */7 13-15 01 * https://test.com").unwrap().time;
    assert!(Local.ymd(1970, 01, 13).and_hms(00, 00, 00) == 
            test2.next_after_time(Local.ymd(1970, 01, 01).and_hms(00, 00, 00)));
    

    assert!(croncfg::parse_Line("* * * * * http://www.google.com").is_ok());
    assert!(croncfg::parse_Line("1 2 3 4 5 https://google.com").is_ok());
    assert!(croncfg::parse_Line("1/1 2 3 4 5 https://google.com").is_err());
    //println!("{:?}", croncfg::parse_Line("1 2 3 4 5 https://google.com").unwrap());
    //println!("{:?}", croncfg::parse_Line("1*2\t4*/4https://bing.com").unwrap());
    //println!("{:?}", croncfg::parse_Line("*****https://reddit.com").unwrap());
    //println!("{:?}", croncfg::parse_Line("@yearly https://ddg.co").unwrap());
    //println!("{:?}", croncfg::parse_Line("* * * JAN SUN https://teamfortresstv.com").unwrap());
    //println!("{:?}", croncfg::parse_Line("* * * MaR fRi https://teamfortresstv.com").unwrap());
}
