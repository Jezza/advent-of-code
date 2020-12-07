#![feature(test)]
#![feature(str_split_once)]

extern crate test;

use std::collections::HashMap;

macro_rules! measure {
    ($expr:expr) => {{
    	let stats = test::bench::iter(&mut || $expr);
		let median = stats.median as usize;
		let deviation = (stats.max - stats.min) as usize;
		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($expr), median, deviation);
		$expr
    }};
}

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

type Graph = HashMap<&'static str, Vec<(usize, &'static str)>>;

#[cfg(not(feature = "cached"))]
fn contains(graph: &Graph, key: &str) -> bool {
	key == "shiny gold" || graph[key].iter()
		.any(|(_, k)| contains(graph, k))
}

#[cfg(feature = "cached")]
fn contains_cached(cache: &mut HashMap<&'static str, bool>, graph: &Graph, key: &'static str) -> bool {
	if !cache.contains_key(key) {
		let result = graph[key].iter().any(|(_, k)| contains_cached(cache, graph, k));
		cache.insert(key, result);
	}
	cache[key]
}

fn count_bags(graph: &Graph, item: &str) -> usize {
	graph[item].iter()
		.map(|(count, item)| count * count_bags(graph, item))
		.sum::<usize>() + 1
}

#[cfg(not(feature = "cached"))]
fn part_one() -> usize {
	let graph = &read_input();

	graph.keys()
		.filter(|k| contains(graph, k))
		.count() - 1 // remove one, as when we hit shiny-gold, we count that as a bag that can hold itself.
}

#[cfg(feature = "cached")]
fn part_one() -> usize {
	let graph = read_input();

	let mut cache = HashMap::new();
	cache.insert("shiny gold", true);

	graph.keys()
		.filter(|k| contains_cached(&mut cache, &graph, k))
		.count() - 1 // remove one, as when we hit shiny-gold, we count that as a bag that can hold itself.
}

fn part_two() -> usize {
	let graph = &read_input();
	// Off by one, as we're adding one when we're counting the bags from shiny gold.
	count_bags(graph, "shiny gold") - 1
}

fn read_input() -> Graph {
	let graph = include_str!("../input/input.txt")
		.lines()
		.map(parse_line)
		.collect();
	// println!("{:#?}", graph);
	graph
}

fn parse_line(input: &str) -> (&str, Vec<(usize, &str)>) {
	let (key, contents) = input.split_once(" bags contain ")
		.unwrap();

	if contents == "no other bags." {
		return (key, vec![]);
	}

	let items: Vec<(usize, &str)> = contents.split(", ")
		.map(|item| {
			let start = item.find(' ').unwrap();
			let end = item.rfind(' ').unwrap();
			let count = item[..start].parse().unwrap();
			let bag = &item[start + 1..end];
			(count, bag)
		})
		.collect();

	(key, items)
}

