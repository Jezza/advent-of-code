#![feature(str_split_once)]
#![feature(bool_to_option)]
#![feature(array_value_iter)]

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

use helper::measure;
use helper::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
}

type Food = (HashSet<&'static str>, Vec<&'static str>);

fn parse_line(line: &'static str) -> Food {
	let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
	let allergens = &allergens[0..allergens.len() - 1];

	let ingredients = ingredients.split_ascii_whitespace()
		.collect::<HashSet<_>>();

	let allergens = allergens.split(", ")
		.collect::<Vec<_>>();

	(ingredients, allergens)
}

fn parse_input() -> (Vec<Food>, HashMap<&'static str, &'static str>) {
	let lines = INPUT.lines()
		.map(|line| parse_line(line))
		.collect::<Vec<_>>();

	let mut candidates = HashMap::new();

	for (ingredients, allergens) in &lines {
		for allergen in allergens {
			let set = candidates.entry(*allergen).or_insert_with(|| ingredients.clone());
			*set = &*set & &ingredients
		}
	}

	let mut lookup = HashMap::new();
	while let Some((&a, _)) = candidates.iter().find(|(_, s)| s.len() == 1) {
		let &i = candidates[a].iter().next().unwrap();
		lookup.insert(a, i);
		for (_, s) in &mut candidates {
			s.remove(&i);
		}
	}

	(lines, lookup)
}

fn part_one() -> u64 {
	let (lines, allergen_map) = parse_input();

	let allergens = allergen_map.values().collect::<HashSet<_>>();

	lines.iter()
		.flat_map(|(ingredients, _)| ingredients.iter())
		.filter(|ingredient| !allergens.contains(ingredient))
		.count() as u64
}

fn part_two() -> String {
	let (_, allergen_map) = parse_input();
	allergen_map.iter()
		.sorted()
		.map(|(_, i)| i)
		.join(",")
}
