use std::sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}};
use num_bigint::{BigUint, ToBigUint};
use crate::seg_queue::SegQueue;
use once_cell::sync::Lazy;

static USIZE_MAX: Lazy<BigUint> = Lazy::new(|| usize::MAX.to_biguint().unwrap());
static ONE_BIGUINT: Lazy<BigUint> = Lazy::new(|| 1.to_biguint().unwrap());

struct RawAtomicResult {
	reused: bool,
	value: BigUint
}

pub struct AtomicPoll {
	// usize::MAX Counter
	counter: Arc<RwLock<BigUint>>,
	// Atomic Usize
	store: Arc<AtomicUsize>,
	removed: SegQueue<BigUint>
}

impl AtomicPoll {
	pub fn new() -> Self {
		Self {
			counter: RwLock::new(BigUint::ZERO).into(),
			store: AtomicUsize::new(0).into(),
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
		
		let store = self.store.load(Ordering::Relaxed).to_biguint().unwrap();
		let counter = self.counter.read().unwrap().clone();
		return RawAtomicResult {
			reused: false,
			value: store + &*USIZE_MAX * counter
		};
	}

	// The `read` must be used before `increase`
	fn increase(&self) {
		let id = self.store.fetch_add(1, Ordering::Relaxed);

		// need to increase counter
		if id == usize::MAX {
			self.store.store(0, Ordering::Relaxed);

			// Add counter
			*self.counter.write().unwrap() += &*ONE_BIGUINT;
		}
	}

	pub fn get(&self) -> BigUint {
		let result = self.read();
		if result.reused {
			self.removed.pop();
		}
		return result.value;
	}

	pub fn get_and_increase(&self) -> BigUint {
		let result = self.read();
		if !result.reused {
			self.increase();
		}
		return result.value;
	}

	pub fn release(&self, id: BigUint) {
		if id > self.read().value {
			return;
		}

		if self.removed.contains(&id) {
			return;
		}

		self.removed.push(id);
	}
}