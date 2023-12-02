use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(
        part_one,
        vec![
            (TEST_1, String::from("CMZ")),
            (INPUT, String::from("MQTPGLLDN")),
        ],
    );
    aoc(
        part_two,
        vec![
            (TEST_1, String::from("MCD")),
            (INPUT, String::from("LVZPSTTCZ")),
        ],
    );
}

type Crate = char;

fn handle_input(input: &str, func: impl Fn(&mut Vec<Crate>)) -> String {
    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut it = crates.lines().rev();

    let count = it.next().unwrap().bytes().filter(|c| *c != b' ').count();

    let mut stacks = vec![Vec::new(); count];

    it.for_each(|line| {
        let input = line.bytes().skip(1).step_by(4);

        stacks
            .iter_mut()
            .zip(input)
            .filter(|(_, c)| *c != b' ')
            .for_each(|(stack, c)| {
                stack.push(c as Crate);
            });
    });

    for op in moves.lines() {
        let (_, count, from, to): (Ignored, usize, usize, usize) =
            split_parse!(op, "move", "from", "to");

        let stack = &mut stacks[from - 1];
        let start = stack.len() - count;
        let mut values: Vec<_> = stack.drain(start..).collect();

        func(&mut values);

        stacks[to - 1].extend_from_slice(&values);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.last())
        .copied()
        .collect()
}

fn part_one(input: &str) -> String {
    handle_input(input, |values| {
        values.reverse();
    })
}

fn part_two(input: &str) -> String {
    handle_input(input, |_stacks| ())
}
