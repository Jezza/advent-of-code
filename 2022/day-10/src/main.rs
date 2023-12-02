use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 13140), (INPUT, 12740)]);
    aoc(part_two, vec![(TEST_1, 124), (INPUT, 107)]);
}

fn handle_input(input: &str, mut func: impl FnMut(i32)) {
    input.lines().for_each(|line| {
        let (_op, value): (&str, Option<i32>) = split_parse!(line, " ");

        func(0);
        if let Some(value) = value {
            func(value);
        }
    });
}

fn part_one(input: &str) -> u64 {
    let mut x = 1;
    let mut cycles = 1;
    let mut signal = 0;

    handle_input(input, |value| {
        if cycles == 20 || (cycles > 20 && ((cycles - 20) % 40) == 0) {
            let strength = cycles * x;
            signal += strength;
        }

        x += value;
        cycles += 1;
    });

    signal as u64
}

type Grid = grid::Grid<bool, 40, 6>;

fn part_two(input: &str) -> u32 {
    let mut x = 1;
    let mut cycles = 1;

    let mut grid = Grid::from_value(40, 6, false);

    handle_input(input, |value| {
        let col = (cycles - 1) % 40;

        if col >= x - 1 && col <= x + 1 {
            *grid.get_raw_mut(cycles as usize) = true;
        }

        x += value;
        cycles += 1;
    });

    // println!("{}", grid.display_with_fmt(|c| if *c { '█' } else { ' ' }));

    grid.values.iter().map(|c| *c as u32).sum()
}
