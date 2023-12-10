use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    // only 12 red cubes, 13 green cubes, and 14 blue cubes

    aoc(part_one,
        vec![
            (TEST_1, 8),
            (INPUT, 2061),
        ],
    );
    aoc(part_two,
        vec![
            (TEST_1, 2286),
            (INPUT, 72596),
        ],
    );
}

fn handle_input(input: &str) -> (&str, impl Iterator<Item = (u8, u8, u8)>) {



}

fn part_one(input: &str) -> u64 {
    input.lines()
        .filter_map(|line| {
            let (game_no, cubes) = line.split_once(":").expect("No Game _: found");

            let valid_gum = cubes.split(";")
                .all(|line| {
                    let (red, green, blue) = line.split(",")
                        .map(|value| {
                            if let Some(value) = value.strip_suffix(" red") {
                                let value = value.trim().parse::<u8>().unwrap();
                                (value, 0, 0)
                            } else if let Some(value) = value.strip_suffix(" green") {
                                let value = value.trim().parse::<u8>().unwrap();
                                (0, value, 0)
                            } else if let Some(value) = value.strip_suffix(" blue") {
                                let value = value.trim().parse::<u8>().unwrap();
                                (0, 0, value)
                            } else {
                                panic!("at the disco")
                            }
                        })
                        .reduce(|acc, value| {
                            (
                                acc.0 + value.0,
                                acc.1 + value.1,
                                acc.2 + value.2,
                            )
                        })
                        .unwrap_or_default();

                    red <= 12 && green <= 13 && blue <= 14
                });
            let game_no = game_no.rsplit_once(" ").unwrap().1
                .parse::<u8>()
                .unwrap();
            valid_gum.then_some(game_no as u64)
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    input.lines()
        .filter_map(|line| {
            // only 12 red cubes, 13 green cubes, and 14 blue cubes

            let (game_no, cubes) = line.split_once(":").expect("No Game _: found");

            let (red, green, blue) = cubes.split(";")
                .map(|line| {
                    line.split(",")
                        .map(|value| {
                            if let Some(value) = value.strip_suffix(" red") {
                                let value = value.trim().parse::<u8>().unwrap();
                                (value, 0, 0)
                            } else if let Some(value) = value.strip_suffix(" green") {
                                let value = value.trim().parse::<u8>().unwrap();
                                (0, value, 0)
                            } else if let Some(value) = value.strip_suffix(" blue") {
                                let value = value.trim().parse::<u8>().unwrap();
                                (0, 0, value)
                            } else {
                                panic!("at the disco")
                            }
                        })
                        .reduce(|acc, value| {
                            (
                                acc.0 + value.0,
                                acc.1 + value.1,
                                acc.2 + value.2,
                            )
                        })
                        .unwrap_or_default()
                })
                .reduce(|acc, value| {
                    (
                        acc.0.max(value.0),
                        acc.1.max(value.1),
                        acc.2.max(value.2),
                    )
                })
                .unwrap_or_default();

            Some(red as u64 * green as u64 * blue as u64)
        })
        .sum()
}
