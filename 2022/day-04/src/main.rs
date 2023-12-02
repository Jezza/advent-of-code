use commons::parse::Parse;
use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 2), (INPUT, 569)]);
    aoc(part_two, vec![(TEST_1, 4), (INPUT, 936)]);
}

#[derive(Debug, Copy, Clone)]
struct Span {
    lower: u16,
    upper: u16,
}

impl Span {
    fn contains(self, other: Self) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn intersects(self, other: Self) -> bool {
        self.lower <= other.lower && self.upper >= other.lower
    }
}

impl<'a> Parse<'a> for Span {
    type Error = std::convert::Infallible;

    fn from_str(input: &'a str) -> Result<Self, Self::Error> {
        let (lower, upper) = split_parse!(input, "-");
        Ok(Self { lower, upper })
    }
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let (left, right): (Span, Span) = split_parse!(line, ",");
            left.contains(right) || right.contains(left)
        })
        .count() as u64
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| {
            let (left, right): (Span, Span) = split_parse!(line, ",");
            left.intersects(right) || right.intersects(left)
        })
        .count() as u64
}
