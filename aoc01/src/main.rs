fn main() {
	if let Err(err) = try_main() {
		eprintln!("Application experienced an error: {}", err);
		std::process::exit(1);
	}
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
	let numbers = read_numbers()?;

	'part_one:
	for l in &numbers {
		for r in &numbers {
			if *l + *r == 2020 {
				println!("Part One: {}", l * r);
				break 'part_one;
			}
		}
	}

	'part_two:
	for l in &numbers {
		for m in &numbers {
			for r in &numbers {
				if *l + *m + *r == 2020 {
					println!("Part Two: {}", *l * *m * *r);
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
