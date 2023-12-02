#![feature(test)]

extern crate test;

fn main() {
    const PART_ONE: &[(usize, usize)] = &[(3, 1)];

    const PART_TWO: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    macro_rules! test_impls {
	    ($($func:ident),*$(,)?) => {{

			$(
				{
					let stats = test::bench::iter(&mut || $func(PART_ONE));
					let median = stats.median as usize;
    				let deviation = (stats.max - stats.min) as usize;
					println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($func), median, deviation);

					let stats = test::bench::iter(&mut || $func(PART_TWO));
					let median = stats.median as usize;
    				let deviation = (stats.max - stats.min) as usize;
					println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($func), median, deviation);

				}
			)*

	    }};
	}

    test_impls! {
        follow_steps_first_functional,
        follow_steps_gen_steps,
        follow_steps_structural,
        follow_steps_dirk,
    }
}

fn follow_steps_dirk(steps: &[(usize, usize)]) -> usize {
    handle_steps(steps, |input, width, &(right, down)| {
        input
            .lines()
            .map(|s| s.as_bytes())
            .enumerate()
            .filter(|&(i, _line)| i % down == 0)
            .enumerate()
            .map(|(i, (_, line))| (i * right, line))
            .filter(|&(i, line)| line[i % width] == b'#')
            .count()
    })
}

fn follow_steps_gen_steps(steps: &[(usize, usize)]) -> usize {
    handle_steps_as_data(steps, |data, width, height, &(right, down)| {
        (0..width)
            .cycle()
            .step_by(right)
            .zip((0..height).step_by(down))
            .skip(1)
            .map(|(x, y)| x + y * width)
            .map(|pos| data[pos])
            .filter(|c| *c == b'#')
            .count()
    })
}

fn follow_steps_first_functional(steps: &[(usize, usize)]) -> usize {
    handle_steps(steps, |input, width, &(right, down)| {
        input
            .lines()
            .step_by(down)
            .zip((0..width).cycle().step_by(right))
            .skip(1)
            .filter(|&(line, x_pos)| line.bytes().nth(x_pos).map(|c| c == b'#').unwrap_or(false))
            .count()
    })
}

fn handle_steps<F>(steps: &[(usize, usize)], handler: F) -> usize
where
    F: Fn(&str, usize, &(usize, usize)) -> usize,
{
    const INPUT: &str = include_str!("../input/input.txt");
    let width = INPUT.find('\n').unwrap();

    steps
        .iter()
        .fold(1, |acc, step| acc * handler(INPUT, width, step))
}

fn handle_steps_as_data<F>(steps: &[(usize, usize)], handler: F) -> usize
where
    F: Fn(&[u8], usize, usize, &(usize, usize)) -> usize,
{
    const INPUT: &str = include_str!("../input/input.txt");
    let width = INPUT.find('\n').unwrap();
    let data = INPUT
        .lines()
        .collect::<String>()
        .into_bytes()
        .into_boxed_slice();

    let height = data.len() / width;

    steps
        .iter()
        .fold(1, |acc, step| acc * handler(&data, width, height, step))
}

fn follow_steps_structural(steps: &[(usize, usize)]) -> usize {
    let map = &Map::new();

    steps.iter().fold(1, |acc, step| {
        let mut pos = (0, 0);
        let mut count = 0;

        loop {
            pos.0 += step.0;
            pos.1 += step.1;

            if let Some(value) = map.at(pos) {
                if value == '#' {
                    count += 1;
                }
            } else {
                break;
            }
        }

        acc * count
    })
}

struct Map {
    width: usize,
    data: Box<[u8]>,
}

impl Map {
    fn new() -> Self {
        let input = include_str!("../input/input.txt");

        let width = input.find('\n').unwrap();
        let data = input
            .lines()
            .collect::<String>()
            .into_bytes()
            .into_boxed_slice();

        Map { width, data }
    }

    fn at(&self, pos: (usize, usize)) -> Option<char> {
        self.data
            .get((pos.0 % self.width) + pos.1 * self.width)
            .cloned()
            .map(|c| c as char)
    }
}
