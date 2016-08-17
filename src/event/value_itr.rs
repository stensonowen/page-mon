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

//Stores the Iterable trait details on fields defined in ast.rs

use std::{ops,cmp};
//use ast::*;
//use super::super::parse::ast::*;
//use super::super::parse::ast::{Value, ContVal};
use super::job::{Value, ContVal};

pub type Range = ops::Range<u8>;

pub struct ValueItr<'a, 'b> {
    //stores a Value as well as data to iterate through valid numbers
    current:    Option<u8>,
    range:      &'a ops::Range<u8>,
    values:     &'b Value,
}

impl<'a, 'b> ValueItr<'a, 'b> {
    pub fn new(val: &'a mut Value, range: &'a Range) -> ValueItr<'a, 'a> {
        //get rid of 'b?
        ValueItr {
            current:    None,
            range:      range,
            values:     val,
        }
    }
}

impl<'a, 'b> Iterator for ValueItr<'a, 'b> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.current = match self.current {
            None    => Some(self.values.first(self.range.start)),
            Some(c) => self.values.next(c, self.range.end),
        };
        self.current
    }
}



pub trait Iterable {
    //functions that a Value will implement
    fn first(&self, min: u8) -> u8;
    //find the first valid option for Value. Always must be defined
    //`min` IS a valid option
    
    fn next(&self, current: u8, max: u8) -> Option<u8>;
    //find the next valid option for value after (excluding) `current`.
    //`current` is valid under `value`'s rules, and was the result of a 
    //  previous call to .next().
    //`max` IS NOT a valid option
}

impl Iterable for Value {
    fn first(&self, min: u8) -> u8 {
        match *self {
            Value::Constant(c)  => c,
            Value::CV(ref cv)   => cv.first(min),
            Value::Skip(ref cv, mult) => {
                let guess = cv.first(min);
                //find the next value above `guess` that's divisible by `mult`
                assert!(mult != 0);
                //formula subtracts 1, so it won't work w/ guess=0
                if guess == 0   { 0 } 
                else { ((guess-1)/mult+1)*mult }
            },
        }
    }
    fn next(&self, current: u8, max: u8) -> Option<u8> {
        //remember: `current` is valid
        match *self {
            Value::CV(ref cv)   => cv.next(current, max),
            Value::Constant(_)  => None,
            Value::Skip(ref cv, mult) => {
                let guess = current + mult;
                let upper_bound = match *cv {
                    ContVal::Asterisk   => max,
                    ContVal::Range(_,y) => cmp::min(max, y),
                };
                if guess < upper_bound { Some(guess) }
                else { None }
            },
        }
    }
}

impl Iterable for ContVal {
    fn first(&self, min: u8) -> u8 {
        match *self {
            ContVal::Asterisk => min,
            ContVal::Range(x,_) => cmp::max(x, min)
        }
    }
    fn next(&self, current: u8, max: u8) -> Option<u8> {
        //Given: `current` is valid
        //it will either be `Some(current+1)` or `None`
        let guess = current + 1;
        let upper_bound = match *self {
            ContVal::Asterisk   => max,
            ContVal::Range(_,y) => cmp::min(max, y),
        };
        if guess < upper_bound { Some(guess) } 
        else { None }
    }
}


