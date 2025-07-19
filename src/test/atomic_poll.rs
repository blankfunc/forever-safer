use rand::Rng;
use rand::rng;
use std::time::{Duration, Instant};
use std::vec;
use super::ns::{display_ns, average_u128};
use super::image::{generate_image_file, generate_line_chart};
use crate::atomic_poll::AtomicPoll;
use ibig::UBig;
use dashmap::DashSet;

pub fn atomic_poll_test() -> String {
	let mut md = vec![];
	md.push(vec![
		"## AtomicPoll",
		"This benchmark test tests from three aspects: `self increasing time`, `usize exceeds repair time`, and `id recycling time`.",
		""
	].join("\n").to_string());

	{
		md.push("### Self Increasing".to_string());
		let mut counter = 10;
		for _ in 0..4 {
			let result = increase_test(counter);
			let (path, url) = generate_image_file(format!("normal_getting_time_{}.png", counter));
			generate_line_chart(&path, "Self Increasing", "Time", &result.1);

			md.append(&mut vec![
				format!("**{} Times.**", counter),
				"".to_string(),
				format!("Total time consumption: {}.", display_ns(result.0)),
				"".to_string(),
				format!("Average time consumption: {}.", display_ns(average_u128(result.1))),
				"".to_string(),
				format!("![Benchmark]({})", url),
				"".to_string()
			]);

			counter *= 10;
		}
	}

	{
		const TIMES: usize = 200;
		md.push("### USize Exceeds Repair Time".to_string());

		let mut results = vec![];
		for _ in 0..TIMES {
			let result = upgrade_test();
			results.push(result);
		}

		let average = average_u128(results.clone());
		let (path, url) = generate_image_file(format!("usize_exceeding_correction_time_{}.png", TIMES));
		generate_line_chart(&path, "USize Exceeds Repair", "Time", &results);
		md.append(&mut vec![
			format!("The average time spent in *{} tests* is: {}.", TIMES, display_ns(average)),
			"".to_string(),
			format!("![Benchmark]({})", url),
			"".to_string()
		]);
	}

	{
		println!("Generate `ID Recycling and Reuse Time`.");
		const TIMES: usize = 6;
		md.push("### ID Recycling and Reuse Time".to_string());

		let mut results1 = vec![];
		let mut results2 = vec![];
		let mut couters = vec![];

		let mut release_couter = 10;
		for _ in 0..TIMES {
			println!("For {}", release_couter);
			let (release_result, reuse_result) = reuse_test(release_couter);

			// For release
			let release_average = average_u128(release_result);
			results1.push(release_average);

			let reuse_average = average_u128(reuse_result);
			results2.push(reuse_average);

			couters.push(release_couter);
			release_couter *= 10;
		}

		let times_at = format!("Test at times {}.", couters.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));

		let (path, url) = generate_image_file("id_recycling_and_reuse_time_release.png".to_string());
		generate_line_chart(&path, "ID Recycling and Reuse (Release)", "Time", &results1);
		md.append(&mut vec![
			"**Child Benchmark** Release ID Time.\n".to_string(),
			times_at.clone(),
			"".to_string(),
			format!("![Benchmark]({})", url),
			"".to_string()
		]);

		let (path, url) = generate_image_file("id_recycling_and_reuse_time_reuse.png".to_string());
		generate_line_chart(&path, "ID Recycling and Reuse (Reuse)", "Time", &results2);
		md.append(&mut vec![
			"**Child Benchmark** Reuse ID Time.\n".to_string(),
			times_at.clone(),
			"".to_string(),
			format!("![Benchmark]({})", url),
			"".to_string()
		]);
	}

	return md.join("\n");
}

fn increase_test(counter: usize) -> (u128, Vec<u128>) {
	let poll = AtomicPoll::new();
	for _ in 0..2 {
		poll.get_and_increase();
	}

	let all_timer = Instant::now();
	let mut times: Vec<Duration> = vec![];
	for _ in 0..counter {
		let timer = Instant::now();
		poll.get_and_increase();
		times.push(timer.elapsed());
	}
	
	return (
		all_timer.elapsed().as_nanos(),
		times.iter().map(|time| time.as_nanos()).collect::<Vec<_>>()
	);
}

fn upgrade_test() -> u128 {
	let poll = AtomicPoll::new();
	poll.get_and_increase();
	let timer = Instant::now();
	poll.get_and_increase();
	return timer.elapsed().as_nanos();
}

fn reuse_test(release_couter: usize) -> (Vec<u128>, Vec<u128>) {
	let poll = AtomicPoll::new();

	// Random id list
	let mut tr = rng();
	// let idlist = (0..=(usize::MAX-2)).choose_multiple(&mut tr, release_couter);
	let list = DashSet::new();
	while list.len() < release_couter {
		list.insert(tr.random_range(0..usize::MAX-2));
	}
	let idlist = list.into_iter().collect::<Vec<_>>();

	let mut release_times = vec![];
	for id in idlist.clone() {
		let timer = Instant::now();
		poll.release(UBig::from(id));
		release_times.push(timer.elapsed().as_nanos());
	}

	let mut reuse_times = vec![];
	for _ in idlist.clone() {
		let timer = Instant::now();
		poll.get_and_increase();
		reuse_times.push(timer.elapsed().as_nanos());
	}
	
	return (release_times, reuse_times);
}