use std::cell::Cell;
use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const TEST_2: &str = include_str!("../input/test-2.txt");
	const TEST_3: &str = include_str!("../input/test-3.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 10),
			(TEST_2, 19),
			(TEST_3, 226),
			(INPUT, 3779),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 36),
			(TEST_2, 103),
			(TEST_3, 3509),
			(INPUT, 96988),
		],
	);
}

type Node<'a> = (Vec<usize>, Cell<i16>, &'a str);

fn read_input(input: &str) -> (Vec<Node>, usize) {
	let mut nodes: Vec<Node> = vec![];

	macro_rules! find_or_insert {
	    ($name:ident) => {{
			 nodes.iter()
				.position(|t| t.2 == $name)
				.unwrap_or_else(|| {
					let revisit = $name.as_bytes().iter().any(u8::is_ascii_uppercase);
					let count = if revisit { -1 } else { 1 };
					nodes.push((vec![], Cell::new(count), $name));
					nodes.len() - 1
			 	})
		}};
	}

	input.lines()
		.filter_map(|line| line.split_once("-"))
		.for_each(|(source, target)| {
			let source_node = find_or_insert!(source);
			let target_node = find_or_insert!(target);

			nodes[source_node].0.push(target_node);
			nodes[target_node].0.push(source_node);
		});

	let start_node = nodes.iter()
		.position(|t| t.2 == "start")
		.unwrap();

	(nodes, start_node)
}

fn dfs(nodes: &[Node], id: usize, revisits: usize) -> usize {
	let (edges, visits, name) = &nodes[id];

	match (*name, visits.get(), revisits) {
		("start" | "end", 0, _) | (_, 0, 0) => 0,
		(_, n, mut revisits) => {
			match n {
				0 => revisits -= 1,
				_ => visits.set(n - 1),
			}
			let paths = edges.iter()
				.map(|id| dfs(nodes, *id, revisits))
				.sum::<usize>();
			if n != 0 {
				visits.set(n);
			}

			paths + if *name == "end" { 1 } else { 0 }
		}
	}
}

fn part_one(input: &str) -> usize {
	let (nodes, start_node) = read_input(input);
	dfs(&nodes, start_node, 0)
}

fn part_two(input: &str) -> usize {
	let (nodes, start_node) = read_input(input);
	dfs(&nodes, start_node, 1)
}