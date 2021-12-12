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

type Node<'a> = (Vec<usize>, bool, &'a str);

fn parse_input(input: &str) -> Vec<Node> {
	let mut nodes: Vec<Node> = vec![];

	macro_rules! find_or_insert {
	    ($name:ident) => {{
			 nodes.iter()
				.position(|t| t.2 == $name)
				.unwrap_or_else(|| {
					let big = $name.as_bytes().iter().any(u8::is_ascii_uppercase);
					nodes.push((vec![], big, $name));
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

	nodes
}

fn dfs(nodes: &[Node], revisit: bool) -> usize {
	assert!(nodes.len() < 32, "Pick a bigger bitset...");
	let start_node = nodes.iter()
		.position(|t| t.2 == "start")
		.unwrap();
	let end_node = nodes.iter()
		.position(|t| t.2 == "end")
		.unwrap();

	let mut count = 0;
	let mut stack = vec![];
	stack.push((start_node, 1 << start_node, false));

	while let Some((id, mask, double)) = stack.pop() {
		let (edges, _, _) = &nodes[id];
		for edge in edges {
			let edge = *edge;
			if edge == start_node {
				continue;
			}
			if edge == end_node {
				count += 1;
				continue;
			}
			let (_, big, _) = nodes[edge];
			if big || (1 << edge) & mask == 0 {
				stack.push((edge, mask | (1 << edge), double));
			} else if revisit && !double {
				stack.push((edge, mask | (1 << edge), true));
			}
		}
	}

	count
}

fn part_one(input: &str) -> usize {
	dfs(&parse_input(input), false)
}

fn part_two(input: &str) -> usize {
	dfs(&parse_input(input), true)
}