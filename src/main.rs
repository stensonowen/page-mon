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

/* main.rs is a mess
 *  mostly used to test stuff before it goes in lib.rs as a unit test.
 *  pardon the sloppiness
 */

mod event;
pub mod croncfg;
pub mod ast;

use event::calendar::*;

extern crate chrono;
use chrono::Local;

fn main() {

    //let tmp = Local.isoywd(2016, 1, chrono::Weekday::Mon);
    //println!("TMP: {}", tmp);

  //let mut tmp = croncfg::parse_Command("1-10,10-20/2,20-30/3 * * * 0 https://test.com").unwrap();
    let mut tmp = croncfg::parse_Command("* * * * 0 https://test.com").unwrap();
    //let tmp: ast::Value = test.minute[0].clone();
    //let range = 0..60;
    let cal = Calendar::from_time(&mut tmp.time);
    let fire_now = cal.fire_now(Local::now());
    println!("{:?}", fire_now);
    //for mut itr in test.minute.into_iter() {
    //    let range = 0..60;
    //    let vi = ValueItem::new(&mut itr, &range);
    //    for i in vi.into_iter() {
    //        println!("\t{}", i);
    //    }
    //}
    //for i in tmp {
    //    println!("\t{:?}", i);
    //}
    //assert_eq!(Local.ymd(1970, 02, 05).and_hms(00, 00, 00), 
    //           test.next_weekday_after_time(Local.ymd(1970, 02, 01).and_hms(00, 00, 00)));
}
