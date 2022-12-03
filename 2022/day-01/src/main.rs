use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one,
        vec![
            (TEST_1, 24000),
            (INPUT, 68442),
        ],
    );
    aoc(part_two,
        vec![
            (TEST_1, 45000),
            (INPUT, 204837),
        ],
    );
    aoc(aoc_2022_01::part_one,
        vec![
            (TEST_1, 24000),
            (INPUT, 68442),
        ],
    );
}

fn sums(input: &str) -> impl Iterator<Item=u64> + '_ {
    input.split("\n\n")
        .map(|group| group.lines().filter_map(|line| line.parse::<u64>().ok()).sum())
}

fn part_one(input: &str) -> u64 {
    sums(input)
        .max()
        .unwrap()
}

fn part_two(input: &str) -> u64 {
    let mut values: Vec<_> = sums(input)
        .collect();

    values.sort_unstable();
    values[values.len() - 3..].iter()
        .sum::<u64>()
}
