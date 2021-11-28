fn main() {
	if let Err(err) = try_main() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
	let mut numbers = read_numbers()?;
	numbers.sort_unstable();

	if let Some((l, r)) = find_pair(&numbers) {
		println!("Part One: {:>4} * {:>4}        = {}", l, r, l * r);
	} else {
		eprintln!("Unable to find pair.");
	}

	if let Some((l, m, r)) = find_triple(&numbers) {
		println!("Part Two: {:>4} * {:>4} * {:>4} = {}", l, m, r, l * m * r);
	} else {
		eprintln!("Unable to find triple.");
	}

	Ok(())
}

fn find_pair(input: &[u32]) -> Option<(u32, u32)> {
	find_match(input, 2020)
}

fn find_triple(input: &[u32]) -> Option<(u32, u32, u32)> {
	for num in input {
		let sum = 2020 - *num;
		if let Some(factors) = find_match(input, sum) {
			return Some((*num, factors.0, factors.1))
		}
	}
	None
}

fn find_match(input: &[u32], sum: u32) -> Option<(u32, u32)> {
	// a + b = sum
	// sum - a = b

	for num in input {
		let num = *num;
		if sum < num {
			break;
		}

		let b = sum - num;

		if let Ok(_) = input.binary_search(&b) {
			return Some((num, b));
		}
	}

	None
}

fn read_numbers() -> Result<Vec<u32>, std::num::ParseIntError> {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| line.parse())
		.collect()
}
