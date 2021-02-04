use crate::action::Action;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pub action: Vec<Action>,
	pub tick: u64,
}

impl Config {
	pub fn setup(&mut self) {
		for action in &mut self.action {
			for criteria in &mut action.criteria {
				// Convert user to userid
				for user in &mut criteria.user {
					criteria.userid.insert(match user {
						toml::Value::Integer(id) => *id as u32,
						toml::Value::String(name) => parse_user(name.to_owned()),
						_ => continue,
					});
				}
			}
		}
	}
}

fn parse_user(user: String) -> u32 {
	match user.as_str() {
		"$USER" => users::get_current_uid(),
		_ => users::get_user_by_name(&user).unwrap().uid(),
	}
}
