use commons::*;
use std::cmp::Ordering;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 13), (INPUT, 5529)]);
    aoc(part_two, vec![(TEST_1, 140), (INPUT, 27690)]);
}

type Value = u8;

#[derive(Debug, Copy, Clone, logos::Logos, Ord, PartialOrd, Eq, PartialEq)]
enum Token {
    #[token("[")]
    ArrayStart,
    #[token("]")]
    ArrayEnd,
    #[regex("[0-9]+", | lex | lex.slice().parse())]
    Value(Value),
    #[regex(r"//.*\n?", logos::skip)]
    #[regex(r"[ \t\n\f,]+", logos::skip)]
    #[error]
    Error,
}

fn tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    <Token as logos::Logos>::lexer(input)
}

fn compare(
    left: impl IntoIterator<Item = Token>,
    right: impl IntoIterator<Item = Token>,
) -> Ordering {
    let mut left = left.into_iter();
    let mut right = right.into_iter();

    let mut left_extra = vec![];
    let mut right_extra = vec![];

    loop {
        let Some(left) = left_extra.pop().or_else(|| left.next()) else {
            return Ordering::Less;
        };
        let Some(right) = right_extra.pop().or_else(|| right.next()) else {
            return Ordering::Greater;
        };

        match (left, right) {
            (Token::ArrayStart, Token::ArrayStart) => (),
            (Token::ArrayEnd, Token::ArrayEnd) => (),
            (Token::ArrayStart, token @ Token::Value(_)) => {
                right_extra.push(Token::ArrayEnd);
                right_extra.push(token);
            }
            (token @ Token::Value(_), Token::ArrayStart) => {
                left_extra.push(Token::ArrayEnd);
                left_extra.push(token);
            }
            (Token::Value(left), Token::Value(right)) => match left.cmp(&right) {
                Ordering::Equal => (),
                ordering => {
                    return ordering;
                }
            },
            (Token::ArrayEnd, _) => {
                return Ordering::Less;
            }
            (_, Token::ArrayEnd) => {
                return Ordering::Greater;
            }
            (left, right) => {
                panic!("Unexpected combination: {:?} <=> {:?}", left, right);
            }
        }
    }
}

fn part_one(input: &str) -> u64 {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, line)| {
            let (left, right) = line.split_once("\n").unwrap();
            let left = tokens(left);
            let right = tokens(right);
            if compare(left, right) == Ordering::Less {
                // println!("Correct: {}", i + 1);
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>() as u64
}

fn part_two(input: &str) -> u64 {
    let mut items = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let tokens = tokens(line).collect::<Vec<_>>();
            (0, tokens)
        })
        .collect::<Vec<_>>();

    let two = tokens("[[2]]").collect::<Vec<_>>();
    let six = tokens("[[6]]").collect::<Vec<_>>();
    items.push((2, two.clone()));
    items.push((6, six.clone()));

    items.sort_unstable_by(|(_, left), (_, right)| {
        compare(left.iter().copied(), right.iter().copied())
    });

    let two = items.iter().position(|(item, _)| *item == 2).unwrap();
    let six = items
        .iter()
        .skip(two)
        .position(|(item, _)| *item == 6)
        .unwrap()
        + two;

    ((two + 1) * (six + 1)) as u64
}
