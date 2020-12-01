fn main() {
	if let Err(err) = try_main() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
	let numbers = read_numbers()?;

	'part_one:
	for (l_i, l) in numbers.iter().enumerate() {
		for (r_i, r) in numbers.iter().enumerate() {
			if r_i > l_i && *l + *r == 2020 {
				println!("Part One: {}, {} = {}", l, r, l * r);
				break 'part_one;
			}
		}
	}

	'part_two:
	for (l_i, l) in numbers.iter().enumerate() {
		for (m_i, m) in numbers.iter().enumerate() {
			for (r_i, r) in numbers.iter().enumerate() {
				if r_i > m_i && m_i > l_i && *l + *m + *r == 2020 {
					println!("Part Two: {} * {} * {} = {}", l, m, r, *l * *m * *r);
					break 'part_two;
				}
			}
		}
	}

	Ok(())
}

fn read_numbers() -> Result<Vec<u32>, std::num::ParseIntError> {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| line.parse())
		.collect()
}
