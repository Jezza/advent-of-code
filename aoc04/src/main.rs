#![feature(str_split_once)]

use std::collections::HashMap;

fn main() {
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	check_details(|opts| opts.contains_key("byr")
		&& opts.contains_key("iyr")
		&& opts.contains_key("eyr")
		&& opts.contains_key("hgt")
		&& opts.contains_key("hcl")
		&& opts.contains_key("ecl")
		&& opts.contains_key("pid"))
}

fn part_two() -> usize {
	let check_range = |value: &str, range: std::ops::RangeInclusive<usize>| range.contains(&value.parse::<usize>().unwrap());

	// a # followed by exactly six characters 0-9 or a-f.
	let check_hex = |input: &str|
		input.starts_with("#") && input.as_bytes()
			.iter()
			.skip(1)
			.all(|c| b'0' <= *c && *c <= b'9' || b'a' <= *c && *c <= b'z');

	let check_eyes = |input: &str| match input {
		| "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
		_ => false,
	};

	// a nine-digit number, including leading zeroes.
	let check_passport = |input: &str|
		input.len() == 9 && input.as_bytes()
			.iter()
			.all(|c| b'0' <= *c && *c <= b'9');

	check_details(|opts| {
		opts.get("byr").map_or(false, |value| check_range(value, 1920..=2002))
			&& opts.get("iyr").map_or(false, |&value| check_range(value, 2010..=2020))
			&& opts.get("eyr").map_or(false, |&value| check_range(value, 2020..=2030))
			&& opts.get("hgt").map_or(false, |&value| check_range(&value[..value.len() - 2], if value.ends_with("cm") { 150..=193 } else { 59..=76 }))
			&& opts.get("hcl").map_or(false, |&value| check_hex(value))
			&& opts.get("ecl").map_or(false, |&value| check_eyes(value))
			&& opts.get("pid").map_or(false, |&value| check_passport(value))
	})
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

