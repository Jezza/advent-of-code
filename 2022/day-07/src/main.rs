use commons::*;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one,
        vec![
            (TEST_1, 95437),
            (INPUT, 1325919),
        ],
    );
    aoc(part_two,
        vec![
            (TEST_1, 24933642),
            (INPUT, 2050735),
        ],
    );
}

type Size = u32;

#[derive(Debug, Copy, Clone)]
enum Node {
    File(Size),
    Folder(Size),
}

fn handle_input<'a>(input: &'a str) -> impl Iterator<Item=Node> + 'a {
    let current_depth = input.lines()
        .skip(1)
        .filter_map(|line| line.strip_prefix("$ cd "))
        .fold(2, |acc, item| {
            if item == ".." {
                acc - 1
            } else {
                acc + 1
            }
        });

    let mut stack = vec![0; current_depth];

    input.lines()
        .rev()
        .filter_map(move |line| {
            if let Some(dir) = line.strip_prefix("$ cd ") {
                if dir == ".." {
                    stack.push(0);
                    None
                } else {
                    let size = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += size;
                    Some(Node::Folder(size))
                }
            } else if line.starts_with(char::is_numeric) {
                let (size, _name): (Size, &str) = split_parse!(line, " ");
                *stack.last_mut().unwrap() += size;
                Some(Node::File(size))
            } else {
                None
            }
        })
}

fn part_one(input: &str) -> u64 {
    handle_input(input)
        .filter_map(|node| match node {
            Node::Folder(size) if size <= 100_000 => Some(size),
            _ => None,
        })
        .sum::<Size>() as u64
}

fn part_two(input: &str) -> u64 {
    let mut free_space = 70_000_000;

    let mut folders = vec![];
    for node in handle_input(input) {
        match node {
            Node::File(size) => {
                free_space -= size;
            }
            Node::Folder(size) => {
                folders.push(size)
            }
        }
    }

    folders.sort_unstable();
    let index = folders.binary_search_by(|folder| {
        (free_space + *folder).cmp(&30_000_000)
    }).unwrap_err();
    folders[index] as u64
}
