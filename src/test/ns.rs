use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};

pub fn display_ns(raw_ns: u128) -> String {
	let ns = Decimal::from_u128(raw_ns).unwrap();

	let n100 = Decimal::new(100, 0);
	let n1000 = Decimal::new(1000, 0);

	let us = ns / n1000;
	let us_2 = (us * n100).round() / n100;

	let ms: Decimal = us / n1000;
	let ms_2 = (ms * n100).round() / n100;

	return format!("{}ns ({}μs, {}ms)", ns, us_2, ms_2);
}

pub fn average_u128(list: Vec<u128>) -> u128 {
	let sum = list.iter().map(|n| Decimal::from_u128(*n).unwrap()).reduce(|a, b| a + b).unwrap();
	let len = Decimal::from_usize(list.len()).unwrap();
	let average = (sum / len).round();
	
	return average.to_u128().unwrap();
}