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

fn main() {
    //let _e: event::Event;

    //event::foo();

    //assert!(croncfg::parse_Line("*").is_ok());
    //assert!(croncfg::parse_Line("42").is_ok());
    //assert!(croncfg::parse_Line("1-2").is_ok());
    //assert!(croncfg::parse_Line("1,2").is_ok());
    //assert!(croncfg::parse_Line("1,2,3-4,5,6-7").is_ok());
    //assert!(croncfg::parse_Line("1,").is_err());
    //assert!(croncfg::parse_Line("*/5").is_ok());
    //assert!(croncfg::parse_Line("#5").is_ok());
    //assert!(croncfg::parse_Line("* *").is_ok());
    //
    assert!(croncfg::parse_Line("* * * * * http://www.google.com").is_ok());
    assert!(croncfg::parse_Line("1 2 3 4 5 https://google.com").is_ok());
    println!("{:?}", croncfg::parse_Line("1 2 3 4 5 https://google.com").unwrap());
    println!("{:?}", croncfg::parse_Line("1*2\t4*/4https://bing.com").unwrap());
    println!("{:?}", croncfg::parse_Line("*****https://reddit.com").unwrap());
    println!("{:?}", croncfg::parse_Line("@yearly https://ddg.co").unwrap());
    println!("{:?}", croncfg::parse_Line("* * * JAN SUN https://teamfortresstv.com").unwrap());
    println!("{:?}", croncfg::parse_Line("* * * MaR fRi https://teamfortresstv.com").unwrap());
}
