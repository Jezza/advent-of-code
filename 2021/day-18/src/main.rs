#![feature(box_patterns)]

use commons::*;
use commons::export::itertools::Itertools;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 1384),
			(INPUT, 4235),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 2778),
			(INPUT, 4659),
		],
	);
}

#[derive(Debug, Clone)]
enum Item {
	Value(u8),
	Pair(Box<Item>, Box<Item>),
}

fn take<'a>(bits: &mut &'a str, at: usize) -> &'a str {
	let (left, rest) = bits.split_at(at);
	*bits = rest;
	left
}

impl Item {
	fn parse(input: &mut &str) -> Self {
		let value = take(input, 1);
		if value == "[" {
			let left = Item::parse(input);
			take(input, 1);

			let right = Item::parse(input);
			take(input, 1);

			Item::Pair(Box::new(left), Box::new(right))
		} else {
			let value = value.as_bytes()[0] - b'0';
			Item::Value(value)
		}
	}
}

fn explode(item: &mut Item) -> bool {
	fn implode<'a>(item: &'a mut Item, previous: &mut Option<&'a mut u8>, exploding: &mut Option<u8>, depth: u8) -> bool {
		match item {
			Item::Value(value) => {
				if let Some(exploding) = exploding {
					*value += *exploding;
					return true;
				} else {
					*previous = Some(value);
				}
			}
			Item::Pair(box Item::Value(left), box Item::Value(right)) if depth >= 4 && exploding.is_none() => {
				if let Some(previous) = previous {
					**previous += *left;
				}
				*exploding = Some(*right);
				*item = Item::Value(0);
			}
			Item::Pair(left, right) => {
				if implode(left, previous, exploding, depth + 1)
					|| implode(right, previous, exploding, depth + 1) {
					return true;
				}
			}
		}
		false
	}

	let mut exploding = None;
	implode(item, &mut None, &mut exploding, 0);
	exploding.is_some()
}

fn split(item: &mut Item) -> bool {
	match item {
		Item::Value(value) if *value >= 10 => {
			let left = *value / 2;
			let right = (*value + 1) / 2;
			*item = Item::Pair(Box::new(Item::Value(left)), Box::new(Item::Value(right)));
			return true;
		}
		Item::Pair(left, right) => {
			if split(left) || split(right) {
				return true;
			}
		}
		_ => (),
	}
	false
}

fn reduce(item: &mut Item) {
	while explode(item) || split(item) {}
}

fn magnitude(item: &Item) -> u64 {
	match item {
		Item::Value(value) => *value as u64,
		Item::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
	}
}

fn part_one(input: &str) -> u64 {
	let reduced = input.lines()
		.map(|mut line| Item::parse(&mut line))
		.reduce(|acc, item| {
			let mut item = Item::Pair(Box::new(acc), Box::new(item));
			reduce(&mut item);
			item
		})
		.unwrap();

	magnitude(&reduced)
}

fn part_two(input: &str) -> u64 {
	let items = input.lines()
		.map(|mut line| Item::parse(&mut line))
		.collect::<Vec<_>>();

	let mut max = u64::MIN;
	for (left, right) in items.iter().cartesian_product(items.iter()) {
		let mut item = Item::Pair(Box::new(left.clone()), Box::new(right.clone()));
		reduce(&mut item);
		max = max.max(magnitude(&item));
	}

	max
}

fn format(item: &Item, out: &mut String) {
	use std::fmt::Write;

	match item {
		Item::Value(v) => write!(out, "{}", v).unwrap(),
		Item::Pair(left, right) => {
			*out += " [";
			format(left, out);
			*out += ", ";
			format(right, out);
			*out += "] ";
		}
	}
}

fn print(item: &Item) {
	let mut out = String::new();
	format(&item, &mut out);
	println!("{}", out);
}