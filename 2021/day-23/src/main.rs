#![feature(int_abs_diff)]

use commons::export::pathfinding::directed::dijkstra::dijkstra;
use commons::grid::parse_grid;
use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const TEST_2: &str = include_str!("../input/test-2.txt");
    const INPUT_1: &str = include_str!("../input/input-1.txt");
    const INPUT_2: &str = include_str!("../input/input-2.txt");

    aoc(part_one, vec![(TEST_1, 12521), (INPUT_1, 15338)]);
    aoc(part_two, vec![(TEST_2, 44169), (INPUT_2, 47064)]);
}

type Position = (usize, usize);
type Positions = grid::Grid<u8, 13, 5>;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Board {
    positions: Positions,
    remaining: [u8; 4],
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            positions: Positions::from_values(width, height, [b' '; 13 * 5]),
            remaining: [0; 4],
        }
    }

    fn print(&self) {
        self.positions.print_with_fmt(|c| *c as char);
    }

    fn cleanup(&mut self) {
        self.positions.iter_pos_tuples().for_each(|pos @ (x, y)| {
            let ty = *self.positions.get(x, y);
            if !matches!(ty, b'A' | b'B' | b'C' | b'D') {
                return;
            }
            let ty_offset = (ty - b'A') as usize;

            let target = (ty_offset * 2 + 3, self.remaining[ty_offset] as usize);

            if pos == target {
                *self.positions.get_mut(x, y) = b'@';
                self.remaining[ty_offset] -= 1;
            }
        });
    }

    fn possible_moves(&self) -> Vec<(Board, u64)> {
        let Board {
            positions,
            remaining,
        } = self;

        positions
            .iter_pos_tuples()
            .flat_map(|pos @ (x, y)| {
                let ty = *positions.get(x, y);
                if !matches!(ty, b'A' | b'B' | b'C' | b'D') {
                    return vec![];
                }
                let ty_offset = (ty - b'A') as usize;

                let mut boards = vec![];

                if y == 0 {
                    let target @ (target_x, target_y) =
                        (ty_offset * 2 + 3, remaining[ty_offset] as usize);

                    if self.has_path(pos, target) {
                        let cost = x.abs_diff(target_x) + y.abs_diff(target_y);

                        let mut board = self.clone();
                        *board.positions.get_mut(x, y) = b'.';
                        *board.positions.get_mut(target_x, target_y) = b'@';
                        board.remaining[ty_offset] -= 1;

                        let cost = COST[ty_offset] * (cost as u64);

                        boards.push((board, cost));
                    }
                } else {
                    for target @ (target_x, target_y) in HALLWAYS {
                        if self.has_path(pos, target) {
                            let cost = x.abs_diff(target_x) + y.abs_diff(target_y);

                            let mut board = self.clone();
                            *board.positions.get_mut(x, y) = b'.';
                            *board.positions.get_mut(target_x, target_y) = ty;

                            let cost = COST[ty_offset] * (cost as u64);

                            boards.push((board, cost));
                        }
                    }
                }

                boards
            })
            .collect()
    }

    fn has_path(
        &self,
        start @ (start_x, start_y): Position,
        target @ (target_x, target_y): Position,
    ) -> bool {
        if start == target {
            true
        } else if start_y == 0 {
            // Starts in a hallway.

            // Check if only self is in the given range.
            let only_self = (start_x.min(target_x)..=start_x.max(target_x))
                .filter(|x| *self.positions.get(*x as usize, 0) != b'.')
                .count()
                == 1;

            // Check if vertical path is clear.
            only_self
                && (0..=target_y)
                    .filter(|y| *self.positions.get(target_x as usize, *y as usize) != b'.')
                    .count()
                    == 0
        } else if target_y == 0 {
            // Ends in a hallway

            // Check if vertical path is clear.
            let only_self = (0..=start_y)
                .filter(|y| *self.positions.get(start_x as usize, *y as usize) != b'.')
                .count()
                == 1;

            // Check if only self is in the given range.
            only_self
                && (start_x.min(target_x)..=start_x.max(target_x))
                    .filter(|x| *self.positions.get(*x as usize, 0) != b'.')
                    .count()
                    == 0
        } else {
            panic!("Invalid positions: {:?} and {:?}", start, target);
        }
    }
}

static HALLWAYS: [Position; 7] = [(1, 0), (2, 0), (4, 0), (6, 0), (8, 0), (10, 0), (11, 0)];
static COST: [u64; 4] = [1, 10, 100, 1000];

fn search(board: Board) -> u64 {
    dijkstra(&board, Board::possible_moves, |board| {
        board.remaining == [0; 4]
    })
    .expect("Ya dun goof'd")
    .1
}

fn parse_input(input: &str) -> Board {
    let mut board = parse_grid(
        input,
        |input| {
            input
                .lines()
                .filter(|line| line.as_bytes().iter().any(|c| *c != b'#' && *c != b' '))
        },
        |line| line.split(""),
        |width, height| Board::new(width, height),
        |board, x, y, segment| {
            if matches!(segment, "A" | "B" | "C" | "D" | "#" | ".") {
                *board.positions.get_mut(x, y) = segment.as_bytes()[0];
            }
        },
    );
    board.remaining = [board.positions.height as u8 - 1; 4];
    board.cleanup();
    board
}

fn part_one(input: &str) -> u64 {
    let board = parse_input(input);
    search(board)
}

fn part_two(input: &str) -> u64 {
    let board = parse_input(input);
    search(board)
}
