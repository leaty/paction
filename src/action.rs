use crate::criteria::Criteria;
use crate::process::Process;
use serde::Deserialize;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
pub struct Action {
	pub name: String,
	exec: Vec<String>,
	undo: Vec<String>,
	pub criteria: Vec<Criteria>,
	#[serde(skip)]
	pub done: bool,
}

impl Action {
	pub fn exec(&mut self) {
		self.cmd(&self.exec);
		self.done = true;
	}

	pub fn undo(&mut self) {
		self.cmd(&self.undo);
		self.done = false;
	}

	fn cmd(&self, cmdtext: &Vec<String>) {
		let _ = Command::new(&cmdtext[0])
			.args(&cmdtext[1..])
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn();
	}

	pub fn meets(&mut self, processes: &Vec<Process>) -> bool {
		let mut met = 0;
		for c in &mut self.criteria {
			for p in processes {
				if c.meets(&p.ruid, &p.name, &p.cmd) {
					met += 1;
					break;
				}
			}
		}

		met == self.criteria.len()
	}
}
