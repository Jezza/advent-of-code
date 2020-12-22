#![feature(str_split_once)]
#![feature(bool_to_option)]
#![feature(array_value_iter)]

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

use helper::measure;
use helper::time;
use std::hash::{Hash, Hasher};

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", time!(part_one()));
	// println!("Part Two: {}", time!(part_two()));
}

type Card = u16;
type Deck = Vec<Card>;

fn parse_input() -> (Deck, Deck) {
	fn parse_player(input: &str) -> Deck {
		input.lines()
			.skip(1)
			.filter_map(|line| line.parse::<Card>().ok())
			.collect()
	}

	let (player_one, player_two) = INPUT.split_once("\n\n").unwrap();

	(
		parse_player(player_one),
		parse_player(player_two),
	)
}

fn part_one() -> impl std::fmt::Display {
	let (mut player_one, mut player_two) = parse_input();

	while !player_one.is_empty() && !player_two.is_empty() {
		let left = player_one.remove(0);
		let right = player_two.remove(0);

		if left > right {
			player_one.push(left);
			player_one.push(right);
		} else {
			player_two.push(right);
			player_two.push(left);
		}
	}

	let winner = if player_two.is_empty() {
		player_one
	} else {
		player_two
	};

	winner.into_iter()
		.rev()
		.enumerate()
		.map(|(i, v)| (i as u16 + 1) * v)
		.sum::<u16>()
}

fn play_recursive_round(mut player_one: Deck, mut player_two: Deck) -> (Deck, Deck) {
	let mut played = HashSet::new();

	while !player_one.is_empty() && !player_two.is_empty() {

		let already_played = {
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			player_one.hash(&mut hasher);
			player_two.hash(&mut hasher);
			let hash = hasher.finish();

			!played.insert((hash, hash))
		};

		if already_played {
			return (player_one, Vec::new());
		}

		let left = player_one.remove(0);
		let right = player_two.remove(0);

		let left_wins = if player_one.len() < left as usize || player_two.len() < right as usize {
			left > right
		} else {
			let player_one = player_one.iter()
				.copied()
				.take(left as usize)
				.collect();
			let player_two = player_two.iter()
				.copied()
				.take(right as usize)
				.collect();
			let (_, winner_two) = play_recursive_round(player_one, player_two);
			winner_two.is_empty()
		};

		if left_wins {
			player_one.push(left);
			player_one.push(right);
		} else {
			player_two.push(right);
			player_two.push(left);
		}
	}

	if player_two.is_empty() {
		(player_one, Vec::new())
	} else {
		(Vec::new(), player_two)
	}
}

fn part_two() -> impl std::fmt::Display {
	let (mut player_one, mut player_two) = parse_input();

	let (player_one, player_two) = play_recursive_round(player_one, player_two);

	let winner = if player_two.is_empty() {
		player_one
	} else {
		player_two
	};

	winner.into_iter()
		.rev()
		.enumerate()
		.map(|(i, v)| (i as u16 + 1) * v)
		.sum::<u16>()
}
