#![feature(test)]
#![feature(type_name_of_val)]
#![feature(generic_const_exprs)]
#![feature(step_trait)]
#![feature(iter_intersperse)]

use std::str::FromStr;

pub mod export {
    #[cfg(feature = "itertools")]
    pub mod itertools {
        pub use itertools::*;
    }

    #[cfg(feature = "pathfinding")]
    pub mod pathfinding {
        pub use pathfinding::*;
    }
}

pub mod ext {
    pub trait OptionExt<T> {
        fn if_present<F: FnMut(T)>(self, f: F);
        fn if_absent<F: FnMut()>(self, f: F);
    }

    impl<T> OptionExt<T> for Option<T> {
        fn if_present<F: FnMut(T)>(self, mut f: F) {
            match self {
                Some(v) => f(v),
                None => (),
            }
        }

        fn if_absent<F: FnMut()>(self, mut f: F) {
            match self {
                Some(_) => (),
                None => f(),
            }
        }
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
macro_rules! split {
	($input:expr, $($split:expr),*$(,)?) => {{
		let mut rest = $input;
		(
			$({
				let (left, right) = rest.split_once($split).unwrap_or((rest, ""));
				rest = right;
				left
			},)*
			rest,
		)
	}};
}

pub struct Ignored;

impl FromStr for Ignored {
    type Err = std::convert::Infallible;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Ignored)
    }
}

pub mod parsing {
    use std::str::FromStr;

    pub trait Parse<'a>: Sized {
        type Error;

        fn from_str(value: &'a str) -> Result<Self, Self::Error>;
    }

    macro_rules! from_str {
        (
            $(
                $ty:ty;
            )+
        ) => {
            $(
                impl<'a> Parse<'a> for $ty {
                    type Error = <$ty as std::str::FromStr>::Err;

                    fn from_str(value: &'a str) -> Result<Self, Self::Error> {
                        value.parse()
                    }
                }
            )+
        };
    }

    impl<'a> Parse<'a> for &'a str {
        type Error = std::convert::Infallible;

        fn from_str(value: &'a str) -> Result<Self, Self::Error> {
            Ok(value)
        }
    }

    // @FIXME jezza - 07 Dec 2022: Maybe one day... :(
    // impl<'a, T> Parse<'a> for T where T: FromStr {
    //     type Error = T::Err;
    //
    //     fn from_str(value: &'a str) -> Result<Self, Self::Error> {
    //         value.parse()
    //     }
    // }

    from_str! {
        String;
        super::Ignored;
        u8;
        u16;
        u32;
        u64;
        u128;
        i8;
        i16;
        i32;
        i64;
        i128;
        usize;
        isize;
    }
}

#[macro_export]
macro_rules! split_parse {
	($input:expr, $($split:expr),*$(,)?) => {{
		let mut rest = $input.trim();
		(
			$({
				let (value, right) = rest.split_once($split).unwrap_or((rest, ""));
				rest = right;
                $crate::parse!(value)
			},)*
			{
                $crate::parse!(rest)
            },
		)
	}};
}

#[macro_export]
macro_rules! parse {
    ($input:expr) => {{
        let input: &str = $input;
        let input = input.trim();
        match $crate::parsing::Parse::from_str(input) {
            Ok(value) => value,
            Err(err) => {
                panic!("Unable to parse '{}': {}", input, err);
            }
        }
	}};
    ($input:expr, $ty:ty) => {{
		let input: &str = $input;
        let input = input.trim();
        match <$ty as $crate::parsing::Parse>::from_str(input) {
            Ok(value) => value,
            Err(err) => {
                panic!("Unable to parse '{}': {}", input, err);
            }
        }
	}};
}

