// wengwengweng

use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::collections::HashMap;
use std::process::Command;

use super::term::style as s;

pub struct Task {
	states: HashMap<PathBuf, Option<SystemTime>>,
	pat: String,
	cmd: String,
	path: PathBuf,
}

impl Task {

	pub fn new(pat: &str, cmd: &str, path: impl AsRef<Path>) -> Result<Self, String> {

		return Ok(Self {
			states: HashMap::new(),
			pat: pat.to_string(),
			cmd: cmd.to_string(),
			path: path.as_ref().to_path_buf(),
		});

	}

	pub fn tick(&mut self) -> Result<(), String> {

		let entries = glob::glob(&format!("{}", self.path.join(&self.pat).display()))
			.map_err(|_| format!("failed to parse pattern"))?
			.flatten();

		for path in entries {

			let modified = get_last_modified(&path);

			if let Some(last_modified) = self.states.get_mut(&path) {

				if &modified != last_modified {

					let cpath = format!("{}", path.display()).replace(&format!("{}", self.path.display()), "");

					println!(
						"{}\n-> {}",
						s(&cpath).yellow().bold(),
						s(&self.cmd).blue()
					);

					let mut env = HashMap::new();
					let mut cmd = self.cmd.clone();

					env.insert("FILE", format!("{}", path.display()));

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

				self.states.insert(path, modified);

			}

		}

		// TODO: remove saved entry that doesn't exist on disk anymore

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

