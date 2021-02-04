pub struct Process {
	pub ruid: u32,
	pub name: String,
	pub cmd: String,
}

/// Returns all processes from procfs as `Vec<Process>`
pub fn get_list() -> Vec<Process> {
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

		processes.push(Process {
			ruid: status.ruid,
			name: status.name,
			cmd,
		});
	}

	processes
}
