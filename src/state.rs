use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub fn read_state() -> anyhow::Result<State> {
	todo!()
}
