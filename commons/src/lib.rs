#![feature(test)]
#![feature(type_name_of_val)]

pub mod export {
	pub mod itertools {
		pub use itertools::*;
	}
}

pub mod test_export {
	extern crate test;

	pub use test::*;

	pub fn print_measure<T>(name: &'static str, func: impl Fn() -> T) -> T {
		let (median, deviation) = measure(&func);
		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", name, median, deviation);
		func()
	}

	pub fn measure<T>(mut func: impl Fn() -> T) -> (usize, usize) {
		let stats = test::bench::iter(&mut func);
		let median = stats.median as usize;
		let deviation = (stats.max - stats.min) as usize;
		(median, deviation)
	}

	// pub fn measure<T>(name: &'static str, mut func: impl Fn() -> T) -> T {
	// 	let stats = test::bench::iter(&mut func);
	// 	let median = stats.median as usize;
	// 	let deviation = (stats.max - stats.min) as usize;
	// 	println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", name, median, deviation);
	// 	func()
	// }
}

// macro_rules! measure {
//     ($expr:expr) => {{
//     	let stats = test::bench::iter(&mut || $expr);
// 		let median = stats.median as usize;
// 		let deviation = (stats.max - stats.min) as usize;
// 		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($expr), median, deviation);
// 		$expr
//     }};
// }

#[macro_export]
macro_rules! parse {
    ($input:expr, $ty:ty) => {{
		$input.lines()
			.filter_map(|v| v.trim().parse::<$ty>().ok())
			.collect::<Vec<_>>()
	}};
	($input:expr, $ty:ty, $sep:literal) => {{
		$input.split($sep)
			.filter_map(|v| v.trim().parse::<$ty>().ok())
			.collect::<Vec<_>>()
	}};
}

#[macro_export]
macro_rules! measure {
    ($expr:expr) => {{
    	::commons::test_export::print_measure(stringify!($expr), || $expr)
    }};
}

#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
    	let start = ::std::time::Instant::now();
    	let result = $expr;
		println!("{}: {:?}", stringify!($expr), start.elapsed());
    	result
    }};
}

pub fn aoc<I, O, F, P>(
	handler: F,
	it: P
)
where
	I: Clone,
	F: Fn(I) -> O,
	P: IntoIterator<Item = (I, O)>,
	O: PartialEq + std::fmt::Display,
{
	let name = std::any::type_name::<F>();
	println!("{}", name);

	#[allow(unused_assignments, unused_mut)]
	let mut measure = true;
	#[cfg(debug_assertions)]
	{
		measure = false;
	}

	for item in it {
		let item: (I, O) = item;
		let (
			input,
			expected,
		) = item;

		let got = handler(input.clone());

		if got != expected {
			println!("\t{:>22} != {}", got, expected);
			if measure {
				continue;
			} else {
				break;
			}
		}

		if measure {
			let func = || handler(input.clone());
			let (median, deviation) = test_export::measure(&func);
			println!("\t{:>22}\tbench:\t{:>11} ns/iter (+/- {})", expected, median, deviation);
		} else {
			println!("\t{:>22}", expected);
		}

		// let value = if measure {
		// 	let t = test_export::measure(&func);
		// } else {
		// 	func()
		// };

		// println!("\t{}: {}", name, value);
		// if output != value {
		// 	println!("\t{} != {}", value, output);
		// }
		// assert_eq!(output, value);
	}
}

pub mod grid {
	pub fn find_grid_size(
		input: &str,
	) -> (usize, usize) {
		let height = input.lines()
			.count();

		let width = input.lines()
			.map(|line| {
				line.split_ascii_whitespace()
					.filter(|segment| !segment.is_empty())
					.count()
			})
			.max()
			.unwrap_or_default();

		(width, height)
	}

	pub fn parse_grid<F>(
		input: &str,
		mut func: F,
	)
	where
		F: FnMut(usize, usize, &str),
	{

		// let mut x = 0;
		// let mut y = 0;
		//
		// {
		// 	let bytes = input.as_bytes();
		// 	let mut index = 0;
		// 	let mut start = 0;
		// 	while index < bytes.len() {
		// 		let byte = bytes[index];
		//
		// 		if byte == b' ' {
		//
		// 			let segment = &input[start..index];
		// 			if !segment.is_empty() {
		//
		// 				x += 1;
		// 			}
		//
		// 			index += 1;
		// 			continue;
		// 		} else if byte == b'\n' {
		//
		// 			x = 0;
		// 			y += 1;
		// 		}
		//
		//
		// 	}
		// }

		// println!("===\n{}\n===", input);
		// input.split(|c: char| c == '\n' || c.is_ascii_whitespace())
		// 	.filter(|segment| !segment.is_empty())
		// 	.for_each(|segment| {
		// 		println!("{}", segment);
		// 	});


		input.lines()
			.filter(|line| !line.is_empty())
			.enumerate()
			.for_each(|(y, line)| {
				line.split_ascii_whitespace()
					.filter(|segment| !segment.is_empty())
					.enumerate()
					.for_each(|(x, segment)| func(x, y, segment))
			})
	}

}