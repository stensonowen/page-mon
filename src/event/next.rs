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

//Alleviate some of the length and disorganization of event/mod.rs by 
// moving the `Next` details to its own file

//do I really have to use another crate just to be able to cast a generic?
extern crate num;
use self::num::ToPrimitive;
use std::{cmp, u8};

//Next: calling .next() on a Value will return a u8, from which
//it can be determined whether that Value overflowed.
//This state needs to be easily stored/compared.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Next {
    value:      u8, 
    overflow:   bool,
}

impl Next {
    pub fn new(v: u8, of: bool) -> Next {
        Next {
            value:      v,
            overflow:   of,
        }
    }
    pub fn worst() -> Next {
        Next {
            value:      u8::MAX,
            overflow:   true,
        }   
    }
    pub fn blank() -> Next {
        Next {
            value:      0,
            overflow:   false,
        }
    }
    //pub fn from_n<T: ToPrimitive>(n: T) -> Next {
    //    Next {
    //        value:      n.to_u8().unwrap(),
    //        overflow:   false,
    //    }
    //}
    pub fn overflowed(&self) -> bool {
        self.overflow
    }
    pub fn as_u32(&self) -> u32 {
        self.value as u32
    }
}

impl PartialOrd for Next {
    fn partial_cmp(&self, other:&Next) -> Option<cmp::Ordering> {
        if self == other {
            Some(cmp::Ordering::Equal)
        } else if self.overflow == false && other.overflow {
            Some(cmp::Ordering::Less)
        } else if self.overflow == other.overflow 
                && self.value < other.value {
            Some(cmp::Ordering::Less)
        } else {
            Some(cmp::Ordering::Greater)
        }
    }   
}
