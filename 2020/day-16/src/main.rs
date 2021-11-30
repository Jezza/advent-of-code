#![feature(str_split_once)]

use std::collections::HashMap;
use std::ops::RangeInclusive;

use commons::measure;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	let mut it = include_str!("../input/input.txt")
		.split("\n\n");

	let rules = it.next().unwrap();
	let my_ticket = it.next().unwrap();
	let nearby_tickets = it.next().unwrap();

	let rules = parse_rules(rules);
	let _my_ticket = parse_ticket(my_ticket.split_once("\n").unwrap().1);
	let nearby_tickets = nearby_tickets.lines()
		.skip(1)
		.map(|ticket| parse_ticket(ticket))
		.collect::<Tickets>();

	nearby_tickets.iter()
		.filter_map(|ticket| {
			ticket.iter()
				.find(|v| rules.iter().all(|(_, lower, upper)| !(lower.contains(v) || upper.contains(v))))
		})
		.fold(0usize, |acc, ticket| acc + *ticket)
}

type Rule = (&'static str, RangeInclusive<usize>, RangeInclusive<usize>);
type Ticket = Box<[usize]>;
type Tickets = Vec<Ticket>;

fn parse_rules(input: &'static str) -> Box<[Rule]> {
	input.lines()
		.map(|line| {
			let (name, rules) = line.split_once(": ").unwrap();
			let (lower, upper) = rules.split_once(" or ").unwrap();

			let lower = parse_range(lower);
			let upper = parse_range(upper);

			(name, lower, upper)
		})
		.collect()
}

fn parse_range(input: &'static str) -> RangeInclusive<usize> {
	let (lower_bound, upper_bound) = input.split_once("-").unwrap();
	let lower_bound = lower_bound.parse().unwrap();
	let upper_bound = upper_bound.parse().unwrap();

	lower_bound..=upper_bound
}

fn parse_ticket(input: &'static str) -> Ticket {
	input.split(",")
		.filter_map(|v| v.parse::<usize>().ok())
		.collect()
}

fn part_two() -> usize {
	let mut it = include_str!("../input/input.txt")
		.split("\n\n");

	let rules = it.next().unwrap();
	let my_ticket = it.next().unwrap();
	let nearby_tickets = it.next().unwrap();

	let rules = parse_rules(rules);
	let my_ticket = parse_ticket(my_ticket.split_once("\n").unwrap().1);
	let mut nearby_tickets = nearby_tickets.lines()
		.skip(1)
		.map(|ticket| parse_ticket(ticket))
		.filter(|ticket| {
			ticket.iter()
				.all(|v| rules.iter().any(|(_, lower, upper)| {
					lower.contains(v) || upper.contains(v)
				}))
		})
		.collect::<Tickets>();

	let cols = my_ticket.len();

	nearby_tickets.push(my_ticket.clone());

	let mut all_possible_rules = (0..cols).map(|col| {
		rules.iter()
			.enumerate()
			.filter(|&(_, &(_, ref lower, ref upper))| {
				nearby_tickets.iter()
					.map(|ticket| ticket[col])
					.all(|v| lower.contains(&v) || upper.contains(&v))
			})
			.collect::<Vec<_>>()
	}).collect::<Vec<_>>();

	let mut assigned_columns = vec![None; cols];
	while let Some(i) = all_possible_rules.iter().position(|s| s.len() == 1) {
		let (col, rule) = *all_possible_rules[i].first().unwrap();
		assigned_columns[i] = Some(rule);
		all_possible_rules.iter_mut()
			.for_each(|s| s.retain(|(v, _)| *v != col));
	}

	assigned_columns.into_iter()
		.enumerate()
		.filter_map(|(i, opt)| opt.map(|rule| (i, rule)))
		.filter(|(_, &(name, ..))| name.starts_with("departure"))
		.map(|(i, _)| my_ticket[i])
		.product()
}
