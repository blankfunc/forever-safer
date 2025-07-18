#![cfg(test)]
use std::{env, fs::{self, OpenOptions}, io::Write, sync::Arc, time::{Duration, Instant}};
use crossbeam::queue;
use crate::{atomic_poll::AtomicPoll, instant_bus::InstantBus};
use plotters::prelude::*;
use ibig::{ubig, UBig};
use once_cell::sync::Lazy;

static ONE_UBIG: Lazy<UBig> = Lazy::new(|| ubig!(1));

#[test]
fn all_test() {
	let content = vec![
		atomic_poll_test(),
		"".to_string(),
		instant_bus_test(),
	].join("\n");
	if let Ok(path) = env::var("GITHUB_STEP_SUMMARY") {
		println!("Write to {}", path);
		let mut file = OpenOptions::new().write(true).append(false).open(path).unwrap();
		writeln!(file, "{}", content).unwrap();
	}

	fs::write("README.md", content).unwrap();
}

fn generate_line_chart(filepath: &str, title: &str, label: &str, data: &[u128]) -> String {
    let width: u32 = 800;
    let height: u32 = 600;
    {
        // let root =
		// 	BitMapBackend::with_buffer(&mut buffer, (width as u32, height as u32))
        //     .into_drawing_area();
		let root = BitMapBackend::new(filepath, (width, height)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let max_x = data.len();
        let max_y = *data.iter().max().unwrap_or(&1);

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(0..max_x, 0u128..max_y)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                data.iter().enumerate().map(|(x, y)| (x, *y)),
                &RED,
            ))
            .unwrap()
            .label(label)
            .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

        chart.configure_series_labels().draw().unwrap();
        root.present().unwrap();
    }

	return format!("https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/{}", filepath.split("/").last().unwrap());
}

fn atomic_poll_test() -> String {
	let mut md: Vec<String> = vec![];
	md.push(vec![
		"## AtomicPoll",
		"This benchmark test tests from three aspects: `normal getting time`, `usize exceeding correction time`, and `id recycling and reuse time`.",
		""
	].join("\n").to_string());

	{
		md.push("### Normal Getting Time".to_string());
		for i in vec![100, 1000, 10000, 100000] {
			let result = atomic_poll_increase_test(i);
			let filepath = format!("test/normal_getting_time_{}.png", i);
			md.append(&mut vec![
				format!("**{} Times.**", i),
				"".to_string(),
				format!("Total time consumption: {}ns ({}ms).", result.0, result.0 / 1000_000),
				"".to_string(),
				format!("![Benchmark]({})", generate_line_chart(
					&filepath,
					"AtomicPoll Getting Test",
					"Time",
					&result.1
				)),
				"".to_string()
			]);
		}
	}

	{
		md.push("### USize Exceeding Correction Time".to_string());
		let times = 200;
		let mut results = vec![];
		for _ in 0..times {
			let result = atomic_poll_upgrade_test();
			results.push(result);
		}

		let total = results.iter().sum::<u128>();
		let filepath = format!("test/usize_exceeding_correction_time_{}.png", times);
		md.append(&mut vec![
			format!("The average time spent in **{} tests** is: {}ns ({}ms).", times, total, total / 1000_000),
			"".to_string(),
			format!("![Benchmark]({})", generate_line_chart(&filepath, "USize Exceeding Correction Test", "Time", &results)),
			"".to_string()
		]);
	}

	{
		md.push("### ID Recycling and Reuse Time".to_string());
		let times = 200;
		let mut results1 = vec![];
		let mut results2 = vec![];
		for _ in 0..times {
			let result = atomic_poll_reuse_test();
			results1.push(result.0);
			results2.push(result.1);
		}

		let total1 = results1.iter().sum::<u128>();
		let filepath = format!("test/id_recycling_and_reuse_time_release_{}.png", times);
		md.append(&mut vec![
			"**Child Benchmark** Release ID Time.\n".to_string(),
			format!("The average time spent in **{} tests** is: {}ns ({}ms).", times, total1, total1 / 1000_000),
			"".to_string(),
			format!("![Benchmark]({})", generate_line_chart(&filepath, "ID Recycling and Reuse Test (Release)", "Time", &results1)),
			"".to_string()
		]);

		let total2 = results2.iter().sum::<u128>();
		let filepath = format!("test/id_recycling_and_reuse_time_reuse_{}.png", times);
		md.append(&mut vec![
			"**Child Benchmark** Reuse ID Time.\n".to_string(),
			format!("The average time spent in **{} tests** is: {}ns ({}ms).", times, total2, total2 / 1000_000),
			"".to_string(),
			format!("![Benchmark]({})", generate_line_chart(&filepath, "ID Recycling and Reuse Test (Reuse)", "Time", &results2)),
			"".to_string()
		]);
	}

	return md.join("\n");
}

fn atomic_poll_increase_test(counter: usize) -> (u128, Vec<u128>) {
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

fn atomic_poll_upgrade_test() -> u128 {
	let poll = AtomicPoll::new();
	poll.get_and_increase();
	let timer = Instant::now();
	poll.get_and_increase();
	return timer.elapsed().as_nanos();
}

fn atomic_poll_reuse_test() -> (u128, u128) {
	let poll = AtomicPoll::new();
	
	let id = ONE_UBIG.clone();

	let timer = Instant::now();
	poll.release(id.clone());
	let release_time = timer.elapsed().as_nanos();

	let timer = Instant::now();
	let _ = poll.get();
	let reuse_time = timer.elapsed().as_nanos();
	
	return (release_time, reuse_time);
}

fn instant_bus_test() -> String {
	let mut md: Vec<String> = vec![];
	md.push(vec![
		"## InstantBus",
		"This benchmark test will test the reception delay of different numbers of receivers (core).",
		""
	].join("\n").to_string());

	for counter in vec![1, 16, 32, 64, 128, 256, 512, 1024] {
		let image = instant_bus_once_test_image(counter);
		md.append(&mut vec![
			format!("### {} Cores / Receivers", counter),
			format!("![Benchmark]({})", image)
		]);
	}

	return md.join("\n");
}

fn instant_bus_once_test_image(test_counter: usize) -> String {
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

	let width: u32 = 800;
    let height: u32 = 600;
	
	let filepath = format!("test/instant_bus_{}.png", test_counter);
	let root = BitMapBackend::new(&filepath, (width, height)).into_drawing_area();
	root.fill(&WHITE).unwrap();
	let y_max = times.iter().cloned().max().unwrap_or(0);
	let mut chart = ChartBuilder::on(&root)
		.margin(20)
		.caption("InstantBus Delay Test", ("Consolas", 30))
		.x_label_area_size(40)
		.y_label_area_size(40)
		.build_cartesian_2d(0..times.len(), 0u128..(y_max + 50))
		.unwrap();

	chart.configure_mesh()
		.x_desc("Core")
		.y_desc("Delay")
		.draw()
		.unwrap();

	chart.draw_series(
        times.iter().enumerate().map(|(x, y)| {
            Circle::new((x, *y), 5, RED.filled())
        })
    ).unwrap();

	return format!("https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/{}", filepath.split("/").last().unwrap());
}