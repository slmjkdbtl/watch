// wengwengweng

mod term;

use std::fs;
use std::thread;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use argh::FromArgs;

use watch::*;
use term::style as s;

fn rec_find(path: impl AsRef<Path>, fname: &str) -> Option<PathBuf> {

	let path = path.as_ref();
	let file = path.join(fname);

	if file.exists() {
		return Some(file.to_path_buf());
	} else {
		if let Some(p) = path.parent() {
			return rec_find(p, fname);
		} else {
			return None;
		}
	}

}

#[derive(FromArgs)]
/// watch files and execute commands
struct Opt {
	#[argh(option, short = 't')]
	/// set check interval (ms)
	time: Option<u64>,
}

fn run(opt: &Opt) -> Result<(), String> {

	let cur_dir = std::env::current_dir()
		.map_err(|_| format!("failed to get current dir"))?;
	let path = rec_find(cur_dir, "Watchfile")
		.ok_or(format!("no Watchfile found"))?;

	let mut watcher = Watcher::from_watchfile(&path).unwrap();
	let time = opt.time.unwrap_or(100);

	println!("{}", s("watch started...").green());

	loop {
		watcher.tick()?;
		thread::sleep(Duration::from_millis(time));
	}

}

fn main() {

	let opt = argh::from_env::<Opt>();

	if let Err(e) = run(&opt) {
		eprintln!("{}", s(&e).red());
	}

}
