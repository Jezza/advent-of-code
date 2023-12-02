use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one, vec![(TEST_1, 5), (INPUT, 6189)]);
    aoc(part_two, vec![(TEST_1, 12), (INPUT, 19164)]);
}

fn read_input(input: &str) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + '_ {
    input.lines().filter_map(|line| {
        let (origin, target) = line.split_once(" -> ")?;
        let (origin_x, origin_y) = origin.split_once(",")?;
        let (target_x, target_y) = target.split_once(",")?;

        let origin_x = origin_x.parse().ok()?;
        let origin_y = origin_y.parse().ok()?;
        let target_x = target_x.parse().ok()?;
        let target_y = target_y.parse().ok()?;

        let origin = (origin_x, origin_y);
        let target = (target_x, target_y);

        Some((origin, target))
    })
}

fn solve(input: &str, include_diagonals: bool) -> u64 {
    let mut grid = Box::new([0u8; 1000 * 1000]);
    let mut seen = |x: i32, y: i32| {
        let index = y * 1000 + x;
        grid[index as usize] += 1;
    };

    for (origin, target) in read_input(input) {
        let (origin_x, origin_y) = origin;
        let (target_x, target_y) = target;

        if !include_diagonals && origin_x != target_x && origin_y != target_y {
            continue;
        }

        let dx = origin_x - target_x;
        let dy = origin_y - target_y;
        let step_x = dx.signum();
        let step_y = dy.signum();

        let distance = (dx * step_x).max(dy * step_y) + 1;
        for i in 0..distance {
            let x = i * step_x + target_x;
            let y = i * step_y + target_y;

            seen(x, y)
        }
    }

    grid.iter().filter(|value| **value >= 2).count() as u64
}

fn part_one(input: &str) -> u64 {
    solve(input, false)
}

fn part_two(input: &str) -> u64 {
    solve(input, true)
}
