#![feature(array_windows)]
#![feature(split_array)]

use commons::*;
use std::collections::HashSet;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 13), (INPUT, 6175)]);
    aoc(part_two, vec![(TEST_1, 1), (INPUT, 2578)]);
}

pub fn for_each_window_mut<const N: usize, T, F>(slice: &mut [T], mut function: F)
where
    F: FnMut(&mut [T; N]),
{
    for start in 0..=(slice.len().saturating_sub(N)) {
        let slice = &mut slice[start..];
        let (slice, _) = slice.split_array_mut::<N>();
        function(slice);
    }
}

fn print_snek(snek: &[(i32, i32)]) {
    let mut buffer = String::new();

    for y in -6..6 {
        for x in -6..6 {
            let mut contains = false;
            for (i, segment) in snek.iter().enumerate() {
                if segment.0 == x && segment.1 == y {
                    if i == 0 {
                        buffer.push('H');
                    } else if i == snek.len() - 1 {
                        buffer.push('T');
                    } else {
                        buffer.push_str(&format!("{}", i));
                    }
                    contains = true;
                    break;
                }
            }
            if !contains {
                buffer.push('.');
            }
        }

        buffer.push('\n');
    }

    println!("{}", buffer);
}

fn handle_input<const N: usize>(input: &str) -> u64 {
    let mut snek = [(0i32, 0i32); N];

    // print_snek(&snek);

    input
        .lines()
        .flat_map(|line| {
            let (direction, count): (&str, i32) = split_parse!(line, " ");
            let offset = match direction {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!("Unknown movement: {}", direction),
            };
            std::iter::repeat(offset).take(count as usize)
        })
        .map(|(dx, dy)| {
            snek[0].0 += dx;
            snek[0].1 += dy;
            // println!("Moved head ({}, {})", dx, dy);

            // for [left, right] in snek.array_windows_mut::<2>()
            for_each_window_mut(&mut snek, |[left, right]| {
                let dx = left.0 - right.0;
                let dy = left.1 - right.1;

                // println!("Offset ({}, {})", dx, dy);

                if dx.abs() < 2 && dy.abs() < 2 {
                    return;
                }

                right.0 += dx.signum();
                right.1 += dy.signum();
            });

            // println!("==============================");
            // print_snek(&snek);

            snek.last().copied().unwrap()
        })
        .collect::<HashSet<_>>()
        .len() as u64
}

fn part_one(input: &str) -> u64 {
    handle_input::<2>(input)
}

fn part_two(input: &str) -> u64 {
    handle_input::<10>(input)
}
