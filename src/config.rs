use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

pub fn read_config() -> anyhow::Result<Option<Config>> {
	todo!()
}
