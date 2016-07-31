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
//#[allow(unused_imports)]
//use event::HasNext;

pub mod croncfg;
pub mod ast;

use event::value_item::ValueItem;
//extern crate chrono;
//use chrono::{Local, TimeZone};

fn main() {

    //let tmp = Local.isoywd(2016, 1, chrono::Weekday::Mon);
    //println!("TMP: {}", tmp);

    let test = croncfg::parse_Command("3-24/3 * * * 0 https://test.com").unwrap().time;
    //let tmp: ast::Value = test.minute[0].clone();
    for itr in test.minute.into_iter() {
        let vi = ValueItem::new(itr, 0..60);
        for i in vi.into_iter() {
            println!("\t{}", i);
        }
    }
    //for i in tmp {
    //    println!("\t{:?}", i);
    //}
    //assert_eq!(Local.ymd(1970, 02, 05).and_hms(00, 00, 00), 
    //           test.next_weekday_after_time(Local.ymd(1970, 02, 01).and_hms(00, 00, 00)));
}
