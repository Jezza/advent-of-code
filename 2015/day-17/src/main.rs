use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![((TEST_1, 25), 4), ((INPUT, 150), 654)]);
    aoc(part_two, vec![((TEST_1, 25), 3), ((INPUT, 150), 57)]);
}

type Num = u64;

fn solve(input: &str, target: Num, mut handler: impl FnMut(&[Num])) {
    let mut numbers: Vec<Num> = input.lines().filter_map(|line| line.parse().ok()).collect();
    numbers.sort_unstable();

    let bits = numbers.len();
    let mask = (0..bits).into_iter().map(|bit| 1 << bit).sum();

    let mut active_numbers = vec![];
    let mut sum = 0;

    macro_rules! reset {
        () => {{
            sum = 0;
            active_numbers.clear();
        }};
    }

    'mask_loop: for mask in 1..=mask {
        for (i, v) in numbers.iter().enumerate() {
            if ((1 << i) & mask) == 0 {
                continue;
            }

            sum += *v;
            if sum > target {
                reset!();
                continue 'mask_loop;
            }
            active_numbers.push(*v);
        }

        if sum == target {
            handler(&active_numbers);
        }

        reset!();
    }
}

fn part_one((input, number): (&str, Num)) -> u64 {
    let mut count = 0;
    solve(input, number, |_| count += 1);
    count
}

// fn part_one((input, number): (&str, Num)) -> u64 {
// 	let mut numbers: Vec<Num> = input.lines()
// 		.filter_map(|line| line.parse().ok())
// 		.collect();
// 	numbers.sort_unstable();
//
// 	let bits = numbers.len();
// 	let mask = (0..bits).into_iter()
// 		.map(|bit| 1 << bit)
// 		.sum();
//
// 	let mut count = 0;
// 	for mask in 1..=mask {
//
// 		let sum = numbers.iter()
// 			.enumerate()
// 			.filter(|(i, _)| ((1 << i) & mask) != 0)
// 			.map(|(_, v)| *v)
// 			.sum::<Num>();
// 		if sum == number {
// 			count += 1;
// 		}
// 	}
//
// 	count
// }

fn part_two((input, number): (&str, Num)) -> usize {
    let mut lengths = vec![];
    solve(input, number, |numbers| lengths.push(numbers.len()));

    let min = *lengths.iter().min().unwrap();

    lengths.iter().filter(|v| **v == min).count()
}
