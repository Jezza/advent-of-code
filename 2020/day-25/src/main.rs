#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

use commons::measure;
use commons::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
}

type Value = u32;

fn parse_input() -> (Value, Value) {
	let (card, door) = INPUT.split_once("\n").unwrap();
	let card = card.parse().unwrap();
	let door = door.parse().unwrap();
	(card, door)
}

fn part_one() -> impl std::fmt::Display {
	let (card, door) = parse_input();

	let mut value = 1;
	let mut iter = 0;

	while value != card && value != door {
		value = value * 7 % 20201227;
		iter += 1;
	}

	let public_key = if value == card {
		door
	} else {
		card
	};

	(0..iter).fold(1 as Value, |acc, _| acc * public_key % 20201227)
}

fn part_two() -> impl std::fmt::Display {
	let (card, door) = parse_input();

	let hash = |value, subject| value * subject % 20201227;

	let (left, right) = if card > door {
		(door, card)
	} else {
		(card, door)
	};

	let mut value = hash(1, 7);
	let mut key = hash(1, right);

	while left != value {
		value = hash(value, 7);
		key = hash(key, right);
	}

	key
}
