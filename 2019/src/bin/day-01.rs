const INPUT: &str = include_str!("../../input/day-01.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> u64 {
	INPUT.lines()
		.filter_map(|line| line.parse::<u64>().ok())
		.map(|v| v / 3 - 2)
		.sum()
}

fn part_two() -> u64 {
	INPUT.lines()
		.filter_map(|line| line.parse::<u64>().ok())
		.map(|mut v| {
			let mut sum = 0;
			loop {
				let x = (v / 3).saturating_sub(2);
				if x <= 0 {
					break;
				}
				sum += x;
				v = x;
			}
			sum
		})
		.sum()
}
