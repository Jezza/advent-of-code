#![feature(box_patterns)]
#![feature(let_else)]

use commons::*;

use commons::ext::OptionExt;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const TEST_2: &str = include_str!("../input/test-2.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 1384), (INPUT, 4235)]);
    aoc(part_two, vec![(TEST_2, 3993), (INPUT, 4659)]);
}

#[derive(Debug, Copy, Clone)]
enum Symbol {
    LeftBracket,
    RightBracket,
    Comma,
    Value(u8),
}

fn display(symbols: &[Symbol]) -> String {
    let mut out = String::new();

    for symbol in symbols {
        out += match symbol {
            Symbol::LeftBracket => "[ ",
            Symbol::RightBracket => " ]",
            Symbol::Comma => ", ",
            Symbol::Value(value) => {
                use std::fmt::Write;
                write!(out, "{}", value).unwrap();
                continue;
            }
        };
    }

    out
}

fn parse_input(input: &str) -> Vec<Symbol> {
    input
        .bytes()
        .map(|b| match b {
            b'[' => Symbol::LeftBracket,
            b']' => Symbol::RightBracket,
            b',' => Symbol::Comma,
            b'0'..=b'9' => Symbol::Value(b - b'0'),
            _ => panic!("Unknown symbol: {}", b),
        })
        .collect()
}

fn explode(symbols: &mut Vec<Symbol>) -> bool {
    let mut depth = 0;
    let pos = symbols.iter().position(|symbol| {
        match symbol {
            Symbol::LeftBracket => depth += 1,
            Symbol::RightBracket => depth -= 1,
            Symbol::Value(_) if depth > 4 => {
                return true;
            }
            _ => (),
        }
        false
    });

    let Some(pos) = pos else {
        // println!("No exploding...");
        return false;
    };

    let start = pos - 1;
    let end = pos + 4;

    // println!("{}", display(&symbols[start..end]));

    let Symbol::Value(left) = &symbols[pos] else {
        panic!("Invalid position: {}", pos);
    };
    let Symbol::Value(right) = &symbols[pos + 2] else {
        panic!("Invalid position: {}", pos);
    };

    let left = *left;
    let right = *right;

    symbols[..start]
        .iter_mut()
        .rev()
        .find_map(|item| {
            if let Symbol::Value(value) = item {
                Some(value)
            } else {
                None
            }
        })
        .if_present(|previous| *previous += left);

    symbols[end..]
        .iter_mut()
        .find_map(|item| {
            if let Symbol::Value(value) = item {
                Some(value)
            } else {
                None
            }
        })
        .if_present(|next| *next += right);

    symbols[start] = Symbol::Value(0);
    symbols.drain(pos..end);

    true
}

fn split(symbols: &mut Vec<Symbol>) -> bool {
    let matched_symbol = symbols.iter().enumerate().find_map(|(pos, symbol)| {
        let Symbol::Value(value) = symbol else {
            return None;
        };
        let value = *value;
        if value < 10 {
            return None;
        }

        let left = value / 2;
        let right = value - left;

        Some((pos, left, right))
    });
    let Some((pos, left, right)) = matched_symbol else {
        // println!("No splitting...");
        return false;
    };
    // println!("Splitting!");

    let replacement = [
        Symbol::LeftBracket,
        Symbol::Value(left),
        Symbol::Comma,
        Symbol::Value(right),
        Symbol::RightBracket,
    ];

    // println!("{}", display(&symbols));
    symbols.splice(pos..=pos, replacement);
    // println!("{}", display(&symbols));

    true
}

fn reduce(symbols: &mut Vec<Symbol>) {
    while explode(symbols) || split(symbols) {
        // println!("{}", display(&symbols));
    }
}

fn magnitude(symbols: &[Symbol]) -> u64 {
    let mut multiplier = 1;
    let mut count = 0;
    for symbol in symbols {
        match symbol {
            Symbol::LeftBracket => multiplier *= 3,
            Symbol::RightBracket => multiplier /= 2,
            Symbol::Value(num) => count += multiplier * *num as u64,
            Symbol::Comma => multiplier = (multiplier / 3) * 2,
        }
    }
    count
}

fn part_one(input: &str) -> u64 {
    let symbols = input
        .lines()
        .map(parse_input)
        .reduce(|mut acc, symbols| {
            acc.insert(0, Symbol::LeftBracket);
            acc.push(Symbol::Comma);
            acc.extend(symbols);
            acc.push(Symbol::RightBracket);
            reduce(&mut acc);
            acc
        })
        .unwrap();

    // println!("{}", display(&symbols));
    magnitude(&symbols)
}

fn part_two(input: &str) -> u64 {
    let symbols: Vec<_> = input.lines().map(parse_input).collect();

    let mut scratch_space = Vec::with_capacity(symbols.len() * 2);

    let mut max = u64::MIN;

    // Ew...
    for (first_index, first) in symbols.iter().enumerate() {
        for (second_index, second) in symbols.iter().enumerate() {
            if first_index == second_index {
                continue;
            }

            scratch_space.clear();

            scratch_space.push(Symbol::LeftBracket);
            scratch_space.extend(first.iter());
            scratch_space.push(Symbol::Comma);
            scratch_space.extend(second.iter());
            scratch_space.push(Symbol::RightBracket);

            reduce(&mut scratch_space);

            let value = magnitude(&scratch_space);
            max = max.max(value);
        }
    }

    max
}
