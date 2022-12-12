use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use commons::*;
use commons::parse::SplitInto;
use parse::Split;

fn main() {
    const TEST_1: &str = include_str!("../input/test-1.txt");
    const INPUT: &str = include_str!("../input/input.txt");

    aoc(part_one,
        vec![
            (TEST_1, 10605),
            (INPUT, 54054),
        ],
    );
    aoc(part_two,
        vec![
            (TEST_1, 2713310158),
            (INPUT, 14314925001),
        ],
    );
}

#[derive(Copy, Clone, Debug)]
enum StressFactor {
    Square,
    Add(Item),
    Multiply(Item),
}

type Item = i64;

#[derive(Clone, Debug)]
struct Monkey {
    inspections: u64,
    items: VecDeque<Item>,
    factor: StressFactor,
    divisor: Item,
    true_branch: i32,
    false_branch: i32,
}

impl<'a> parse::Parse<'a> for Monkey {
    type Error = std::convert::Infallible;

    fn from_str(value: &'a str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        // This is the ordinal.
        let _ = lines.next().unwrap();

        let (_, SplitInto(mut items)): (Ignored, SplitInto<',', VecDeque<Item>>) = split_parse!(lines.next().unwrap(), ": ");

        let factor = {
            let line = lines.next().unwrap();

            let (factor, scale, _): (Option<Item>, char, Ignored) = rsplit_parse!(line, " ", " ");
            match (scale, factor) {
                ('+', Some(factor)) => StressFactor::Add(factor),
                ('*', Some(factor)) => StressFactor::Multiply(factor),
                ('*', None) => StressFactor::Square,
                _ => panic!("Unknown combination of things"),
            }
        };

        let (divisor, _): (Item, Ignored) = rsplit_parse!(lines.next().unwrap(), " ");
        let (true_branch, _): (i32, Ignored) = rsplit_parse!(lines.next().unwrap(), " ");
        let (false_branch, _): (i32, Ignored) = rsplit_parse!(lines.next().unwrap(), " ");

        Ok(Self {
            inspections: 0,
            items,
            factor,
            divisor,
            true_branch,
            false_branch,
        })
    }
}

fn handle_input<const R: usize>(monkeys: &mut [Monkey], chill: impl Fn(Item) -> Item) {
    let mut items = VecDeque::new();

    for _ in 0..R {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            monkey.inspections += monkey.items.len() as u64;

            let divisor = monkey.divisor;
            let factor = monkey.factor;
            let true_branch = monkey.true_branch;
            let false_branch = monkey.false_branch;

            std::mem::swap(&mut items, &mut monkey.items);

            for item in items.drain(..) {

                let item = match factor {
                    StressFactor::Square => item * item,
                    StressFactor::Add(scale) => item + scale,
                    StressFactor::Multiply(scale) => item * scale,
                };

                let item = chill(item);

                let target = if item % divisor == 0 {
                    true_branch
                } else {
                    false_branch
                };

                monkeys[target as usize].items.push_back(item);
            }
        }
    }
}

fn part_one(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n")
        .map(|monkey| parse!(monkey))
        .collect();

    handle_input::<20>(&mut monkeys, |item| item / 3);

    monkeys.sort_unstable_by(|left, right| right.inspections.cmp(&left.inspections));

    (monkeys[0].inspections * monkeys[1].inspections) as _
}

fn part_two(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n")
        .map(|monkey| parse!(monkey))
        .collect();

    let lcd: Item = monkeys.iter().map(|m| m.divisor).product();

    handle_input::<10_000>(&mut monkeys, |item| item % lcd);

    monkeys.sort_unstable_by(|left, right| right.inspections.cmp(&left.inspections));

    (monkeys[0].inspections * monkeys[1].inspections) as _
}
