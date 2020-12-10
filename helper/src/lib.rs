#![feature(test)]

pub mod test_export {
	extern crate test;

	pub use test::*;

	pub fn measure<T>(name: &'static str, mut func: impl Fn() -> T) -> T {
		let stats = test::bench::iter(&mut func);
		let median = stats.median as usize;
		let deviation = (stats.max - stats.min) as usize;
		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", name, median, deviation);
		func()
	}
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
macro_rules! measure {
    ($expr:expr) => {{
    	::helper::test_export::measure(stringify!($expr), || $expr)
    }};
}

