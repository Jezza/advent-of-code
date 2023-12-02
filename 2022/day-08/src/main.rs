use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 21), (INPUT, 1672)]);
    aoc(part_two, vec![(TEST_1, 8), (INPUT, 327180)]);
}

const MASK: u8 = 0b0000_1111;
const NORTH: u8 = 0b0001_0000;
const SOUTH: u8 = 0b0010_0000;
const EAST: u8 = 0b0100_0000;
const WEST: u8 = 0b1000_0000;

fn name(value: u8) -> &'static str {
    if value & NORTH != 0 {
        "NORTH"
    } else if value & SOUTH != 0 {
        "SOUTH"
    } else if value & EAST != 0 {
        "EAST"
    } else if value & WEST != 0 {
        "WEST"
    } else {
        unreachable!()
    }
}

type Grid = grid::Grid<u8, 100, 100>;

fn part_one(input: &str) -> u64 {
    let mut grid = grid::parse_grid(
        input,
        str::lines,
        |line| line.split(""),
        |width, height| Grid::from_values(width, height, [0u8; 100 * 100]),
        |grid, x, y, segment| {
            *grid.get_mut(x, y) = segment.parse::<u8>().unwrap();
        },
    );

    macro_rules! visible {
        ($x:expr, $y:expr, $bit:expr) => {{
            let x = $x;
            let y = $y;
            let value = *grid.get_mut(x, y);
            // println!("\t\tVISIBLE FROM {} ({},{}) => {}", name($bit), x, y, value & MASK);
            *grid.get_mut(x, y) = value | $bit;
        }};
    }

    macro_rules! check {
        ($bit:expr, |$x:ident = $x_init:expr, $y:ident = $y_init:expr;| $step:expr; $limit:expr) => {{
            {
                #[allow(unused_mut)]
                let mut $y = $y_init;
                #[allow(unused_mut)]
                let mut $x = $x_init;
                // println!("Checking {}... ({}, {})", name($bit), $x, $y);
                let edge = *grid.get($x, $y);
                let mut edge = edge & MASK;

                // println!("\t{} <-> 0", edge);
                visible!($x, $y, $bit);

                loop {
                    if $limit {
                        break;
                    }

                    $step;

                    let tree = *grid.get($x, $y);
                    let tree = tree & MASK;
                    // println!("\t{} <-> {}", tree, edge);

                    if tree > edge {
                        visible!($x, $y, $bit);
                        edge = tree;
                    }
                }
            }
        }};
    }

    for y in 0..grid.height {
        check!(WEST, |x = 0, y = y;| x += 1; x == grid.width - 1);
        check!(EAST, |x = grid.width - 1, y = y;| x -= 1; x == 0);
    }

    for x in 0..grid.width {
        check!(NORTH, |x = x, y = 0;| y += 1; y == grid.height - 1);
        check!(SOUTH, |x = x, y = grid.height - 1;| y -= 1; y == 0);
    }

    grid.values
        .iter()
        .filter(|value| (*value & 0b1111_0000) != 0)
        .count() as u64
}

#[derive(Default, Copy, Clone)]
struct Tile {
    value: u8,
    scenic_score: u32,
}

impl Tile {
    fn new(value: u8) -> Self {
        Self {
            value,
            scenic_score: 1,
        }
    }
}

type Grid2 = grid::Grid<Tile, 100, 100>;

fn part_two(input: &str) -> u64 {
    let mut grid = grid::parse_grid(
        input,
        str::lines,
        |line| line.split(""),
        |width, height| Grid2::from_values(width, height, [Tile::new(0); 100 * 100]),
        |grid, x, y, segment| {
            *grid.get_mut(x, y) = Tile::new(segment.parse().unwrap());
        },
    );

    // const MASK: u16 = 0b0000_1111;
    const NORTH: u16 = 0b0001_0000;
    const SOUTH: u16 = 0b0010_0000;
    const EAST: u16 = 0b0100_0000;
    const WEST: u16 = 0b1000_0000;

    macro_rules! visible {
        ($x:expr, $y:expr, $bit:expr, $count:expr) => {{
            // let x = $x;
            // let y = $y;
            // println!("\t\t{} trees visible FROM {} ({},{}) => {} ({})", $count, name($bit as u8), x, y, tile.value, tile.scenic_score);
            grid.get_mut($x, $y).scenic_score *= $count;

            // *grid.get_mut(x, y) = value | $bit;
        }};
    }

    let mut stack = vec![];

    macro_rules! check {
        ($bit:expr, |$x:ident = $x_init:expr, $y:ident = $y_init:expr;| $step:expr; $limit:expr) => {{
            {
                #[allow(unused_mut)]
                let mut $y = $y_init;
                #[allow(unused_mut)]
                let mut $x = $x_init;
                // println!("Checking {}... ({}, {})", name($bit as u8), $x, $y);
                let edge = grid.get($x, $y).value;

                visible!($x, $y, $bit, 0);

                stack.clear();
                stack.push(edge);

                loop {
                    if $limit {
                        break;
                    }

                    $step;

                    let tree = grid.get($x, $y).value;

                    // println!("\t{}", tree);
                    let index = stack
                        .iter()
                        .rposition(|value| *value >= tree)
                        .unwrap_or_default();
                    let count = stack.len() - index;
                    visible!($x, $y, $bit, count as u32);

                    if tree == 9 {
                        stack.clear();
                    }
                    stack.push(tree);
                }
            }
        }};
    }

    for y in 0..grid.height {
        check!(WEST, |x = 0, y = y;| x += 1; x == grid.width - 1);
        check!(EAST, |x = grid.width - 1, y = y;| x -= 1; x == 0);
    }

    for x in 0..grid.width {
        check!(NORTH, |x = x, y = 0;| y += 1; y == grid.height - 1);
        check!(SOUTH, |x = x, y = grid.height - 1;| y -= 1; y == 0);
    }

    grid.values
        .iter()
        .map(|value| value.scenic_score)
        .max()
        .unwrap() as u64
}
