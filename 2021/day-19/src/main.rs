#![feature(array_windows)]
#![feature(int_abs_diff)]
#![feature(vec_retain_mut)]

use std::collections::HashMap;

use commons::export::itertools::Itertools;
use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(
        part_one,
        vec![
            (TEST_1, 79), //  1,821,047 ns/iter (+/- 1014292)
            (INPUT, 451), // 97,546,830 ns/iter (+/- 6184379)
        ],
    );
    aoc(
        part_two,
        vec![
            (TEST_1, 3621), //  1,681,655 ns/iter (+/-  174433)
            (INPUT, 13184), // 97,006,000 ns/iter (+/- 4269830)
        ],
    );
}

type Coord = i16;
type Beacon = [Coord; 3];
type Scanner = Vec<Beacon>;

fn try_align(aligned: &Scanner, candidate: &Scanner) -> Option<(Scanner, Beacon)> {
    let mut result = vec![];
    let mut delta = vec![];
    let mut p_delta = None;
    let mut pp_delta = None;

    'dimension: for dim in 0..3 {
        let dim_values: Vec<_> = aligned.iter().map(|beacon| beacon[dim]).collect();

        for (dim, mirror) in [(0, 1), (1, 1), (2, 1), (0, -1), (1, -1), (2, -1)] {
            if matches!(p_delta, Some(d) if d == dim) || matches!(pp_delta, Some(d) if d == dim) {
                continue;
            }

            let transforms: Vec<_> = candidate
                .iter()
                .map(|beacon| beacon[dim] * mirror)
                .collect();

            let frequencies: HashMap<Coord, u16> = dim_values
                .iter()
                .cartesian_product(transforms.iter())
                .map(|(a, b)| *b - *a)
                .fold(HashMap::new(), |mut acc, item| {
                    *acc.entry(item).or_default() += 1;
                    acc
                });

            let (value, count) = frequencies
                .iter()
                .max_by_key(|(_diff, count)| **count)
                .unwrap();

            if *count >= 12 {
                let value = *value;

                pp_delta = p_delta;
                p_delta = Some(dim);

                let offsets: Vec<_> = transforms.into_iter().map(|v| v - value).collect();
                result.push(offsets);
                delta.push(value);
                continue 'dimension;
            }
        }

        return None;
    }

    let scanner = result[0]
        .iter()
        .zip(result[1].iter())
        .zip(result[2].iter())
        .map(|((x, y), z)| [*x, *y, *z])
        .collect();

    let deltas = [delta[0], delta[1], delta[2]];

    Some((scanner, deltas))
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let (x, line) = line.split_once(",")?;
                    let (y, z) = line.split_once(",")?;
                    Some([x.parse().ok()?, y.parse().ok()?, z.parse().ok()?])
                })
                .collect()
        })
        .collect()
}

fn solve(input: &str) -> (usize, Vec<Beacon>) {
    let mut scanners = parse_input(input);
    let mut scratch_space = vec![];

    let mut next = vec![scanners.remove(0)];

    let mut deltas = vec![[0 as Coord; 3]];

    let mut beacons = vec![];

    while let Some(scanner) = next.pop() {
        for candidate in scanners.drain(..) {
            if let Some((updated, delta)) = try_align(&scanner, &candidate) {
                next.push(updated);
                deltas.push(delta);
            } else {
                scratch_space.push(candidate);
            }
        }

        std::mem::swap(&mut scanners, &mut scratch_space);

        beacons.extend(scanner.into_iter());
    }

    // Apparently it's just quicker to dump the whole thing into a vector, sort, then remove duplicates...
    beacons.sort_unstable();
    beacons.dedup();

    (beacons.len(), deltas)
}

fn part_one(input: &str) -> u64 {
    solve(input).0 as u64
}

fn part_two(input: &str) -> u64 {
    let (_, shifts) = solve(input);
    shifts
        .iter()
        .cartesian_product(shifts.iter())
        .map(|(left, right)| {
            left[0].abs_diff(right[0]) + left[1].abs_diff(right[1]) + left[2].abs_diff(right[2])
        })
        .max()
        .unwrap() as u64
}
