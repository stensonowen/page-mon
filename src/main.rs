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
#[allow(unused_imports)]
use event::HasNext;

pub mod croncfg;
pub mod ast;

extern crate chrono;
use chrono::{Local, TimeZone};

fn main() {

    let test1 = croncfg::parse_Command("0 0 29-31 * * https://test.com").unwrap().time;
    assert_eq!(Local.ymd(1970, 03, 29).and_hms(00, 00, 00), 
               test1.next_after_time(Local.ymd(1970, 02, 01).and_hms(00, 00, 00)));
}
