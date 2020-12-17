#![feature(str_split_once)]
#![feature(const_generics)]

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use helper::measure;

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

type Unit = i8;

fn simulate_pocket<Pos: Hash + Eq + Copy, F: Fn(&HashSet<Pos>) -> HashMap<Pos, usize>>(
	mut active: HashSet<Pos>,
	count_neighbours: F,
	steps: usize,
) -> usize {
	for _ in 0..steps {
		active = count_neighbours(&active)
			.iter()
			.filter_map(|(pos, n)| match (active.contains(pos), n) {
				(true, 2) | (_, 3) => Some(*pos),
				_ => None,
			})
			.collect();
	}

	active.len()
}

type Pos3 = (Unit, Unit, Unit);

fn count_neighbours(active: &HashSet<Pos3>) -> HashMap<Pos3, usize> {
	const OFFSETS: [Pos3; 26] = [
		(-1,-1,-1), (-1,-1, 0), (-1,-1, 1),
		(-1, 0,-1), (-1, 0, 0), (-1, 0, 1),
		(-1, 1,-1), (-1, 1, 0), (-1, 1, 1),
		( 0,-1,-1), ( 0,-1, 0), ( 0,-1, 1),
		( 0, 0,-1),             ( 0, 0, 1),
		( 0, 1,-1), ( 0, 1, 0), ( 0, 1, 1),
		( 1,-1,-1), ( 1,-1, 0), ( 1,-1, 1),
		( 1, 0,-1), ( 1, 0, 0), ( 1, 0, 1),
		( 1, 1,-1), ( 1, 1, 0), ( 1, 1, 1),
	];

	active.iter()
		.flat_map(|(x, y, z)| OFFSETS.iter().map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz)))
		.fold(HashMap::new(), |mut counts, pos| {
			*counts.entry(pos).or_insert(0) += 1;
			counts
		})
}

type Pos4 = (Unit, Unit, Unit, Unit);

fn count_neighbours_4(active: &HashSet<Pos4>) -> HashMap<Pos4, usize> {
	const OFFSETS: [Pos4; 80] = [
		(-1,-1,-1,-1), (-1,-1,-1, 0), (-1,-1,-1, 1),
		(-1,-1, 0,-1), (-1,-1, 0, 0), (-1,-1, 0, 1),
		(-1,-1, 1,-1), (-1,-1, 1, 0), (-1,-1, 1, 1),
		(-1, 0,-1,-1), (-1, 0,-1, 0), (-1, 0,-1, 1),
		(-1, 0, 0,-1), (-1, 0, 0, 0), (-1, 0, 0, 1),
		(-1, 0, 1,-1), (-1, 0, 1, 0), (-1, 0, 1, 1),
		(-1, 1,-1,-1), (-1, 1,-1, 0), (-1, 1,-1, 1),
		(-1, 1, 0,-1), (-1, 1, 0, 0), (-1, 1, 0, 1),
		(-1, 1, 1,-1), (-1, 1, 1, 0), (-1, 1, 1, 1),
		( 0,-1,-1,-1), ( 0,-1,-1, 0), ( 0,-1,-1, 1),
		( 0,-1, 0,-1), ( 0,-1, 0, 0), ( 0,-1, 0, 1),
		( 0,-1, 1,-1), ( 0,-1, 1, 0), ( 0,-1, 1, 1),
		( 0, 0,-1,-1), ( 0, 0,-1, 0), ( 0, 0,-1, 1),
		( 0, 0, 0,-1),                ( 0, 0, 0, 1),
		( 0, 0, 1,-1), ( 0, 0, 1, 0), ( 0, 0, 1, 1),
		( 0, 1,-1,-1), ( 0, 1,-1, 0), ( 0, 1,-1, 1),
		( 0, 1, 0,-1), ( 0, 1, 0, 0), ( 0, 1, 0, 1),
		( 0, 1, 1,-1), ( 0, 1, 1, 0), ( 0, 1, 1, 1),
		( 1,-1,-1,-1), ( 1,-1,-1, 0), ( 1,-1,-1, 1),
		( 1,-1, 0,-1), ( 1,-1, 0, 0), ( 1,-1, 0, 1),
		( 1,-1, 1,-1), ( 1,-1, 1, 0), ( 1,-1, 1, 1),
		( 1, 0,-1,-1), ( 1, 0,-1, 0), ( 1, 0,-1, 1),
		( 1, 0, 0,-1), ( 1, 0, 0, 0), ( 1, 0, 0, 1),
		( 1, 0, 1,-1), ( 1, 0, 1, 0), ( 1, 0, 1, 1),
		( 1, 1,-1,-1), ( 1, 1,-1, 0), ( 1, 1,-1, 1),
		( 1, 1, 0,-1), ( 1, 1, 0, 0), ( 1, 1, 0, 1),
		( 1, 1, 1,-1), ( 1, 1, 1, 0), ( 1, 1, 1, 1),
	];

	active.iter()
		.flat_map(|(x, y, z, w)| OFFSETS.iter().map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw)))
		.fold(HashMap::new(), |mut counts, pos| {
			*counts.entry(pos).or_insert(0) += 1;
			counts
		})
}

fn part_one() -> usize {
	let input = include_str!("../input/input.txt")
		.lines()
		.enumerate()
		.flat_map(|(y, row)| {
			row.as_bytes()
				.iter()
				.enumerate()
				.filter(|&(_, c)| *c == b'#')
				.map(move |(x, _)| (x as Unit, y as Unit, 0 as Unit))
		})
		.collect::<HashSet<_>>();

	simulate_pocket(input, count_neighbours, 6)
}

fn part_two() -> usize {

	let input: HashSet<[Unit; 4]> = collect_input();

	let input = include_str!("../input/input.txt")
		.lines()
		.enumerate()
		.flat_map(|(y, row)| {
			row.as_bytes()
				.iter()
				.enumerate()
				.filter(|&(_, c)| *c == b'#')
				.map(move |(x, _)| (x as Unit, y as Unit, 0 as Unit, 0 as Unit))
		})
		.collect::<HashSet<_>>();

	simulate_pocket(input, count_neighbours_4, 6)
}

fn collect_input<const N: usize>() -> HashSet<[Unit; N]> {
	include_str!("../input/input.txt")
		.lines()
		.enumerate()
		.flat_map(|(y, row)| {
			row.as_bytes()
				.iter()
				.enumerate()
				.filter(|&(_, c)| *c == b'#')
				.map(move |(x, _)| {
					let mut pos: [Unit; N] = [0; N];
					pos[0] = x as Unit;
					pos[1] = y as Unit;
					pos
					// [
					// 	..Default::default()
					// ]
					// (x as Unit, y as Unit, 0 as Unit, 0 as Unit)
				})
		})
		.collect::<HashSet<_>>()
}
