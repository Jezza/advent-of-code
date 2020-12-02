
fn main() {
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	process_inputs(|min, max, char, password| {
		let count = password.iter()
			.filter(|c|  **c == char)
			.count();

		min <= count && count <= max
	})
}

fn part_two() -> usize {
	let handler = |pos_one, pos_two, char, password: &'static [u8]| (password[pos_one - 1] == char) ^ (password[pos_two - 1] == char);
	process_inputs(handler)
}

fn process_inputs(mut handler: impl FnMut(usize, usize, u8, &'static [u8]) -> bool) -> usize {
	let input = include_str!("../input/input.txt");

	let regex = regex::RegexBuilder::new(r"(\d+)\-(\d+) (\w): (\w+?)$")
		.multi_line(true)
		.unicode(false)
		.build()
		.unwrap();

	regex.captures_iter(input)
		.filter(|cap| {
			let min: usize = cap.get(1).unwrap().as_str().parse().unwrap();
			let max: usize = cap.get(2).unwrap().as_str().parse().unwrap();
			let character: char = cap.get(3).unwrap().as_str().parse().unwrap();
			let password = cap.get(4).unwrap().as_str().as_bytes();

			handler(min, max, character as u8, password)
		})
		.count()
}
