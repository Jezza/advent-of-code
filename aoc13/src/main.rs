#![feature(str_split_once)]

use helper::measure;

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	let t = include_str!("../input/input.txt");

	let (start, times) = t.split_once('\n').unwrap();
	let start = start.parse::<usize>().unwrap();

	times.split(',')
		.filter_map(|v| v.parse::<usize>().ok())
		.map(|id| (id, id - (start % id)))
		.min_by_key(|&(_, val)| val)
		.map(|(id, val)| id * val)
		.unwrap()
}

fn part_two() -> usize {
	let t = include_str!("../input/input.txt");

	let (_, times) = t.split_once('\n').unwrap();
	// let start = start.parse::<usize>().unwrap();

	let ids: Vec<_> = times.split(',')
		.enumerate()
		.filter_map(|(i, v)| v.parse::<usize>().ok().map(|v| (i, v)))
		.collect();

	let mut time = ids.first().map(|&(_, id)| id).unwrap();
	let mut step = time;

	for (index, id) in ids.into_iter().skip(1) {
		for time_slot in (time..).step_by(step) {
			if (time_slot + index) % id == 0 {
				time = time_slot;
				step *= id;
				break;
			}
		}
	}

	time
}