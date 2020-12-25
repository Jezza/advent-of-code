#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};

use helper::measure;
use helper::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
}

fn part_one() -> impl std::fmt::Display {

	let (card, door) = INPUT.split_once("\n").unwrap();
	let card = card.parse::<usize>().unwrap();
	let door = door.parse::<usize>().unwrap();

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

	(0..iter).fold(1usize, |acc, _| acc * public_key % 20201227)
}

fn part_two() -> impl std::fmt::Display {
	0
}
