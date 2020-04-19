// wengwengweng

use std::fs;
use std::path::PathBuf;
use std::path::Path;

use crate::*;

pub struct Watcher {
	tasks: Vec<Task>,
	path: PathBuf,
}

impl Watcher {

	pub fn new(p: impl AsRef<Path>) -> Self {
		return Self {
			tasks: vec![],
			path: p.as_ref().to_path_buf(),
		};
	}

	pub fn from_watchfile(path: impl AsRef<Path>) -> Result<Self, String> {

		let fpath = path.as_ref();
		let path = fpath.parent().ok_or(format!("invalid path {}", fpath.display()))?;
		let content = fs::read_to_string(&fpath)
			.map_err(|_| format!("failed to read {}", fpath.display()))?;
		let rules = parser::parse_watchfile(&content)
			.map_err(|_| format!("failed to parse {}", fpath.display()))?;
		let mut watcher = Self::new(&path);

		for r in rules {
			watcher.add_task(Task::new(&r.pat, &r.cmd, &path)?);
		}

		return Ok(watcher);

	}

	pub fn add_task(&mut self, t: Task) {
		self.tasks.push(t);
	}

	pub fn tick(&mut self) -> Result<(), String> {
		for t in &mut self.tasks {
			t.tick()?;
		}
		return Ok(());
	}

}

