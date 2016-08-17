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

//Parse all of the data from the input file more comprehensively than
// ast.rs can on its own

#[allow(dead_code)]
pub mod croncfg;
#[allow(dead_code)]
pub mod ast;

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

<<<<<<< HEAD
=======
const DEFAULT_PJURL: &'static str = "https://api.pushjet.io";

>>>>>>> parent of a37a8d7... So now everything compiles. But reorganizing might be in order
#[derive(PartialEq, Hash, Eq)]
pub enum Var {
    EmailDomain,        //necessary for email
    EmailSecret,        //necessary for email
    EmailRecip,         //nec for email
    PushjetUrl,         //optional  for pushjet
    PushjetSecret,      //necessary for pushjet
    Dir,                //necessary
}

pub type Vars = HashMap<Var,String>;


//pub fn parse(input: String) {
pub fn parse(input: &Path) -> Result<(Vec<ast::Command>,Vars),Vec<String>> {
    //all goes well, return a tuple of all commands and all variables
    //if there are parsing errors, return a vector of of them
    let file = match File::open(input) {
        Ok(f)  => f,
        Err(e) => return Err(vec![format!("Failed to open config file for parsing: {}",
                                          e.description()).to_string()]),
    };
    //keep track of lines:
    let mut variables = Vars::new();
    let mut commands  = Vec::<ast::Command>::new();
    let mut errors    = Vec::<String>::new();
    //iterate through config by newline
    let reader = BufReader::new(file);
    for line in reader.lines() {
        //unwrap line
        let buffer = match line {
            Ok(l)  => l,
            Err(e) => {
                let err = format!("Failed to read line from config: {}", e.description());
                errors.push(err);
                continue
            }
        };
        //skip if there's nothing on the line
        //what is the better way to do this in the grammar?
        if buffer.trim_right().len() == 0 {
            continue
        }
        //parse line
        let parsed = match croncfg::parse_Line(&buffer) {
            Ok(l)  => l,
            Err(e) => {
                let err = format!("Failed to parse line `{}` from config: {:?}", buffer, e);
                errors.push(err);
                continue
            }
        };
        //organize result
        match parsed {
            ast::Line::Cmd(cmd) => commands.push(cmd),
            ast::Line::VarSet(v)=> {
                let pair = match v {
                    ast::Var::EmailDomain(u) => (Var::EmailDomain, u),
                    ast::Var::EmailSecret(u) => (Var::EmailSecret, u),
<<<<<<< HEAD
                    ast::Var::EmailRecip(u)  => (Var::EmailRecip, u),
=======
                    ast::Var::EmailRecip(u) => (Var::EmailRecip, u),
>>>>>>> parent of a37a8d7... So now everything compiles. But reorganizing might be in order
                    ast::Var::PjSecret(u) => (Var::PushjetSecret, u),
                    ast::Var::PjUrl(u)    => (Var::PushjetUrl, u),
                    ast::Var::DataDir(u)  => (Var::Dir, u),
                };
                variables.insert(pair.0, pair.1);
            },
            ast::Line::Comment  => (),
        };
    }
<<<<<<< HEAD
=======
    insert_default_variable_values(&mut variables);
    if let Err(e) = verify(&variables) {
        errors.push(e);
    }
>>>>>>> parent of a37a8d7... So now everything compiles. But reorganizing might be in order
    //return errors or content
    if errors.is_empty() {
        Ok((commands, variables))
    } else {
        Err(errors)
    }
}

<<<<<<< HEAD
=======

fn insert_default_variable_values(vars: &mut Vars) {
    //fill in default value(s?)
    if vars.contains_key(&Var::PushjetUrl) == false {
        vars.insert(Var::PushjetUrl, DEFAULT_PJURL.to_string());
    }
}

fn verify(vars: &Vars) -> Result<(),String> {
    if vars.contains_key(&Var::Dir) == false {
        Err("No `DIR` variable set".to_string())
    }
    else {
        Ok(())
    }
}

>>>>>>> parent of a37a8d7... So now everything compiles. But reorganizing might be in order
