fn main() {
	if let Err(err) = try_main() {
		eprintln!("Error: {}", err);
		std::process::exit(1);
	}
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
	let numbers = read_numbers()?;

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
	for (l_i, l) in input.iter().enumerate() {
		for (r_i, r) in input.iter().enumerate() {
			if r_i > l_i && *l + *r == 2020 {
				return Some((*l, *r));
			}
		}
	}
	None
}

fn find_triple(input: &[u32]) -> Option<(u32, u32, u32)> {
	for (l_i, l) in input.iter().enumerate() {
		for (m_i, m) in input.iter().enumerate() {
			for (r_i, r) in input.iter().enumerate() {
				if r_i > m_i && m_i > l_i && *l + *m + *r == 2020 {
					return Some((*l, *m, *r));
				}
			}
		}
	}
	None
}

fn find_match() -> () {
}

fn read_numbers() -> Result<Vec<u32>, std::num::ParseIntError> {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| line.parse())
		.collect()
}
