use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const TEST_2: &str = include_str!("../input/test-2.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one,
        vec![
            (TEST_1, 142),
            (INPUT, 54388),
        ],
    );
    aoc(part_two,
        vec![
            (TEST_1, 142),
            (TEST_2, 381),
            (INPUT, 53515),
        ],
    );
}

fn part_one(input: &str) -> u64 {
    input.lines()
        .filter_map(|line| {
            let left = line.chars().find(|c| c.is_digit(10))? as u8 - b'0';
            let right = line.chars().rfind(|c| c.is_digit(10))? as u8 - b'0';
            let value = left * 10 + right;
            Some(value as u64)
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    input.lines()
        .filter_map(|line| {
            let left: u8 = find_num(line);
            let right: u8 = rfind_num(line);
            let value = left * 10 + right;
            Some(value as u64)
        })
        .sum()
}

fn find_num(input: &str) -> u8 {
    let mut input = input.as_bytes();

    loop {
        break match input {
            [b'1', ..] | [b'o', b'n', b'e', ..] => 1,
            [b'2', ..] | [b't', b'w', b'o', ..] => 2,
            [b'3', ..] | [b't', b'h', b'r', b'e', b'e', ..] => 3,
            [b'4', ..] | [b'f', b'o', b'u', b'r', ..] => 4,
            [b'5', ..] | [b'f', b'i', b'v', b'e', ..] => 5,
            [b'6', ..] | [b's', b'i', b'x', ..] => 6,
            [b'7', ..] | [b's', b'e', b'v', b'e', b'n', ..] => 7,
            [b'8', ..] | [b'e', b'i', b'g', b'h', b't', ..] => 8,
            [b'9', ..] | [b'n', b'i', b'n', b'e', ..] => 9,
            [_, rest @ ..] => {
                input = rest;
                continue;
            }
            [] => panic!(),
        };
    }
}

fn rfind_num(input: &str) -> u8 {
    let mut input = input.as_bytes();
    loop {
        break match input {
            [.., b'1'] | [.., b'o', b'n', b'e'] => 1,
            [.., b'2'] | [.., b't', b'w', b'o'] => 2,
            [.., b'3'] | [.., b't', b'h', b'r', b'e', b'e'] => 3,
            [.., b'4'] | [.., b'f', b'o', b'u', b'r'] => 4,
            [.., b'5'] | [.., b'f', b'i', b'v', b'e'] => 5,
            [.., b'6'] | [.., b's', b'i', b'x'] => 6,
            [.., b'7'] | [.., b's', b'e', b'v', b'e', b'n'] => 7,
            [.., b'8'] | [.., b'e', b'i', b'g', b'h', b't'] => 8,
            [.., b'9'] | [.., b'n', b'i', b'n', b'e'] => 9,
            [rest @ .., _] => {
                input = rest;
                continue;
            }
            [] => panic!(),
        };
    }
}
