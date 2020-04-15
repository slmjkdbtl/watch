// wengwengweng

use std::thread;
use std::time::Duration;

use argh::FromArgs;

use watch::*;

#[derive(FromArgs)]
/// watch files and execute commands
struct Opt {
	#[argh(option, short = 't')]
	/// set check interval (ms)
	time: Option<u64>,
}

fn main() {

	let opt = argh::from_env::<Opt>();

	if let Ok(content) = std::fs::read_to_string("Watchfile") {

		let mut watcher = Watcher::from_watchfile(&content).unwrap();
		let time = opt.time.unwrap_or(100);

		loop {
			if let Err(e) = watcher.tick() {
				eprintln!("{}", e);
			}
			thread::sleep(Duration::from_millis(time));
		}

	} else {

		eprintln!("No Watchfile found!");

	}

}
