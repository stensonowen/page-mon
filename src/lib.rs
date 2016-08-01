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

mod ast;
pub mod croncfg;
pub mod event;
extern crate chrono;

#[cfg(test)]
mod tests {
    use super::croncfg;
    use chrono::Local;
    use event::calendar::Calendar;

    #[test]
    fn fire_now0() {
        let mut tmp = croncfg::parse_Command("* * * * 0 https://test.com").unwrap();
        let cal = Calendar::from_time(&mut tmp.time);
        assert!(false == cal.fire_now(Local::now()));
    }
}
