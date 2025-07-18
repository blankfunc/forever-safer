use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc}};
use parking_lot::RwLock;
use ibig::{ubig, UBig};
use crate::seg_queue::SegQueue;
use once_cell::sync::Lazy;

#[cfg(test)]
use std::time::Instant;

#[cfg(test)]
use std::time::Duration;

static USIZE_MAX: Lazy<UBig> = Lazy::new(|| UBig::from(usize::MAX));
static ZERO_UBIG: Lazy<UBig> = Lazy::new(|| ubig!(0));
static ONE_UBIG: Lazy<UBig> = Lazy::new(|| ubig!(1));

struct RawAtomicResult {
	reused: bool,
	value: UBig
}

pub struct AtomicPoll {
	// usize::MAX Counter (Counter + Result)
	counter: Arc<RwLock<(UBig, UBig)>>,
	// Atomic Usize
	store: Arc<AtomicUsize>,
	removed: SegQueue<UBig>
}

impl AtomicPoll {
	pub fn new() -> Self {
		#[cfg(not(test))]
		let init_number = 0;

		#[cfg(test)]
		let init_number = usize::MAX - 2; // usize::MAX - 2 (trigger upgrade after +2).

		Self {
			counter: RwLock::new((ZERO_UBIG.clone(), ZERO_UBIG.clone())).into(),
			store: AtomicUsize::new(init_number).into(),
			removed: SegQueue::new()
		}
	}

	fn read(&self) -> RawAtomicResult {
		if let Some(value) = self.removed.peek() {
			return RawAtomicResult {
				reused: true,
				value
			};
		}
		
		let store = UBig::from(self.store.load(Ordering::Relaxed));
		let (_, offset) = &*self.counter.read();
		return RawAtomicResult {
			reused: false,
			value: store + offset
		};
	}

	// The `read` must be used before `increase`
	fn increase(&self) {
		let id = self.store.fetch_add(1, Ordering::Relaxed);

		// need to increase counter
		if id == usize::MAX - 1 {
			#[cfg(test)]
			println!("TRIG UPGRADE!");

			self.store.store(0, Ordering::SeqCst);

			// Add counter and result
			let mut guard = self.counter.write();
			guard.0 += &*ONE_UBIG;	// Counter
			guard.1 += &*USIZE_MAX;	// Result
		}
	}

	pub fn get(&self) -> UBig {
		let result = self.read();
		if result.reused {
			self.removed.pop();
		}
		return result.value;
	}

	pub fn get_and_increase(&self) -> UBig {
		let result = self.read();
		if !result.reused {
			self.increase();
		}
		return result.value;
	}

	pub fn release(&self, id: UBig) {
		if id > self.read().value {
			return;
		}

		if self.removed.contains(&id) {
			return;
		}

		self.removed.push(id);
	}
}

#[test]
fn increase_100times_test() {
	let poll = AtomicPoll::new();
	for _ in 0..2 {
		poll.increase();
	}

	let all_timer = Instant::now();
	let mut times: Vec<Duration> = vec![];
	for _ in 0..101 {
		let timer = Instant::now();
		poll.get_and_increase();
		times.push(timer.elapsed());
	}

	println!("[INCREASE_100T] INCREASE: {}ns ({})",
		all_timer.elapsed().as_nanos(),
		times.iter().map(|time| format!("{}ns", time.as_nanos())).collect::<Vec<_>>().join(" ")
	);
}

#[test]
fn increase_test() {
	let poll = AtomicPoll::new();

	let timer = Instant::now();
	poll.get_and_increase();
	println!("[INCREASE] INCREASE: {}ns", timer.elapsed().as_nanos());
}

#[test]
fn upgrade_test() {
	let poll = AtomicPoll::new();
	for i in 0..3 {
		let timer = Instant::now();
		poll.get_and_increase();
		if i == 1 {
			// trigger upgrade
			println!("[UPGRADE] INCREASE_AND_UPGRADE: {}ns", timer.elapsed().as_nanos());
		} else {
			println!("[UPGRADE] INCREASE: {}ns", timer.elapsed().as_nanos());
		}
	}
}

#[test]
fn reuse_test() {
	let poll = AtomicPoll::new();
	
	let id = ONE_UBIG.clone();

	let timer = Instant::now();
	poll.release(id.clone());
	println!("[REUSE] RELEASE {}: {}ns", id, timer.elapsed().as_nanos());

	let timer = Instant::now();
	let id = poll.get();
	println!("[REUSE] REUSE {}: {}ns", id, timer.elapsed().as_nanos());
}