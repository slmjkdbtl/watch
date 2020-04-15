// wengwengweng

use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::collections::HashMap;
use std::process::Command;

pub struct Task {
	buffer: HashMap<PathBuf, Option<SystemTime>>,
	pat: String,
	cmd: String,
}

impl Task {

	pub fn new(pat: &str, cmd: &str) -> Result<Self, String> {

		return Ok(Self {
			buffer: HashMap::new(),
			pat: pat.to_string(),
			cmd: cmd.to_string(),
		});

	}

	pub fn tick(&mut self) -> Result<(), String> {

		let entries = glob::glob(&self.pat)
			.map_err(|_| format!("failed to parse pattern"))?
			.flatten();

		for path in entries {

			let modified = get_last_modified(&path);

			if let Some(last_modified) = self.buffer.get_mut(&path) {

				if &modified != last_modified {

					let mut env = HashMap::new();

					env.insert("FILE", format!("{}", path.display()));

					let mut cmd = self.cmd.clone();

					for (k, v) in env {
						cmd = cmd.replace(&format!("$({})", k), &v);
					}

					build_cmd(&cmd)
						.ok_or(format!("failed to build command"))?
						.spawn()
						.map_err(|_| format!("failed to run command"))?;

				}

				*last_modified = modified;

			} else {

				self.buffer.insert(path, modified);

			}

		}

		// TODO: remove buffered entry that doesn't exist on disk anymore

		return Ok(());

	}

}

fn get_last_modified(p: impl AsRef<Path>) -> Option<SystemTime> {
	return std::fs::metadata(p)
		.ok()
		.map(|d| d.modified().ok())
		.flatten();
}

fn build_cmd(cmd: &str) -> Option<Command> {

	let mut chunks = cmd.split_whitespace();

	if let Some(program) = chunks.next() {
		let mut cmd = Command::new(program);
		cmd.args(chunks);
		return Some(cmd);
	} else {
		return None;
	}

}

