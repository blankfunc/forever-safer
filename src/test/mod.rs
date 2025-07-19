#![cfg(test)]
use once_cell::sync::Lazy;
// use ibig::{ubig, UBig};
use std::{env, fs::OpenOptions, io::Write};

// pub static ONE_UBIG: Lazy<UBig> = Lazy::new(|| ubig!(1));

pub static SUMMARY: Lazy<Option<String>> = Lazy::new(|| env::var("GITHUB_STEP_SUMMARY").ok());
pub const GITHUB_PREFIX: &'static str = "https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images";

mod image;
mod ns;

mod atomic_poll;
mod instant_bus;

use atomic_poll::atomic_poll_test;
use instant_bus::instant_bus_test;

#[test]
fn all_test() {
	let content = vec![
		atomic_poll_test(),
		"".to_string(),
		instant_bus_test()
	].join("\n");

	let path = SUMMARY.clone().unwrap_or("test/README.md".to_string());

	println!("Write to {}", path);

	let mut file = OpenOptions::new().write(true).append(false).open(path).unwrap();
	writeln!(file, "{}", content).unwrap();

	// fs::write("README.md", content).unwrap();
}