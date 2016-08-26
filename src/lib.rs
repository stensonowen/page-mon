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

/* NOTE about this file:
 *  test suite was initially made way at the beginning of this project,
 *  when I had a different approach in mind: I was trying to create an 
 *  actual schedule by enabling each `Time` struct to predict when it 
 *  would next run, which would have a theoretical performance benefit 
 *  over the alternative (currently in use), which just checks all of
 *  the jobs every minute. I put a lot of thought into abandoning that
 *  strategy after a week or two of working on it, mostly because it'd 
 *  be difficult to verify tasks would run on time with certainty, which
 *  is kind of important to this project. The test suite was initially
 *  designed to amend that, but there are just too many weird corner 
 *  cases (cough Pope Gregory XIII) for it to be comprehensive. This 
 *  file remains, and adds some possibly unnecessary certainty that 
 *  most everything in event/ *.rs is implemented correctly.
*/

//making these public removes some warnings about unused entities
#[allow(dead_code)]
mod parse;
#[allow(dead_code)]
mod event;

extern crate chrono;

#[cfg(test)]
mod tests {
    use super::parse::croncfg::{parse_Line, parse_Command};
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

    #[test]
    fn parse_lines() {
        //make sure parsing isn't broken
        assert!(parse_Line("* * * * * http://gnu.org").is_ok());
        assert!(parse_Line("1 2-9 */10 jan 1-9/3 https://gnu.org").is_ok());
        assert!(parse_Line("1/1 * * * * https://gnu.org").is_err());
        assert!(parse_Line("*****https://gnu.org -> text").is_ok());
        assert!(parse_Line("*****https://gnu.org -> EMail").is_ok());
        assert!(parse_Line("*****https://gnu.org -> foo").is_err());
        assert!(parse_Line(" # just a comment ").is_ok());
    }

    #[ignore]
    #[test]
    fn fire_now_weekday_0() {
        //fire every minute of every Sunday
        //check every day of Jan 1970 that it only runs on Sundays
        let mut cmd = parse_Command("* * * * 0 https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);
        assert!(cal.fire_now(&time) == false);
        assert!(cal.fire_now(&time.with_day(4).unwrap()) == true);
        for i in 1 .. 32 {
            let dt = time.with_day(i).unwrap();
            let is_sunday = dt.weekday().num_days_from_sunday() == 0;
            print_datetime(&dt, "weekday_0");
            assert!(cal.fire_now(&dt) == is_sunday);
        }
    }

    #[ignore]
    #[test]
    fn fire_now_date_0() {
        //fire every even date 
        //check every day of Jan 1970 that it only runs on even dates
        let mut cmd = parse_Command("* * */2 * * https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);
        for i in 1 .. 32 {
            let dt = time.with_day(i).unwrap();
            let is_even = dt.day()%2 == 0;
            print_datetime(&dt, "date_0");
            assert!(cal.fire_now(&dt) == is_even);
        }
    }

    #[ignore]
    #[test]
    fn fire_now_date_wd_0() {
        //fire on every Tue/Thu/Sat OR any dates with a 3 in them
        let mut cmd = parse_Command("* * 3,13,23,30-31 * 2-6/2
                                             https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1970, 1, 1).and_hms(0, 0, 0);
        for i in 1 .. 32 {
            let dt = time.with_day(i).unwrap();
            let dow_index = dt.weekday().num_days_from_sunday();
            let is_tu_th_sa = dow_index%2 == 0 && dow_index>0;   //even and not Sunday
            let has_a_3 = vec![3, 13, 23, 30, 31].contains(&i);
            print_datetime(&dt, "date_wd_0");
            assert!(cal.fire_now(&dt) == is_tu_th_sa || has_a_3);
        }
    }

    /* Doesn't make sense to test these here
     *  fire_now() doesn't adjust month lengths. if it's a valid DateTime, it 
     *  assumes it's valid. Changing that doesn't pose a notable benefit.
     *  In the future similar tests might be useful. But today is not that day.
    #[test]
    fn fire_now_feb_29_leap() {
        //it's hard to test invalid dates, because the passed argument is a DateTime
        //I think the only test to be done is feb 29 on a leap year
        let mut cmd = croncfg::parse_Command("* * * * * https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        let time = Local.ymd(1972, 2, 29).and_hms(0, 0, 0);
        assert!(cal.fire_now(time));
    }

    #[test]
    fn fire_at_vals_feb_29_common() {
        //now test feb 29 on a common year
        let mut cmd = croncfg::parse_Command("* * * * * https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        assert!(cal.fire_at_vals(0,0,29,2,0) == false);
    }
    */


    #[ignore]
    #[test]
    fn fire_at_vals_invalid_zeros() {
        //some fields have nonzero minimum values; test if they observe them
        //it's hard to test invalid dates, because the passed argument is a DateTime
        //I think the only test to be done is feb 29 on a leap year
        let mut cmd = parse_Command("* * * * * https://test.com").unwrap();
        let cal = Calendar::from_time(&mut cmd.time);
        //the date and month values cannot be zero; smallest values are `0 0 1 1 0`
        assert!(cal.fire_at_vals(0,0,1,1,0) == true);
        assert!(cal.fire_at_vals(0,0,0,1,0) == false);
        assert!(cal.fire_at_vals(0,0,1,0,0) == false);
    }

    //That's probably all the tests of fire_X for now. Minute and hour don't really
    //make sense to test, as they don't implement any functionality that date
    //doesn't extend upon, so if there's something wrong date would be wrong (I hope)
}
