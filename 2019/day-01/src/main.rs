use commons::*;

const INPUT: &str = {
	include_str!("../input/input.txt")
};

fn main() {
	measure!(part_one());
	println!("Part One: {}", part_one());

	measure!(part_two());
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
