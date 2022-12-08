use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one,
        vec![
            (TEST_1, 15),
            (INPUT, 13565),
        ],
    );
    aoc(part_two,
        vec![
            (TEST_1, 12),
            (INPUT, 12424),
        ],
    );
}

fn handle_input(input: &str, func: impl Fn(u8, u8) -> u8) -> u64 {
    input.lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(left, right)| {
            let left = left.as_bytes()[0];
            let right = right.as_bytes()[0];
            func(left, right) as u64
        })
        .sum::<u64>()
}

const LEFT_ROCK: u8 = b'A';
const LEFT_PAPER: u8 = b'B';
const LEFT_SCISSORS: u8 = b'C';

const SCORE_ROCK: u8 = 1;
const SCORE_PAPER: u8 = 2;
const SCORE_SCISSORS: u8 = 3;

const SCORE_LOSE: u8 = 0;
const SCORE_DRAW: u8 = 3;
const SCORE_WIN: u8 = 6;

fn part_one(input: &str) -> u64 {
    const RIGHT_ROCK: u8 = b'X';
    const RIGHT_PAPER: u8 = b'Y';
    const RIGHT_SCISSORS: u8 = b'Z';

    handle_input(input, |left, right| {
        match (left, right) {
            (LEFT_ROCK, RIGHT_ROCK) => SCORE_DRAW + SCORE_ROCK,
            (LEFT_PAPER, RIGHT_ROCK) => SCORE_LOSE + SCORE_ROCK,
            (LEFT_SCISSORS, RIGHT_ROCK) => SCORE_WIN + SCORE_ROCK,

            (LEFT_ROCK, RIGHT_PAPER) => SCORE_WIN + SCORE_PAPER,
            (LEFT_PAPER, RIGHT_PAPER) => SCORE_DRAW + SCORE_PAPER,
            (LEFT_SCISSORS, RIGHT_PAPER) => SCORE_LOSE + SCORE_PAPER,

            (LEFT_ROCK, RIGHT_SCISSORS) => SCORE_LOSE + SCORE_SCISSORS,
            (LEFT_PAPER, RIGHT_SCISSORS) => SCORE_WIN + SCORE_SCISSORS,
            (LEFT_SCISSORS, RIGHT_SCISSORS) => SCORE_DRAW + SCORE_SCISSORS,
            _ => panic!()
        }
    })
}

fn part_two(input: &str) -> u64 {
    const RIGHT_LOSE: u8 = b'X';
    const RIGHT_DRAW: u8 = b'Y';
    const RIGHT_WIN: u8 = b'Z';

    handle_input(input, |left, right| {
        match (left, right) {
            (LEFT_ROCK, RIGHT_LOSE) => SCORE_LOSE + SCORE_SCISSORS,
            (LEFT_PAPER, RIGHT_LOSE) => SCORE_LOSE + SCORE_ROCK,
            (LEFT_SCISSORS, RIGHT_LOSE) => SCORE_LOSE + SCORE_PAPER,

            (LEFT_ROCK, RIGHT_DRAW) => SCORE_DRAW + SCORE_ROCK,
            (LEFT_PAPER, RIGHT_DRAW) => SCORE_DRAW + SCORE_PAPER,
            (LEFT_SCISSORS, RIGHT_DRAW) => SCORE_DRAW + SCORE_SCISSORS,

            (LEFT_ROCK, RIGHT_WIN) => SCORE_WIN + SCORE_PAPER,
            (LEFT_PAPER, RIGHT_WIN) => SCORE_WIN + SCORE_SCISSORS,
            (LEFT_SCISSORS, RIGHT_WIN) => SCORE_WIN + SCORE_ROCK,
            _ => panic!()
        }
    })
}
