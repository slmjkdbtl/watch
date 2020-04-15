// wengwengweng

use crate::*;

pub struct Watcher {
	tasks: Vec<Task>,
}

impl Watcher {

	pub fn new() -> Self {
		return Self {
			tasks: vec![],
		};
	}

	pub fn from_watchfile(content: &str) -> Result<Self, String> {

		let rules = parser::parse_watchfile(content)
			.map_err(|_| format!("failed to parse Watchfile"))?;
		let mut watcher = Self::new();

		for r in rules {
			watcher.add_task(Task::new(&r.pat, &r.cmd)?);
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

