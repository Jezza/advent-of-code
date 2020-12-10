
const CARGO_TOML: &str = r#"
[package]
name = "{}"
version = "0.1.0"
authors = ["Jezza <jezzadabomb@gmail.com>"]
edition = "2018"

[dependencies]
helper = { path = "../helper" }
"#;

const MAIN_RS: &str = r#"
use helper::measure;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	0
}

fn part_two() -> usize {
	0
}
"#;


fn main() {
	// println!("Hello, World!");
}

