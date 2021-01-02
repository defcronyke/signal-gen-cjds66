/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! Miscellaneous utility functions used by the library,
which make some common tasks more convenient to perform.
*/

use std::path::{Path, PathBuf};

/** Change the file extension at the end of a file path,
and return the whole new path.
*/
pub fn change_file_extension(path: &str, new_ext: &str) -> String {
	let mut new_path_obj = PathBuf::from(path);
	new_path_obj.pop();
	let basename = Path::new(path).file_stem().unwrap().to_str().unwrap();
	new_path_obj.push(format!("{}{}", basename, new_ext));
	new_path_obj.to_str().unwrap().to_string()
}
