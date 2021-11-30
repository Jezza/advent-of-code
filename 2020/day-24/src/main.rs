#![feature(array_windows)]

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

// part_one(): 468µs
// Part One: 391
// part_two(): 65.2108ms
// Part Two: 3876

// test part_one()                          	bench:	    137_313 ns/iter (+/- 5056)
// Part One: 391
// test part_two()                          	bench:	 60_708_170 ns/iter (+/- 2726116)
// Part Two: 3876

static OFFSETS: [Point; 6] = [
	( 1,  0),
	( 0,  1),
	(-1,  1),
	(-1,  0),
	( 0, -1),
	( 1, -1)
];

type Unit = i64;
type Point = (Unit, Unit);

fn parse_line(input: &str) -> Point {
	let input = input.as_bytes();
	let mut index = 0;
	let (mut q, mut r) = (0, 0);

	while index < input.len() {
		let ((dq, dr), offset) = match (input[index], input.get(index + 1).unwrap_or(&0)) {
			(b'e', _)    => (OFFSETS[0], 1),
			(b's', b'e') => (OFFSETS[1], 2),
			(b's', b'w') => (OFFSETS[2], 2),
			(b'w', _)    => (OFFSETS[3], 1),
			(b'n', b'w') => (OFFSETS[4], 2),
			(b'n', b'e') => (OFFSETS[5], 2),
			c => panic!("Unknown: {:?}", c),
		};

		q += dq;
		r += dr;

		index += offset;
	}

	(q, r)
}

fn parse_input() -> HashSet<Point> {
	let mut points = HashSet::new();

	INPUT.lines()
		.map(parse_line)
		.for_each(|point| if !points.insert(point) {
			points.remove(&point);
		});

	points
}

fn part_one() -> impl std::fmt::Display {
	parse_input().len()
}

fn part_two() -> impl std::fmt::Display {
	let mut points = parse_input();

	let mut neighbours = HashMap::new();
	for _ in 0..100 {
		points.iter()
			.flat_map(|(q, r)| OFFSETS.iter().map(move |(dq, dr)| (q + dq, r + dr)))
			.for_each(|point| *neighbours.entry(point).or_insert(0) += 1);

		points = neighbours.drain()
			.filter(|&(ref t, n)| n == 2 || (n == 1 && points.contains(t)))
			.map(|(t, _)| t)
			.collect();
	}

	points.len()
}
