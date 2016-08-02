/*  Periodically crawl web pages and alert the user of changes 
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
 *  More information in the enclosed `LICENSE' file
 */

//Send messages via pushjet using data from pushjet.json data

extern crate json;
extern crate hyper;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::io::Read;
//use hyper::*;
//use hyer::url;

const CONFIG_FILE: &'static str = "pushjet.json";
const DEFAULT_URL: &'static str = "https://api.pushjet.io";

fn find_file(name: &str) -> Option<PathBuf> {
    //check current directory for file
    //if absent, check previous directory for file
    //start with current working directory
    let mut path = env::current_dir().unwrap();
    loop {
        //for each parent directory, check if file is present
        let guess = path.join(name);
        //println!("\tguess = {:?}", guess);
        if guess.is_file() {
            //sought file is present
            return Some(guess);
        } else if path.pop() == false {
            //parent directory doesn't exist, so file isn't found
            return None;
        }
    }
}

pub fn load_config() -> Result<(hyper::Url,String),String> {
    //returns url and `secret`
    //parse file first
    //locate file in some parent/grandparent folder;
    let path = match find_file(CONFIG_FILE) {
        Some(p) => p,
        None    => return Err("No such file exists".to_string())
    };
    //open it
    let mut file = match File::open(path) {
        Ok(f)   => f,
        //Err(e)=>return Err(e.description().to_string().as_ref()) //TODO
        //do this with Result<_,&str> and lifetimes?
        Err(e)  => 
            return Err(format!("File input err: {}", e.description()))
    };
    // and read its contents;
    let mut text = String::new();
    if let Err(e) = file.read_to_string(&mut text) {
        return Err(format!("File read err: {}", e.description()))
    }
    //then parse the data in it.
    let data = match json::parse(&text) {
        Ok(d)   => d,
        Err(e)  => 
            return Err(format!("JSON parse err: {}", e.description()))
    };
    //`secret` must be present
    let secret = match data["secret"].as_str() { 
        Some(s) => s.to_string(),
        None => 
            return Err("Data err: missing field `secret`".to_string())
    };
    //`url` has a reasonable default 
    let address = match data["url"].as_str() {
        Some(u) => u,
        None    => DEFAULT_URL,
    };
    //make sure url is valid
    let url = match hyper::Url::parse(address) {
        Ok(u)   => u,
        Err(e)  => 
            return Err(format!("Url parse err: {}", e.description()))
    };

    Ok((url, secret))
}