#[macro_export]
macro_rules! split_parse_to_vec {
    ($input:expr, $ty:ty) => {{
		$input.lines()
			.filter_map(|v| v.trim().parse::<$ty>().ok())
			.collect::<Vec<$ty>>()
	}};
	($input:expr, $ty:ty, $sep:literal) => {{
		$input.split($sep)
			.filter_map(|v| v.trim().parse::<$ty>().ok())
			.collect::<Vec<$ty>>()
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

fn dumb_thousands(number: usize) -> String {
    let output = format!("{}", number);
    let data: Vec<_> = output.as_bytes()
        .rchunks(3)
        .rev()
        .intersperse(&[b','])
        .flatten()
        .copied()
        .collect();

    String::from_utf8(data).unwrap()
}

pub fn aoc<I, O, F, P>(
    handler: F,
    it: P,
)
    where
        I: Clone,
        F: Fn(I) -> O,
        P: IntoIterator<Item=(I, O)>,
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
            break;
        }

        if measure {
            let func = || handler(input.clone());
            let (median, deviation) = test_export::measure(&func);
            println!("\t{:>22}\tbench:\t{:>11} ns/iter (+/- {})", expected, dumb_thousands(median), dumb_thousands(deviation));
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

    #[derive(Eq, PartialEq, Clone, Debug, Hash)]
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
        pub fn display_with_fmt<O: Display>(&self, handler: impl Fn(&T) -> O) -> String {
            let mut last_y = 0;

            let mut out = String::new();

            for (x, y) in self.iter_pos_tuples() {
                if last_y != y {
                    out.push('\n');
                    last_y = y;
                }
                out.push_str(&format!("{}", handler(self.get(x, y))));
            }

            out
        }

        pub fn print_with_fmt<O: Display>(&self, handler: impl Fn(&T) -> O) {
            println!("{}", self.display_with_fmt(handler))
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

        pub fn iter_pos(&self) -> impl Iterator<Item=usize> {
            0..(self.width * self.height)
        }

        pub fn iter_pos_tuples(&self) -> impl Iterator<Item=(usize, usize)> {
            let width = self.width;

            self.iter_pos()
                .map(move |i| {
                    let x = i % width;
                    let y = i / width;

                    (x, y)
                })
        }

        pub fn iter_pos_tuples_rev(&self) -> impl Iterator<Item=(usize, usize)> {
            let width = self.width;

            (0..(self.width * self.height))
                .rev()
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
        mut func: H,
    ) -> G
        where
            CF: Fn(&'a str) -> CI,
            RF: Fn(&'a str) -> RI,
            CI: Iterator<Item=&'a str>,
            RI: Iterator<Item=&'a str>,
            F: FnMut(usize, usize) -> G,
            H: FnMut(&mut G, usize, usize, &'a str)
    {
        let (width, mut height) = row_splitter(input)
            .filter(|line| !line.is_empty())
            .map(|line| {
                column_splitter(line)
                    .filter(|segment| !segment.is_empty())
                    .count()
            })
            .enumerate()
            .fold((usize::MIN, usize::MIN), |(width, height), (index, value)| {
                (width.max(value), height.max(index))
            });

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

pub mod utils {
    #[deprecated]
    pub fn parse_range<T: std::str::FromStr>(input: &str) -> (T, T)
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        parse_range_with_sep(input, "..")
    }

    #[deprecated]
    pub fn parse_range_with_sep<T: std::str::FromStr>(input: &str, delimiter: &str) -> (T, T)
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let (left, right) = input.split_once(delimiter).unwrap();
        (left.parse::<T>().unwrap(), right.parse::<T>().unwrap())
    }

    #[cfg(all(feature = "num-integer", feature = "itertools"))]
    pub type Offsets<T, const N: usize> = [[T; N]; 3_usize.pow(N as u32)];

    #[cfg(all(feature = "num-integer", feature = "itertools"))]
    pub fn gen_offsets<T, const N: usize>() -> Offsets<T, N>
        where
            T: Copy + num_integer::Integer + std::iter::Step,
            [T; N]: Sized,
            [[T; N]; 3_usize.pow(N as u32)]: Sized,
    {
        use crate::export::itertools::Itertools;

        let iter = (0..N).map(|_| (T::zero() - T::one())..=T::one()).multi_cartesian_product();

        let value = [T::zero(); N];
        let mut output = [value; 3_usize.pow(N as u32)];

        for (i, offset) in iter.enumerate() {
            output[i] = offset.into_iter()
                .rev()
                .enumerate()
                .fold(output[i], |mut acc, (i, v)| {
                    acc[i] = v;
                    acc
                });
        }

        output
    }
}