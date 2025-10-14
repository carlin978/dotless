use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

pub fn read_config() -> anyhow::Result<Option<Config>> {
	todo!()
}

#[cfg(test)]
mod test {
	#[test]
	fn parses_default_config_asset() {
		toml::from_str::<super::Config>(include_str!("../assets/config.toml")).unwrap();
	}
}
