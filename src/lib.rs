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
    use chrono::{Local, TimeZone, Datelike, Timelike, DateTime};
    use event::calendar::Calendar;

    fn print_datetime(dt: &DateTime<Local>, fn_name: &str) {
        //printing is used to see additional debugging info while spamming assert!()
        //see println calls using `cargo test -- --nocapture`
        //more readable than println!("{}", dt);
        //actually, tests run in parallel, so that makes things tricky
        println!("\t{:16}\t{:?},   {:4} / {:2} / {:2} :  {:02}:{:02}", fn_name,
                 dt.weekday(),  dt.year(),  dt.month(), 
                 dt.day(),      dt.hour(),  dt.minute());
    }

    #[ignore]
    #[test]
    fn fire_now_weekday_0() {
        //fire every minute of every Sunday
        //check every day of Jan 1970 that it only runs on Sundays
        let mut cmd = croncfg::parse_Command("* * * * 0 https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);
        assert!(cal.fire_now(time) == false);
        assert!(cal.fire_now(time.with_day(4).unwrap()) == true);
        for i in 1 .. 32 {
            let dt = time.with_day(i).unwrap();
            let is_sunday = dt.weekday().num_days_from_sunday() == 0;
            print_datetime(&dt, "weekday_0");
            assert!(cal.fire_now(dt) == is_sunday);
        }
    }

    #[ignore]
    #[test]
    fn fire_now_date_0() {
        //fire every even date 
        //check every day of Jan 1970 that it only runs on even dates
        let mut cmd = croncfg::parse_Command("* * */2 * * https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);
        for i in 1 .. 32 {
            let dt = time.with_day(i).unwrap();
            let is_even = dt.day()%2 == 0;
            print_datetime(&dt, "date_0");
            assert!(cal.fire_now(dt) == is_even);
        }
    }

    #[ignore]
    #[test]
    fn fire_now_date_wd_0() {
        //fire on every Tue/Thu/Sat OR any dates with a 3 in them
        let mut cmd = croncfg::parse_Command("* * 3,13,23,30-31 * 2-6/2
                                             https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);
        for i in 1 .. 32 {
            let dt = time.with_day(i).unwrap();
            let dow_index = dt.weekday().num_days_from_sunday();
            let is_tu_th_sa = dow_index%2 == 0 && dow_index>0;   //even and not Sunday
            let has_a_3 = vec![3, 13, 23, 30, 31].contains(&i);
            print_datetime(&dt, "date_wd_0");
            assert!(cal.fire_now(dt) == is_tu_th_sa || has_a_3);
        }
    }
}
