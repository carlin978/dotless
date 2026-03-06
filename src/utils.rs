use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn ls_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
	let mut files: Vec<PathBuf> = Vec::new();

	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				let mut ls = ls_dir(&path)?;

				files.append(&mut ls);
			} else {
				files.push(path);
			}
		}
	}

	Ok(files)
}
