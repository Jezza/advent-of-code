#![feature(array_windows)]

use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 4512), (INPUT, 46920)]);
    aoc(part_two, vec![(TEST_1, 1924), (INPUT, 12635)]);
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    values: [(u16, bool); 5 * 5],
}

impl Grid {
    fn new(board: &str) -> Self {
        grid::parse_grid(
            board,
            str::lines,
            str::split_ascii_whitespace,
            |width, height| Grid {
                width,
                height,
                values: [(0, false); 5 * 5],
            },
            |grid, x, y, segment| {
                let pos = y * grid.width + x;
                grid.values[pos].0 = segment.parse().unwrap();
            },
        )
    }

    fn sum(&self) -> u32 {
        self.values
            .iter()
            .filter_map(|(v, flag)| if *flag { None } else { Some(*v as u32) })
            .sum()
    }

    fn mark(&mut self, value: u16) -> bool {
        for (i, (v, flag)) in self.values.iter_mut().enumerate() {
            if *v != value {
                continue;
            }

            *flag = true;
            // We can check the row and column immediately, and see if it was a winning "move".

            let y = i / self.width;
            let x = i % self.width;

            let row_win = (0..self.width)
                .map(|new_x| y * self.width + new_x)
                .all(|pos| self.values[pos].1);

            let column_win = (0..self.height)
                .map(|new_y| new_y * self.width + x)
                .all(|pos| self.values[pos].1);

            return row_win || column_win;
        }

        false
    }
}

fn wins(input: &str) -> Vec<(usize, u16, Grid)> {
    let (values, grids) = input.split_once("\n\n").unwrap();

    let values: Vec<_> = values
        .split(",")
        .filter_map(|value| value.parse::<u16>().ok())
        .collect();

    let mut wins: Vec<_> = grids
        .split("\n\n")
        .map(|grid| {
            let mut grid = Grid::new(grid);

            let (first_win, winning_value) = values
                .iter()
                .enumerate()
                .find(|(_, v)| grid.mark(**v))
                .unwrap();

            (first_win, *winning_value, grid)
        })
        .collect();

    wins.sort_unstable_by_key(|(first_win, _, _)| *first_win);

    wins
}

fn part_one(input: &str) -> u32 {
    let wins = wins(input);

    let win = wins.first().unwrap();
    let (_, winning_value, grid) = win;

    grid.sum() * *winning_value as u32
}

fn part_two(input: &str) -> u32 {
    let wins = wins(input);

    let win = wins.last().unwrap();
    let (_, winning_value, grid) = win;

    grid.sum() * *winning_value as u32
}
