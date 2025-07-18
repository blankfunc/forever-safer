use std::sync::{atomic::{AtomicBool, Ordering}, Arc, Weak};
use std::hash::Hash;
use dashmap::DashMap;
use ibig::UBig;
use flume::{unbounded, Receiver, Sender};

use crate::atomic_poll::AtomicPoll;

#[cfg(test)]
use std::time::Instant;

#[cfg(test)]
use crossbeam::queue::SegQueue;

pub struct InstantBus<T>
where
	T: Eq + Hash + Clone
{
	inner: Arc<DashMap<UBig, (Sender<Arc<T>>, Arc<AtomicBool>)>>,
	next_id: AtomicPoll
}

impl<T> InstantBus<T>
where
	T: Eq + Hash + Clone
{
	pub fn new() -> Self {
		Self {
			inner: Arc::new(DashMap::new()),
			next_id: AtomicPoll::new()
		}
	}

	pub fn subscribe(&self) -> Subscriber<T> {
		let (sender, receiver) = unbounded::<Arc<T>>();
		let id = self.next_id.get_and_increase();
		let closed = Arc::new(AtomicBool::new(false));

		self.inner.insert(id.clone(), (sender, closed.clone()));

		Subscriber {
			id, closed, receiver,
			parent: Arc::downgrade(&self.inner)
		}
	}

	pub fn send(&self, value: T) {
		let remove_receiver = |id: UBig| self.inner.remove(&id);
		let arc_value = Arc::new(value);

		for entry in self.inner.iter() {
			let id = entry.key().clone();
			let (sender, closed) = entry.value();

			if closed.load(Ordering::Relaxed) {
				remove_receiver(id);
				continue;
			}

			if sender.send(arc_value.clone()).is_err() {
				remove_receiver(id);
				continue;
			}
		}
	}
}

pub struct Subscriber<T>
where
	T: Eq + Hash + Clone
{
    id: UBig,
    closed: Arc<AtomicBool>,
    receiver: Receiver<Arc<T>>,
    parent: Weak<DashMap<UBig, (Sender<Arc<T>>, Arc<AtomicBool>)>>,
}

impl<T> Subscriber<T>
where
	T: Eq + Hash + Clone
{
	pub fn recv(&mut self) -> Option<T> {
		if self.is_closed() {
			return None;
		}

		match self.receiver.recv() {
			Ok(value) => Some((*value).clone()),
			Err(_) => None,
		}
	}

	pub fn close(&self) {
		self.closed.store(true, Ordering::Relaxed);
		if let Some(parent) = self.parent.upgrade() {
			parent.remove(&self.id);
		}
	}

	pub fn is_closed(&self) -> bool {
		self.closed.load(Ordering::Relaxed)
	}
}

#[cfg(test)]
fn delay_test(test_counter: usize) {
	let bus = Arc::new(InstantBus::<Instant>::new());

	let counter = Arc::new(std::sync::atomic::AtomicU8::new(0));
	let timers = Arc::new(SegQueue::<Instant>::new());
	for _ in 0..test_counter {
		let timers_clone = timers.clone();
		let counter_clone = counter.clone();
		let mut bus_clone = bus.clone().subscribe();
		std::thread::spawn(move || {
			counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
			let _ = bus_clone.recv().unwrap();
			let start = Instant::now();
			bus_clone.recv().unwrap();
			timers_clone.push(start);
		});
	}
	
	loop {
		if counter.load(std::sync::atomic::Ordering::SeqCst) == test_counter.try_into().unwrap() {
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

	println!("[{}T_DELAY] DELAY: {}ns / {}",
		test_counter,
		times.iter().sum::<u128>() / times.len() as u128,
		times.iter().map(|time| format!("{}ns", time)).collect::<Vec<_>>().join(" ")
	);
}

#[test]
fn delay_once_test() {
	delay_test(1);
}

#[test]
fn delay_32t_test() {
	delay_test(32);
}

#[test]
fn delay_64t_test() {
	delay_test(64);
}

#[test]
fn delay_128t_test() {
	delay_test(128);
}