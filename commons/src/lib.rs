#![feature(test)]
#![feature(type_name_of_val)]
#![feature(generic_const_exprs)]

pub mod export {
	#[cfg(feature = "itertools")]
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
		let (input, expected): (I, O) = item;

		let got = handler(input.clone());

		if got != expected {
			println!("\t{:>22} != {}", got, expected);
			continue;
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
	use std::fmt::Display;

	pub struct Grid<T, const W: usize, const H: usize> where [T; W * H]: Sized {
		pub width: usize,
		pub height: usize,
		pub values: [T; W * H],
	}

	impl<T, const W: usize, const H: usize> Grid<T, W, H> where [T; W * H]: Sized, T: Display {
		pub fn display(&self) -> String {
			let mut last_y = 0;

			let mut out = String::new();

			for (x, y) in self.iter_pos_tuples() {
				if last_y != y {
					out.push('\n');
					last_y = y;
				}
				out.push_str(&format!("{} |", self.get(x, y)));
			}

			out
		}

		pub fn print(&self) {
			println!("{}", self.display())
		}
	}

	impl<T, const W: usize, const H: usize> Grid<T, W, H> where [T; W * H]: Sized {
		pub fn from_values(width: usize, height: usize, values: [T; W * H]) -> Self {
			Grid {
				width,
				height,
				values,
			}
		}

		pub fn as_values(&self) -> &[T; W * H] {
			&self.values
		}

		pub fn as_values_mut(&mut self) -> &mut [T; W * H] {
			&mut self.values
		}

		pub fn get_raw(&self, i: usize) -> &T {
			&self.values[i]
		}

		pub fn get_raw_mut(&mut self, i: usize) -> &mut T {
			&mut self.values[i]
		}

		pub fn get(&self, x: usize, y: usize) -> &T {
			&self.values[x + y * self.width]
		}

		pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
			&mut self.values[x + y * self.width]
		}

		pub fn iter_pos(&self) -> impl Iterator<Item = usize> {
			0..(self.width * self.height)
		}

		pub fn iter_pos_tuples(&self) -> impl Iterator<Item = (usize, usize)> {
			let width = self.width;

			self.iter_pos()
				.map(move |i| {
					let x = i % width;
					let y = i / width;

					(x, y)
				})
		}
	}

	impl<T, const W: usize, const H: usize> Grid<T, W, H> where [T; W * H]: Sized + Default {
		pub fn new(width: usize, height: usize) -> Self {
			Grid {
				width,
				height,
				values: Default::default(),
			}
		}
	}

	pub fn parse_grid<'a, G, F, H, CF, CI, RF, RI>(
		input: &'a str,
		row_splitter: RF,
		column_splitter: CF,
		mut init: F,
		mut func: H
	) -> G
	where
		CF: Fn(&'a str) -> CI,
		RF: Fn(&'a str) -> RI,
		CI: Iterator<Item = &'a str>,
		RI: Iterator<Item = &'a str>,
		F: FnMut(usize, usize) -> G,
		H: FnMut(&mut G, usize, usize, &'a str)
	{

		let (mut height, width) = row_splitter(input)
			.filter(|line| !line.is_empty())
			.map(|line| {
				column_splitter(line)
					.filter(|segment| !segment.is_empty())
					.count()
			})
			.enumerate()
			.max()
			.unwrap();
		// `enumerate` starts at 0, so we need to bump it up by one.
		height += 1;

		let mut grid = init(width, height);

		row_splitter(input)
			.filter(|line| !line.is_empty())
			.enumerate()
			.for_each(|(y, line)| {
				column_splitter(line)
					.filter(|segment| !segment.is_empty())
					.enumerate()
					.for_each(|(x, segment)| func(&mut grid, x, y, segment))
			});

		grid
	}
}