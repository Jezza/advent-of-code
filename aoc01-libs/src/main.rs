use anyhow::{Result, Context};

use itertools::Itertools;

fn main() -> Result<()> {
	let numbers = read_numbers()?;

	let (l0, r0) = numbers.iter()
		.cartesian_product(numbers.iter())
		.find(|(l, r)| *l + *r == 2020)
		.context("Unable to find matching pair.")?;

	println!("Part One: {}", l0 * r0);

	let ((l, m), r) = numbers.iter()
		.cartesian_product(numbers.iter())
		.cartesian_product(numbers.iter())
		.find(|((l, m), r)| *l + *m + *r == 2020)
		.context("Unable to find matching pair.")?;

	println!("Part Two: {}", l * m * r);

	Ok(())
}

fn read_numbers() -> Result<Vec<u32>> {
	include_str!("../input/input.txt")
		.lines()
		.enumerate()
		.map(|(i, line)| line.parse().with_context(|| format!("Unable to parse number on line {}", i)))
		.collect()
}
