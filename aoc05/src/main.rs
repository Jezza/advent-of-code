
fn main() {
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	read_ids()
		.max()
		.unwrap()
}

fn part_two() -> usize {
	let mut all = read_ids()
		.collect::<Vec<usize>>();

	all.sort_unstable();
	all.windows(2)
		.find(|slice| slice[0] + 1 != slice[1])
		.unwrap()[0] + 1usize
}

fn read_ids() -> impl Iterator<Item = usize> {
	include_str!("../input/input.txt")
		.lines()
		.map(|l| l.as_bytes().iter().fold(0, |acc, half| {
			(acc << 1) | if *half == b'B' || *half == b'R' { 1 } else { 0 }
		}))
}
