#![feature(array_windows)]

use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const TEST_2: &str = include_str!("../input/test-2.txt");
    const TEST_3: &str = include_str!("../input/test-3.txt");
    const TEST_4: &str = include_str!("../input/test-4.txt");
    const TEST_5: &str = include_str!("../input/test-5.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(
        part_one,
        vec![
            (TEST_1, 7),
            (TEST_2, 5),
            (TEST_3, 6),
            (TEST_4, 10),
            (TEST_5, 11),
            (INPUT, 1987),
        ],
    );
    aoc(
        part_two,
        vec![
            (TEST_1, 19),
            (TEST_2, 23),
            (TEST_3, 23),
            (TEST_4, 29),
            (TEST_5, 26),
            (INPUT, 3059),
        ],
    );
}

fn handle_input<const N: usize>(input: &str) -> u64 {
    (input
        .as_bytes()
        .array_windows()
        .position(|window: &[u8; N]| {
            // @FIXME jezza - 06 Dec 2022: There's a better way to do this.
            //  We just need to prime the counts, then we only care about new values, and last values.

            let mut counts = 0u32;
            for c in window {
                let mark = counts;
                counts |= 1 << (*c - b'a') as usize;
                if mark == counts {
                    return false;
                }
            }
            true
        })
        .unwrap()
        + N) as u64
}

fn part_one(input: &str) -> u64 {
    handle_input::<4>(input)
}

fn part_two(input: &str) -> u64 {
    handle_input::<14>(input)
}
