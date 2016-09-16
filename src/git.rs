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

/* Note: be careful of using this if you forked this code (or if you're me), 
 * because it might screw with your .git repo
 */

extern crate git2;
use std::path::Path;
use std::str;

pub struct Repo {
    repo: git2::Repository,
}


impl Repo {
    pub fn open(path: &str) -> Result<Repo, git2::Error> {
		//creates repo if it didn't exist
		let loc = Path::new(path).join(".git");
		let exists = loc.is_dir();
		if exists {
			let r = try!(git2::Repository::open(path));
            Ok(Repo{ repo: r})
		} else {
			let r = try!(git2::Repository::init(path));
            let mut repo = Repo{ repo: r };
            try!(repo.commit_orphan("first commit", None));
            Ok(repo)
		}
	}
    pub fn get_oid(&self) -> Result<git2::Oid, git2::Error> {
        self.repo.refname_to_id("HEAD")
    }
	pub fn add_file(&self, file: &str) -> Result<(), git2::Error> {
		let mut index = try!(self.repo.index());
		try!(index.add_path(Path::new(file)));
		index.write()
	}
	pub fn commit(&self, msg: &str, parent: git2::Oid) -> Result<git2::Oid, git2::Error> {
        //commit, but don't allow parent=None
        //parent=None should only be viable for the first commit in `open`
        self.commit_orphan(msg, Some(parent))
    }
	fn commit_orphan(&self, msg: &str, parent: Option<git2::Oid>) 
                    -> Result<git2::Oid, git2::Error> {
        //commit, with or without a parent
		let sig = try!(self.repo.signature());
		let tree_id = {
			let mut index = try!(self.repo.index());
			try!(index.write_tree())
		};
		let tree = try!(self.repo.find_tree(tree_id));

        //commit with or without a parent
        if let Some(p) = parent {
            let last = try!(self.repo.find_commit(p));
            self.repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&last])
        } else {
            self.repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[])
        }
	}
	pub fn diff(&self, oid1: git2::Oid, oid2: git2::Oid) -> Result<String, git2::Error> {
		//diff the trees
		let obj1 = try!(self.repo.find_object(oid1, None));
		let obj2 = try!(self.repo.find_object(oid2, None));
		let tree1 = try!(obj1.peel(git2::ObjectType::Tree));
		let tree2 = try!(obj2.peel(git2::ObjectType::Tree));
		let diff = try!(self.repo.diff_tree_to_tree(tree1.as_tree(), tree2.as_tree(), None));

		//start adding info to a String
		let mut output = String::new();
		//retrieve `stats`
		let stats = try!(diff.stats());
		let format = git2::DIFF_STATS_SHORT;
		let stats_buf = try!(stats.to_buf(format, 256));
		output.push_str(str::from_utf8(&*stats_buf).unwrap());
		//retrieve line-by-line differences
		try!(diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
			match line.origin() {
				'+' | '-' | ' ' => output.push(line.origin()),
				_               => {},
			}
			output.push_str(str::from_utf8(line.content()).unwrap());
			true
		}));
		Ok(output)
	}
}

