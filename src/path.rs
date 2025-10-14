#[cfg(debug_assertions)]
///On debug mode treat the current directory as the home directory
fn home_dir() -> Option<PathBuf> {
	std::env::current_dir().ok()
}
#[cfg(not(debug_assertions))]
use std::env::home_dir;

use std::path::PathBuf;

///Gets the path to the dotfiles repository
pub fn get_repo_path() -> PathBuf {
	use super::DOTFILES_DIR;

	home_dir().expect("Home directory not found").join(DOTFILES_DIR)
}

///Gets the path to the config file in the repository
pub fn get_config_path() -> PathBuf {
	get_repo_path().join("dotless.toml")
}

///Expands path and checks if it is in the home directory, returns the canonical path and the
///stripped path if true, returns None otherwise
pub fn expand_path_if_in_home_dir(path: PathBuf) -> Option<(PathBuf, PathBuf)> {
	let home_dir = home_dir().expect("Home directory not found");

	let canonical_path = path.canonicalize().ok()?;

	canonical_path.starts_with(home_dir.clone()).then_some((
		canonical_path.clone(),
		canonical_path.strip_prefix(home_dir).ok()?.to_path_buf(),
	))
}
