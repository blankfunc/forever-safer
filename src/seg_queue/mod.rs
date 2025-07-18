use crossbeam::queue;
use dashmap::DashSet;
use std::sync::Arc;
use parking_lot::RwLock;
use std::hash::Hash;

pub struct SegQueue<T>
where
	T: Eq + Hash + Clone
{
	inner: queue::SegQueue<T>,
	inner_cache: Arc<RwLock<Option<T>>>,
	inner_set: DashSet<T>
}

impl<T> SegQueue<T>
where
	T: Eq + Hash + Clone
{
	pub fn new() -> Self {
		Self {
			inner: queue::SegQueue::new(),
			inner_cache: Arc::new(RwLock::new(None)),
			inner_set: DashSet::new()
		}
	}

	fn read_first(&self) -> Option<T> {
		if let Some(value) = &*self.inner_cache.read() {
			return Some(value.clone());
		}

		if let Some(value) = &self.inner.pop() {
			*self.inner_cache.write() = Some(value.clone());
			return Some(value.clone());
		}

		return None;
	}

	// Read first without Removed
	pub fn peek(&self) -> Option<T> {
		return self.read_first();
	}

	// Read first with Removed
	pub fn pop(&self) -> Option<T> {
		if self.read_first().is_none() {
			return None;
		}

		let value = self.inner_cache.write().take().unwrap();
		self.inner_set.remove(&value);

		return Some(value);
	}

	// Contains value
	pub fn contains(&self, value: &T) -> bool {
		return self.inner_set.contains(value);
	}

	// Push value
	pub fn push(&self, value: T) {
		if !self.inner_set.insert(value.clone()) {
			return;
		}
		
		self.inner.push(value);
	}
}