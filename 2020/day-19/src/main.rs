#![feature(str_split_once)]
#![feature(bool_to_option)]

use std::collections::{BTreeMap, VecDeque};

use commons::measure;
use commons::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
}

fn part_one() -> u64 {
	let (rules, input) = INPUT.split_once("\n\n").unwrap();

	let rules = rules.lines()
		.map(parse_rule)
		.collect::<BTreeMap<_, _>>();

	input.lines()
		.filter(|&line| {
			let mut stack = VecDeque::new();
			stack.push_front(0);
			check_rule(line, &rules, stack)
		})
		.count() as u64
}

fn check_rule(
	input: &'static str,
	rules: &BTreeMap<RuleId, Rule>,
	mut stack: VecDeque<RuleId>,
) -> bool {
	let rule_id = if let Some(rule_id) = stack.pop_front() {
		rule_id
	} else {
		return input == "";
	};

	let rule = &rules[&rule_id];

	// println!("{}R:{} on {} @ {:?}", " ".repeat(rule_id as usize), rule_id, input, stack);

	match rule {
		Rule::Literal(value) => {
			input.starts_with(*value as char)
				.then(|| &input[1..])
				.map(|input| check_rule(input, rules, stack))
				.unwrap_or(false)
		}
		Rule::Seq(seq) => {
			for id in seq.iter().rev() {
				stack.push_front(*id);
			}
			check_rule(input, rules, stack)
		},
		Rule::Or(left_seq, right_seq) => {
			let left_stack = {
				let mut stack = stack.clone();
				for id in left_seq.iter().rev() {
					stack.push_front(*id);
				}
				stack
			};
			let right_stack = {
				let mut stack = stack;
				for id in right_seq.iter().rev() {
					stack.push_front(*id);
				}
				stack
			};

			check_rule(input, rules, left_stack) || check_rule(input, rules, right_stack)
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
enum Rule {
	Literal(u8),
	Seq(Rules),
	Or(Rules, Rules),
}

type RuleId = u8;
type Rules = Box<[RuleId]>;

fn parse_rule(input: &str) -> (RuleId, Rule) {
	let (index, input) = input.split_once(": ").unwrap();
	let index = index.parse::<RuleId>().unwrap();

	if input.starts_with("\"") {
		let character = input.bytes()
			.nth(1)
			.unwrap();
		return (index, Rule::Literal(character));
	}

	if let Some((left, right)) = input.split_once(" | ") {
		// println!("{} OR {}", left, right);
		let left = parse_ids(left);
		let right = parse_ids(right);

		(index, Rule::Or(left, right))
	} else {
		// println!("{}", input);
		let ids = parse_ids(input);

		(index, Rule::Seq(ids))
	}
}

fn parse_ids(input: &str) -> Rules {
	input.split_ascii_whitespace()
		.filter_map(|branch| branch.parse::<RuleId>().ok())
		.collect()
}

fn part_two() -> u64 {
	let (rules, input) = INPUT.split_once("\n\n").unwrap();

	let mut rules = rules.lines()
		.map(parse_rule)
		.collect::<BTreeMap<RuleId, Rule>>();

	rules.insert(8, parse_rule("8: 42 | 42 8").1);
	rules.insert(11, parse_rule("11: 42 31 | 42 11 31").1);

	input.lines()
		.filter(|&line| {
			let mut stack = VecDeque::new();
			stack.push_front(0);
			check_rule(line, &rules, stack)
		})
		.count() as u64
}
