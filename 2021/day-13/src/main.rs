use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(
        part_one,
        vec![
            (TEST_1, 17), //  1866 ns/iter (+/- 106)
            (INPUT, 695), // 79946 ns/iter (+/- 6951)
        ],
    );
    aoc(
        part_two,
        vec![
            (TEST_1, 30), //  2102 ns/iter (+/- 186)
            (INPUT, 240), // 91708 ns/iter (+/- 3740)
        ],
    );
}

type Points = Vec<Point>;
type Point = (u32, u32);

fn fold(input: &str, n: usize) -> Points {
    let (points, instructions) = input.split_once("\n\n").unwrap();

    let mut points: Points = points
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    for line in instructions.lines().take(n) {
        let (instruction, value) = line.split_once("=").unwrap();
        let fold: u32 = value.parse().unwrap();

        if instruction.ends_with("x") {
            points
                .iter_mut()
                .filter(|p| p.0 > fold)
                .for_each(|p| p.0 = 2 * fold - p.0);
        } else {
            points
                .iter_mut()
                .filter(|p| p.1 > fold)
                .for_each(|p| p.1 = 2 * fold - p.1);
        }
    }

    points.sort_unstable();
    points.dedup();

    points
}

fn part_one(input: &str) -> usize {
    fold(input, 1).len()
}

fn part_two(input: &str) -> usize {
    let points = fold(input, usize::MAX);
    let display = display_points(points);
    // print!("{}", display);
    display.len()
}

fn display_points(points: Points) -> String {
    let (width, height) = points.iter().fold((0, 0), |acc, item| {
        (acc.0.max(item.0 + 2), acc.1.max(item.1 + 1))
    });

    let mut grid = vec![b' '; (width * height) as usize];
    points
        .into_iter()
        .for_each(|(x, y)| grid[(x + y * width) as usize] = b'#');
    (0..height).for_each(|y| grid[((width - 1) + y * width) as usize] = b'\n');

    String::from_utf8(grid).unwrap()
}
