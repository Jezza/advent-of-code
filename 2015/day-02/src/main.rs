use commons::export::itertools::Itertools;
use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 150), (INPUT, 2070300)]);
    aoc(part_two, vec![(TEST_1, 900), (INPUT, 2078985210)]);
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let areas = line
                .splitn(3, "x")
                .filter_map(|value| value.parse::<u32>().ok())
                .combinations(2)
                .map(|x| x.iter().copied().product::<u32>())
                .collect_vec();

            let sum: u32 = areas.iter().copied().map(|x| x * 2).sum();

            let min: u32 = areas.iter().copied().min().unwrap_or_default();

            sum + min
        })
        .sum::<u32>() as u64
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let areas = line
                .splitn(3, "x")
                .filter_map(|value| value.parse::<u32>().ok())
                .combinations(2)
                .min();

            println!("{:#?}", areas);

            // let sum: u32 = areas.iter()
            // 	.copied()
            // 	.map(|x| x * 2)
            // 	.sum();
            //
            // let min: u32 = areas.iter()
            // 	.copied()
            // 	.min()
            // 	.unwrap_or_default();

            0
        })
        .sum()
}
