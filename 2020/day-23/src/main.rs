use commons::measure;
use commons::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
    println!("Part One: {}", measure!(part_one()));
    // println!("Part Two: {}", measure!(part_two()));
    // println!("Part One: {}", time!(part_one()));
    println!("Part Two: {}", time!(part_two()));
}

macro_rules! wrapping_sub {
    ($expr:expr, $max:expr) => {{
        let result = $expr;
        if result == 1 {
            $max
        } else {
            result - 1
        }
    }};
}

fn setup_input(length: usize) -> (usize, Vec<usize>) {
    let input: Box<[usize]> = INPUT.bytes().map(|c| (c - b'0') as usize).collect();

    let mut data = vec![0; length + 1];

    let start = input[0];

    let mut current = start;
    for i in 1..length {
        let next = if i < input.len() { input[i] } else { i + 1 };
        data[current] = next;
        current = next;
    }
    data[current] = start;

    (start, data)
}

fn play_game(length: usize, rounds: usize) -> Vec<usize> {
    let (start, mut list) = setup_input(length);

    let mut current = start;
    for _ in 0..rounds {
        let a = list[current];
        let b = list[a];
        let c = list[b];

        let mut dest = wrapping_sub!(current, length);
        while dest == a || dest == b || dest == c {
            dest = wrapping_sub!(dest, length)
        }

        list[current] = list[c];
        list[c] = list[dest];
        list[dest] = a;
        current = list[current];
    }

    list
}

fn part_one() -> impl std::fmt::Display {
    let list = play_game(9, 100);

    let mut answer = 0;

    let mut current = 1;
    while current < 9 {
        let value = list[current];
        answer = (answer * 10) + value;
        current = value;
    }

    answer
}

fn part_two() -> impl std::fmt::Display {
    let list = play_game(1_000_000, 10_000_000);

    list[1] * list[list[1]]
}
