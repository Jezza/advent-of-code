use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 40), (INPUT, 745)]);
    aoc(part_two, vec![(TEST_1, 315), (INPUT, 3002)]);
}

type Grid = grid::Grid<u16, 100, 100>;

fn parse_input(input: &str) -> Grid {
    grid::parse_grid(
        input,
        str::lines,
        |line| line.split(""),
        |width, height| Grid::from_values(width, height, [0; 100 * 100]),
        |grid, x, y, segment| {
            *grid.get_mut(x, y) = segment.parse().unwrap();
        },
    )
}

fn search<C, G>(width: usize, height: usize, cost: C, goal: G) -> i64
where
    C: Fn(usize, usize) -> i64,
    G: Fn(usize, usize) -> bool,
{
    let mut seen = vec![false; width * height];

    let mut queue = std::collections::BinaryHeap::new();
    queue.push((0, 0, 0));

    while let Some((risk, x, y)) = queue.pop() {
        if goal(x, y) {
            return -risk;
        }
        if seen[x + y * width] {
            continue;
        }
        seen[x + y * width] = true;

        macro_rules! queue {
            ($x:expr, $y:expr) => {{
                let x = $x;
                let y = $y;
                if !seen[x + y * width] {
                    queue.push((risk - cost(x, y), x, y));
                }
            }};
        }

        if x > 0 {
            queue!(x - 1, y);
        }
        if x < width - 1 {
            queue!(x + 1, y);
        }
        if y > 0 {
            queue!(x, y - 1);
        }
        if y < height - 1 {
            queue!(x, y + 1);
        }
    }

    panic!("Reached end of possible solutions...")
}

fn part_one(input: &str) -> i64 {
    let grid = parse_input(input);

    search(
        grid.width,
        grid.height,
        |x, y| *grid.get(x, y) as i64,
        |x, y| x == grid.width - 1 && y == grid.height - 1,
    )
}

fn part_two(input: &str) -> i64 {
    let grid = parse_input(input);
    let width = grid.width;
    let height = grid.height;

    let cost = |x, y| {
        let risk = *grid.get(x % width, y % height) as i64;
        let world_x = (x / width) as i64;
        let world_y = (y / height) as i64;
        (risk + world_x + world_y - 1) % 9 + 1
    };

    let width = width * 5;
    let height = height * 5;

    search(width, height, cost, |x, y| {
        x == width - 1 && y == height - 1
    })
}
