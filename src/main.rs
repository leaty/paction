use std::collections::HashSet;
use std::{thread, time};
use std::error::Error;
use std::process::Command;
use serde::Deserialize;
use git_version::git_version;

struct Process {
	ruid: u32,
	name: String,
	cmd: String,
}

#[derive(Deserialize)]
struct Config {
	action: Vec<Action>,
	tick: u64,
}

impl Config {
	fn setup(&mut self) {
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

#[derive(Deserialize)]
struct Action {
	name: String,
	exec: Vec<String>,
	undo: Vec<String>,
	criteria: Vec<Criteria>,
	#[serde(skip)]
	done: bool,
}

impl Action {
	fn exec(&mut self) {
		self.cmd(&self.exec);
		self.done = true;
	}

	fn undo(&mut self) {
		self.cmd(&self.undo);
		self.done = false;
	}

	fn cmd(&self, cmdtext: &Vec<String>) {
		let _ = Command::new(&cmdtext[0])
			.args(&cmdtext[1..])
			.spawn();
	}

	fn meets(&mut self, processes: &Vec<Process>) -> bool {
		let mut met = 0;
		for r in &mut self.criteria {
			for p in processes {
				if r.meets(&p.ruid, &p.name, &p.cmd) {
					met += 1;
				}
			}
		}

		met == self.criteria.len()
	}
}

#[derive(Deserialize)]
struct Criteria {
	user: Vec<toml::Value>,
	#[serde(skip)]
	userid: HashSet<u32>,
	name: HashSet<String>,
	cmd: HashSet<String>,
}

impl Criteria {
	fn meets(&self, ruid: &u32, name: &String, cmd: &String) -> bool { 
		if (!self.userid.is_empty() && !self.userid.contains(ruid))
			|| (!self.name.is_empty() && !self.name.contains(name)) {
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

fn main() -> Result<(), Box<dyn Error>> {
	// Cmdline args
	let version = git_version!();
	let args = clap::App::new("paction")
		.version(version)
		.author("leaty <dev@leaty.net>")
		.about("Execute actions relative to process existance.")
		.arg(clap::Arg::with_name("config")
			.short("c")
			.long("config")
			.value_name("FILE")
			.help("Config file"))
		.get_matches();

	// Load config file
	let xdgd = xdg::BaseDirectories::with_prefix("paction").unwrap();
	let xdgc = xdgd.place_config_file("config.toml").unwrap();
	let default = xdgc.to_str().unwrap();
	let config_file = args.value_of("config").unwrap_or(&default);
	let config_str = std::fs::read_to_string(&config_file)?;
	let mut config: Config = toml::from_str(&config_str).unwrap();
	config.setup();

	let tick = time::Duration::from_millis(config.tick);

	// Service
	loop {
		let procs = get_processes();

		// Go through each action and check if it should execute
		for action in &mut config.action {
			if action.meets(&procs) {
				if !action.done {
					println!("EXEC - {}", action.name);
					action.exec();
				}
			}
			else if action.done {
				println!("UNDO - {}", action.name);
				action.undo();
			}
		}

		thread::sleep(tick);
	}
}

/// Returns all processes from procfs as `Vec<Process>`
fn get_processes() -> Vec<Process> {
	let procs = procfs::process::all_processes().unwrap();
	let mut processes = vec![];

	for p in procs {
		let status = match p.status() {
			Ok(v) => v,
			Err(_e) => continue,
		};
		let cmd = match p.cmdline() {
			Ok(v) => v.join(" "),
			Err(_e) => continue,
		};

		processes.push(Process{ruid: status.ruid, name: status.name, cmd});
	}

	processes
}

fn parse_user(user: String) -> u32 {
	match user.as_str() {
		"$USER" => users::get_current_uid(),
		_ => users::get_user_by_name(&user).unwrap().uid()
	}
}
