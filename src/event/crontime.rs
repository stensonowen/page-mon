/*  Periodically crawl web pages and alert the user of changes
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
 *  More information in the enclosed `LICENSE' file
 */

#[derive(Debug)]
enum Char {
    //up to parser to determine whether these values valid in context
    Number(u8),     // 0-59, 0-23, 1-31, 1-12, 0-6
    Special(char),  // * , - ? L W #
    //Label(String),    //should be converted to numeric equivalent
}


#[derive(Debug)]
pub struct CronTime {
    minutes:        Char,
    hours:          Char,
    day_of_month:   Char,
    month:          Char,
    day_of_week:    Char,
}

impl CronTime {

    pub fn from_string(source: &str) -> Result<CronTime,&str> {
        let fields: Vec<&str> = source.split_whitespace().collect();
        if fields.len() != 5 {
            //TODO: use lifetimes to return a String.as_str to use format! to include fields.len
            return Err("Wrong number of fields");
        }

        let minutes = match (fields[0], fields[0].parse::<u8>()) {
            (_, Ok(n)) if n<60  => Char::Number(n),
            (_, Ok(n)) if n>59  => return Err("Minutes value out of range"),
            ("*",_)             => Char::Special('*'),
            //(",",_)             => Char::Special('*'),
            //("-",_)             => Char::Special('-'),
            _           => return Err("?"),
        };


        let minutes = match fields[0] {
            //Char::Number(_) => 1,
            "x" => 1,
            _ => 2,
        };

        let a:u8 = "4".parse::<u8>().unwrap();

        Err("???")

    }

}
