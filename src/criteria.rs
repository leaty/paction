use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
pub struct Criteria {
	pub user: Vec<toml::Value>,
	#[serde(skip)]
	pub userid: HashSet<u32>,
	name: HashSet<String>,
	pub cmd: HashSet<String>,
}

impl Criteria {
	pub fn meets(&self, ruid: &u32, name: &String, cmd: &String) -> bool {
		if (!self.userid.is_empty() && !self.userid.contains(ruid))
			|| (!self.name.is_empty() && !self.name.contains(name))
		{
			return false;
		}

		if !self.cmd.is_empty() {
			for cmdl in &self.cmd {
				if cmd.contains(cmdl) {
					return true;
				}
			}

			return false;
		}

		return true;
	}
}
