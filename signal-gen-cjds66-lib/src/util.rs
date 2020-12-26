use std::path::{Path, PathBuf};

// Change the file extension at the end of a file path,
// and return the whole new path.
pub fn change_file_extension(path: &str, new_ext: &str) -> String {
	let mut new_path_obj = PathBuf::from(path);
	new_path_obj.pop();
	let basename = Path::new(path).file_stem().unwrap().to_str().unwrap();
	new_path_obj.push(format!("{}{}", basename, new_ext));
	new_path_obj.to_str().unwrap().to_string()
}
