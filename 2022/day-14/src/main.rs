#![feature(array_windows)]

use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 24), (INPUT, 1001)]);
    aoc(part_two, vec![(TEST_1, 93), (INPUT, 27976)]);
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Tile {
    Air,
    Wall,
    Sand,
    Source,
}

type Grid = grid::Grid<Tile, 1000, 1000>;

type Axis = usize;
type Point = (Axis, Axis);

const OFFSETS: &[(i8, i8); 3] = &[(0, 1), (-1, 1), (1, 1)];

fn handle(input: &str, func: impl Fn(&mut Grid, &mut Axis)) -> u64 {
    let mut grid = Grid::from_value(1000, 1000, Tile::Air);

    let mut max_y = Axis::MIN;

    for line in input.lines() {
        let points = line
            .split(" -> ")
            .map(|point| split_parse!(point, ","))
            .collect::<Vec<Point>>();

        for (_, y) in points.iter() {
            max_y = max_y.max(*y);
        }

        for &[left, right] in points.array_windows() {
            for y in min_max_range(left.1, right.1) {
                for x in min_max_range(left.0, right.0) {
                    *grid.get_mut(x, y) = Tile::Wall;
                }
            }
        }
    }

    func(&mut grid, &mut max_y);

    const SOURCE: Point = (500, 0);
    let mut source = SOURCE;
    let mut path = vec![];
    let mut count = 0;

    *grid.get_mut(source.0, source.1) = Tile::Source;

    'falling: loop {
        for (dx, dy) in *OFFSETS {
            let (x, y) = source;

            let Some(x) = x.checked_add_signed(dx as _) else {
                continue;
            };
            let Some(y) = y.checked_add_signed(dy as _) else {
                continue;
            };

            let tile = *grid.get(x, y);

            if let Tile::Air = tile {
                source = (x, y);
                path.push(source);

                if y > max_y {
                    break 'falling;
                } else {
                    continue 'falling;
                }
            }
        }

        *grid.get_mut(source.0, source.1) = Tile::Sand;
        count += 1;
        if source == SOURCE {
            break;
        }

        let _ = path.pop();
        source = path.pop().unwrap_or(SOURCE);
    }

    count as u64
}

fn part_one(input: &str) -> u64 {
    handle(input, |_, _| ())
}

fn part_two(input: &str) -> u64 {
    handle(input, |grid, max_y| {
        let y = *max_y + 2;
        *max_y += 1;

        for y in y..=y {
            for x in 0..=1000 {
                // println!("\t({}, {})", x, y);
                *grid.get_mut(x, y) = Tile::Wall;
            }
        }
    })
}
