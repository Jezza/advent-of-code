#![feature(str_split_once)]
#![feature(test)]

extern crate test;

macro_rules! measure {
    ($expr:expr) => {{
    	let stats = test::bench::iter(&mut || $expr);
		let median = stats.median as usize;
		let deviation = (stats.max - stats.min) as usize;
		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($expr), median, deviation);
		$expr
    }};
}

use std::collections::HashMap;

static FIELDS: &[(&str, fn(&str) -> bool)] = {
	#[inline(always)]
	fn check_range(value: &str, range: std::ops::RangeInclusive<usize>) -> bool {
		range.contains(&value.parse::<usize>().unwrap())
	}

	&[
		("byr", |value| check_range(value, 1920..=2002)),
		("iyr", |value| check_range(value, 2010..=2020)),
		("eyr", |value| check_range(value, 2020..=2030)),
		("hgt", |value| check_range(&value[..value.len() - 2], if value.ends_with("cm") { 150..=193 } else { 59..=76 })),
		("hcl", |value| value.starts_with("#") && value.as_bytes().iter().skip(1).all(|c| b'0' <= *c && *c <= b'9' || b'a' <= *c && *c <= b'z')),
		("ecl", |value| matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")),
		("pid", |value| value.len() == 9 && value.as_bytes().iter().all(|c| b'0' <= *c && *c <= b'9')),
	]
};

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
}

fn part_one() -> usize {
	check_details(|passport| FIELDS.iter().all(|(value, _)| passport.contains_key(value)))
}

fn part_two() -> usize {
	check_details(|passport| FIELDS.iter().all(|&(value, func)| passport.get(value).map_or(false, |&f| func(f))))
}

fn check_details(handler: impl Fn(&HashMap<&'static str, &'static str>) -> bool) -> usize {
	include_str!("../input/input.txt")
		.split("\n\n")
		.map(|line| line.split_ascii_whitespace()
			.filter_map(|line| line.split_once(':'))
			.collect::<HashMap<&'static str, &'static str>>())
		.filter(|opts| handler(&opts))
		.count()
}

