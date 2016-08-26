/*	Periodically crawl web pages and alert the user of changes 
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

/* BS:
 *  The biggest thing that a Command object needs to do is, given its 
 *  internal data and the current time, return the next time it should run.
 *  There seem to be two ways of approaching this.
 *  1. each vector of Values is consolidated into a hash set of valid times:
 *      this would be slightly costly in terms of initial construction and 
 *      memory usage, but fetching the 'next' valid value would be very fast.
 *      5 hash tables large enough to store `*****` is 263 bytes (134 u8s)
 *  2. fetch the 'next' valid value for each Value in the vector. that would
 *      require implementing 'next' for Asterisk, Range, Constant, and Skip.
 *      This would require less memory but maybe a little more cpu to find 
 *      'next' an arbitrary (though effectively like 1 or 2) number of times.
 *      It might be in our best interest to consolidate values also (e.g. 
 *      `2,1-3` and `1-3,3-4` are redundant), but I'm not sure how I'd do that.
 *  (1) would probably be simpler, cleaner, and less interesting. 
 *      In hindsight, (1) involved a lot of redundant checks, so it is 
 *      certainly not an optimal use of cpu. It also makes spaghetti code
 *      because the Gregorian calendar is awful. (2) is not an optimal use of 
 *      ram, but firing an event requires calling .contains() on a BTreeSet 
 *      of u8s instead of calling .next() on an arbitrary number of values.
 *  3. Populate (2)'s set of valid options and check once per minute. 
 *      Checks frequently, but there's not a lot there to mess up. 
 *      Negligibly more expensive than (2), and probably more reliable.
 *      It should probably have a test suite anyway, though smaller than what
 *      (2) would require (and WAY smaller than what (1) would require).
 */

/* mod.rs
 * This file will eventually house framework to tie in functionality for 
 *  Calendar and Action and maybe Schedule. It shouldn't be hefty.
 */


pub mod value_itr;
pub mod calendar;

