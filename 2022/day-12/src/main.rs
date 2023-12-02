#![feature(array_windows)]
// #![feature(array_zip)]

use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 31), (INPUT, 350)]);
    aoc(part_two, vec![(TEST_1, 29), (INPUT, 349)]);
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Tile {
    Start,
    End,
    Value(u8),
    Tombstone,
}

impl Tile {
    #[track_caller]
    fn value(self) -> u8 {
        match self {
            Tile::Start => b'a',
            Tile::End => b'z',
            Tile::Value(c) => c,
            Tile::Tombstone => panic!(),
        }
    }
}

const OFFSETS: &[(i8, i8); 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

type Grid<T = Tile> = grid::Grid<T, 100, 100>;
type Point = (usize, usize);

fn handle_input(input: &str, goals: impl Fn(&Grid) -> (Point, Tile)) -> u64 {
    let mut grid = grid::parse_grid(
        input,
        str::lines,
        |line| line.split(""),
        |width, height| Grid::from_value(width, height, Tile::Start),
        |grid, x, y, segment| {
            *grid.get_mut(x, y) = match segment.as_bytes()[0] {
                b'S' => Tile::Start,
                b'E' => Tile::End,
                c @ b'a'..=b'z' => Tile::Value(c),
                c => panic!("Unknown character: {}", c as char),
            };
        },
    );

    let (start, end) = goals(&grid);

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
    struct Node {
        point: Point,
        distance: u64,
        value: Tile,
    }

    let start = Node {
        point: start,
        distance: 0,
        value: Tile::End,
    };

    let grid = &mut grid;

    let path = export::pathfinding::directed::dijkstra::dijkstra(
        &start,
        move |&point| {
            let (x, y) = point.point;
            let current = grid.get(x, y).value();

            *grid.get_mut(x, y) = Tile::Tombstone;

            OFFSETS
                .into_iter()
                .filter_map(|&(dx, dy)| {
                    let Some(x) = x.checked_add_signed(dx as _) else {
                        return None;
                    };
                    let Some(y) = y.checked_add_signed(dy as _) else {
                        return None;
                    };

                    let Some(value) = grid.try_get(x, y).copied() else {
                        return None;
                    };

                    if let Tile::Tombstone = value {
                        return None;
                    }

                    if value.value() < current - 1 {
                        return None;
                    }

                    let node = Node {
                        point: (x, y),
                        distance: point.distance + 1,
                        value,
                    };

                    Some((node, node.distance))
                })
                .collect::<Vec<_>>()
        },
        |node| end == node.value,
    );

    let (path, _) = path.unwrap();
    (path.len() - 1) as u64
}

fn part_one(input: &str) -> u64 {
    handle_input(input, |grid| {
        let start = grid
            .values
            .iter()
            .position(|item| matches!(item, Tile::End))
            .map(|index| grid.as_pos(index))
            .unwrap();

        (start, Tile::Start)
    })
}

fn part_two(input: &str) -> u64 {
    handle_input(input, |grid| {
        let start = grid
            .values
            .iter()
            .position(|item| matches!(item, Tile::End))
            .map(|index| grid.as_pos(index))
            .unwrap();

        (start, Tile::Value(b'a'))
    })
}
