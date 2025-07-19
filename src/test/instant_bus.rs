use std::{sync::Arc, time::Instant};
use crossbeam::queue;

use crate::instant_bus::InstantBus;
use super::ns::{display_ns, average_u128};
use super::image::{generate_image_file, generate_dot_series_chart};

pub fn instant_bus_test() -> String {
	let mut md: Vec<String> = vec![];
	md.push(vec![
		"## InstantBus",
		"This benchmark test will test the reception delay of different numbers of receivers (core).",
		""
	].join("\n").to_string());

	for counter in vec![1, 32, 64, 256, 1024] {
		let (average, url) = once_test(counter);
		md.append(&mut vec![
			format!("### {} Cores / Receivers", counter),
			format!("Average time consumption: {}.", display_ns(average)),
			"".to_string(),
			format!("![Benchmark]({})", url)
		]);
	}

	return md.join("\n");
}

fn once_test(test_counter: usize) -> (u128, String) {
	let bus = Arc::new(InstantBus::<Instant>::new());

	let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
	let timers = Arc::new(queue::SegQueue::<Instant>::new());
	for _ in 0..test_counter {
		let timers_clone = timers.clone();
		let counter_clone = counter.clone();
		let mut bus_clone = bus.clone().subscribe();
		std::thread::spawn(move || {
			counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
			let _ = bus_clone.recv();
			let start = Instant::now();
			bus_clone.recv();
			timers_clone.push(start);
		});
	}
	
	loop {
		if counter.load(std::sync::atomic::Ordering::SeqCst) == test_counter {
			break;
		}
	}

	bus.send(Instant::now());

	let timer = Instant::now();
	bus.send(timer);

	loop {
		if timers.len() == test_counter {
			break;
		}
	}
	
	let mut times: Vec<u128> = vec![];
	while !timers.is_empty() {
		times.push((timer - timers.pop().unwrap()).as_nanos());
	}

	let (path, url) = generate_image_file(format!("instant_bus_{}.png", test_counter));
	generate_dot_series_chart(&path, "InstantBus Delay", "Core", "Delay", &times);
	return (average_u128(times), url);
}