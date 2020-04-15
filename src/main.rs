// wengwengweng

use std::thread;
use std::time::Duration;

use argh::FromArgs;

use watch::*;

#[derive(FromArgs)]
/// pack a binary to a MacOS .app bundle
struct Opt {
	#[argh(option, short = 't')]
	/// check interval in ms
	time: Option<u64>,
}

fn main() {

	if let Ok(content) = std::fs::read_to_string("Watchfile") {

		let opt = argh::from_env::<Opt>();
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
