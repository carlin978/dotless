use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait SerializeState {
	fn serialize_state(&self) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dotfile {
	File(File),
	Directory(Directory),
	Template(Template),
}

pub type State = HashMap<String, Dotfile>;

impl SerializeState for State {
	fn serialize_state(&self) -> Option<String> {
		ron::to_string(self).ok()
	}
}

pub fn read_state() -> anyhow::Result<State> {
	todo!()
}
