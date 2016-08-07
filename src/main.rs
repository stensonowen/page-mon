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
mod action;
//pub mod croncfg;
//pub mod ast;

//use event::pushjet::{load_config, contact};
//use action::pushjet::{load_config, contact};
//use action::scrape::*;

extern crate hyper;
extern crate diff;

pub mod parse;
use parse::*;

fn main() {
    /*
    let (url, secret) = load_config().unwrap();
    let message = "this is a 'message test', with bells! and whistles?";
    let level = 3u8;
    let title = "title test";
    let link = "https://teamfortress.tv";
    
    let res = contact(url, secret.as_str(), message, title, level, link);
    println!("res: {:?}", res);
    */
    //let url = "https://en.wikipedia.org/wiki/Ancient_Rome";
    //let url = "https://api.twitch.tv/kraken/channels/b4nny";
    //let url = "http://icanhazip.com/";
    
    //let textA = "ab\ncde\nfg";
    //let textB = "ab\nZde\nfg";
    //println!("{:?}", diff::chars(textA, textB));
    //*
    /*
    let url = "https://news.ycombinator.com/item?id=12227922";
    //let url = "https://reddit.com";
    let url = hyper::Url::parse(url).unwrap();
    println!("Downloaded");
    let diff = compare(url);
    println!("{:?}", diff);
    */
    
}
