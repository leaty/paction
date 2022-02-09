mod action;
mod config;
mod criteria;
mod process;
use config::Config;
use git_version::git_version;
use std::error::Error;
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {
	// Cmdline args
	let version = git_version!();
	let args = clap::App::new("paction")
		.version(version)
		.author("leaty <dev@leaty.net>")
		.about("Execute actions relative to process existance.")
		.arg(
			clap::Arg::with_name("config")
				.short("c")
				.long("config")
				.value_name("FILE")
				.help("Config file"),
		)
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
		let procs = process::get_list();

		// Go through each action and check if it should execute
		for action in &mut config.action {
			if action.meets(&procs) {
				if !action.done {
					println!("EXEC - {}", action.name);
					action.exec();
				}
			} else if action.done {
				println!("UNDO - {}", action.name);
				action.undo();
			}
		}

		thread::sleep(tick);
	}
}
