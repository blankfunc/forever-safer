use std::sync::{Arc, RwLock, atomic::{AtomicU64, Ordering}};
use num_bigint::{BigUint, ToBigUint};
use crate::seg_queue::SegQueue;

enum PollStore {
	Quick,
	Super
}

struct RawAtomicResult {
	reused: bool,
	value: BigUint
}

pub struct AtomicPoll {
	store: RwLock<PollStore>,
	quick_store: Arc<AtomicU64>,
	super_store: Arc<RwLock<BigUint>>,
	removed: SegQueue<BigUint>
}

impl AtomicPoll {
	pub fn new() -> Self {
		Self {
			store: PollStore::Quick.into(),
			quick_store: AtomicU64::new(0).into(),
			super_store: RwLock::new(BigUint::ZERO).into(),
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

		let value = match &*self.store.read().unwrap() {
			PollStore::Quick => {
				let uint = self.quick_store.load(Ordering::Acquire);
				let biguint = uint.to_biguint().unwrap();
				// Upgrade
				if uint >= u64::MAX {
					let mut store_guard = self.store.write().unwrap();
					if matches!(*store_guard, PollStore::Quick) {
						*store_guard = PollStore::Super;
						*self.super_store.write().unwrap() = biguint.clone();
					}
				}
				biguint
			},
			PollStore::Super => {
				self.super_store.read().unwrap().clone()
			},
		};

		return RawAtomicResult {
			reused: false,
			value
		};
	}

	// The `read` must be used before `increase`
	fn increase(&self) {
		match &*self.store.read().unwrap() {
			PollStore::Quick => {
				self.quick_store.fetch_add(1, Ordering::AcqRel);
			},
			PollStore::Super => {
				*self.super_store.write().unwrap() += 1.to_biguint().unwrap();
			},
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