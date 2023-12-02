use commons::*;

const INPUT: &str = { include_str!("../input/input.txt") };

fn main() {
    measure!(part_one());
    measure!(part_one_2());
    println!("Part One: {}", part_one());
    println!("Part One: {}", part_one_2());

    measure!(part_two());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i64 {
    let mut floor = 0;

    for c in INPUT.bytes() {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => (),
        }
    }

    floor
}

fn part_one_2() -> i32 {
    INPUT.bytes().fold(
        0i32,
        |acc, value| {
            if value == b'(' {
                acc + 1
            } else {
                acc - 1
            }
        },
    )
}

fn part_two() -> i64 {
    let mut floor = 0;

    for (i, c) in INPUT.bytes().enumerate() {
        match c {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => (),
        }

        if floor < 0 {
            return i as i64 + 1;
        }
    }

    floor
}
