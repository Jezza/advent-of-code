#![feature(str_split_once)]
#![feature(const_generics)]

use std::collections::{HashMap, HashSet};

use helper::measure;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part Two: {}", part_one());
	// println!("Part Two: {}", part_two());
}

type Unit = i8;

fn part_one() -> usize {
	simulate::<3>()
}

fn part_two() -> usize {
	simulate::<4>()
}

fn gen_offsets<const N: usize>() -> impl Iterator<Item = [Unit; N]> {
	use itertools::Itertools;

	(0..N).map(|_| -1..=1)
		.multi_cartesian_product()
		.filter(|offset| offset.iter().any(|v| *v != 0))
		.map(|offset| {
			let mut output: [Unit; N] = [0; N];
			offset.into_iter()
				.enumerate()
				.for_each(|(i, v)| output[i] = v as Unit);
			output
		})
}

fn count_neighbours<const N: usize>(offsets: &[[Unit; N]], active: &HashSet<[Unit; N]>) -> HashMap<[Unit; N], usize> {
	active.iter()
		.flat_map(|values| {
			offsets.iter()
				.map(move |offset: &[Unit; N]| {
					let mut output: [Unit; N] = [0; N];
					for i in 0..offset.len() {
						output[i] = offset[i] + values[i];
					}
					output
				})
		})
		.fold(HashMap::new(), |mut counts, pos| {
			*counts.entry(pos).or_insert(0) += 1;
			counts
		})
}

fn simulate<const N: usize>() -> usize {
	let mut active: HashSet<[Unit; N]> = collect_input();

	let offsets = gen_offsets()
		.collect::<Vec<_>>();

	for _ in 0..6 {
		active = count_neighbours(&offsets, &active)
			.iter()
			.filter_map(|(pos, n)| match (active.contains(pos), n) {
				(true, 2) | (_, 3) => Some(*pos),
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
