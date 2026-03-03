use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const ID_LENGTH: usize = 10;

mod file {
	use super::*;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct File {
		pub home_path: String,
		pub local_path: String,
		pub non_dotfile: bool,
	}
}

mod directory {
	use super::*;

	#[derive(Debug, Serialize, Deserialize)]
	pub enum Files {
		All,
		Whitelist(Vec<PathBuf>),
		Blacklist(Vec<PathBuf>),
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Directory {
		pub path: PathBuf,
		pub files: Files,
	}
}

mod template {
	use super::*;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Template {}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dotfile {
	File(file::File),
	Directory(directory::Directory),
	Template(template::Template),
}
impl Dotfile {
	pub fn new_file(home_path: String, local_path: String, non_dotfile: bool) -> Self {
		Self::File(file::File { home_path, local_path, non_dotfile })
	}
}

#[derive(Debug, Default)]
pub struct State(HashMap<nid::Nanoid<ID_LENGTH>, Dotfile>);
impl State {
	fn parse_state(state: &str) -> anyhow::Result<Self> {
		Ok(Self(ron::from_str(state)?))
	}

	fn serialize_state(&self) -> anyhow::Result<String> {
		ron::to_string(&self.0).context("Failed to serialize state")
	}

	pub fn write_state(self) -> anyhow::Result<()> {
		std::fs::write(crate::path::get_state_path(), self.serialize_state()?)?;

		Ok(())
	}

	pub fn add_dotfile(&mut self, dotfile: Dotfile) {
		self.0.insert(nid::Nanoid::<ID_LENGTH>::new(), dotfile);
	}

	pub fn update_dotfile(
		&mut self,
		id: nid::Nanoid<ID_LENGTH>,
		dotfile: Dotfile,
	) -> Option<Dotfile> {
		self.0.insert(id, dotfile)
	}

	pub fn remove_dotfile(&mut self, id: &nid::Nanoid<ID_LENGTH>) -> Option<Dotfile> {
		self.0.remove(id)
	}
}

pub fn read_state() -> anyhow::Result<State> {
	let state = std::fs::read_to_string(crate::path::get_state_path())?;

	State::parse_state(&state)
}
