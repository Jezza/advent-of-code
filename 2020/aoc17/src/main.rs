#![feature(const_generics)]

use std::collections::{HashMap, HashSet};

use helper::{measure, time};

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
	println!("Part Two: {}", time!(simulate::<5>()));
}

type Unit = i8;

fn part_one() -> usize {
	simulate::<3>()
}

fn part_two() -> usize {
	simulate::<4>()
}

fn simulate<const N: usize>() -> usize {
	let mut active: HashSet<[Unit; N]> = collect_input();

	let offsets = gen_offsets();

	for _ in 0..6 {
		active = count_neighbours(&offsets, &active)
			.into_iter()
			.filter_map(|(pos, n)| match (active.contains(&pos), n) {
				(true, 2) | (_, 3) => Some(pos),
				_ => None,
			})
			.collect();
	}

	active.len()
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
				})
		})
		.collect::<HashSet<_>>()
}

fn gen_offsets<const N: usize>() -> Vec<[Unit; N]> {
	use itertools::Itertools;

	(0..N).map(|_| -1..=1)
		.multi_cartesian_product()
		.filter(|offset| offset.iter().any(|v| *v != 0))
		.map(|offset| {
			offset.into_iter()
				.enumerate()
				.fold([0 as Unit; N], |mut acc, (i, v)| {
					acc[i] = v as Unit;
					acc
				})
		})
		.collect()
}

fn count_neighbours<const N: usize>(
	offsets: &[[Unit; N]],
	active: &HashSet<[Unit; N]>,
) -> HashMap<[Unit; N], usize> {

	active.iter()
		.flat_map(|pos| {
			offsets.iter()
				.map(move |offset| {
					let mut output = [0; N];
					for i in 0..offset.len() {
						output[i] = offset[i] + pos[i];
					}
					output
				})
		})
		.fold(HashMap::new(), |mut counts, pos| {
			*counts.entry(pos).or_insert(0) += 1;
			counts
		})
}