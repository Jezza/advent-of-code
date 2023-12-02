#![feature(str_split_once)]
#![feature(bool_to_option)]
#![feature(array_value_iter)]

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use commons::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
    // println!("Part One: {}", measure!(part_one()));
    // println!("Part Two: {}", measure!(part_two()));
    println!("Part One: {}", time!(part_one()));
    println!("Part Two: {}", time!(part_two()));
}

// test part_one()                          	bench:	       8081 ns/iter (+/- 697)
// Part One: 32162
// test part_two()                          	bench:	    3802625 ns/iter (+/- 364415)
// Part Two: 32534

type Card = u8;
type Deck = Vec<Card>;

fn parse_input() -> (Deck, Deck) {
    fn parse_player(input: &str) -> Deck {
        input
            .lines()
            .skip(1)
            .filter_map(|line| line.parse::<Card>().ok())
            .collect()
    }

    let (player_one, player_two) = INPUT.split_once("\n\n").unwrap();

    (parse_player(player_one), parse_player(player_two))
}

enum RoundWinner {
    PlayerOne(Card, Card),
    PlayerTwo(Card, Card),
    Default,
}

fn play_game<'a>(
    player_one: &'a mut Deck,
    player_two: &'a mut Deck,
    round: impl Fn(&mut Deck, &mut Deck, &mut HashSet<u64>) -> RoundWinner,
) -> &'a mut Deck {
    let mut played = HashSet::new();

    while !player_one.is_empty() && !player_two.is_empty() {
        match round(player_one, player_two, &mut played) {
            RoundWinner::PlayerOne(left, right) => {
                player_one.push(left);
                player_one.push(right);
            }
            RoundWinner::PlayerTwo(left, right) => {
                player_two.push(left);
                player_two.push(right);
            }
            RoundWinner::Default => {
                player_two.clear();
                return player_one;
            }
        }
    }

    if player_two.is_empty() {
        player_one
    } else {
        player_two
    }
}

fn standard_round(left: &mut Deck, right: &mut Deck, _: &mut HashSet<u64>) -> RoundWinner {
    let left = left.remove(0);
    let right = right.remove(0);
    if left > right {
        RoundWinner::PlayerOne(left, right)
    } else {
        RoundWinner::PlayerTwo(right, left)
    }
}

fn part_one() -> impl std::fmt::Display {
    let (mut player_one, mut player_two) = parse_input();

    play_game(&mut player_one, &mut player_two, standard_round)
        .iter()
        .copied()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v as usize)
        .sum::<usize>()
}

fn recursive_round(
    player_one: &mut Deck,
    player_two: &mut Deck,
    played: &mut HashSet<u64>,
) -> RoundWinner {
    let already_played = {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        player_one.hash(&mut hasher);
        player_two.hash(&mut hasher);
        let hash = hasher.finish();

        !played.insert(hash)
    };

    if already_played {
        return RoundWinner::Default;
    }

    let left = player_one.remove(0);
    let right = player_two.remove(0);

    let left_wins = if player_one.len() < left as usize || player_two.len() < right as usize {
        left > right
    } else {
        let mut player_one = player_one
            .iter()
            .take(left as usize)
            .copied()
            .collect::<Deck>();
        let mut player_two = player_two
            .iter()
            .take(right as usize)
            .copied()
            .collect::<Deck>();

        if player_one.iter().max().unwrap() > player_two.iter().max().unwrap() {
            true
        } else {
            play_game(&mut player_one, &mut player_two, recursive_round);
            player_two.is_empty()
        }
    };

    if left_wins {
        RoundWinner::PlayerOne(left, right)
    } else {
        RoundWinner::PlayerTwo(right, left)
    }
}

fn part_two() -> impl std::fmt::Display {
    let (mut player_one, mut player_two) = parse_input();
    let winner = play_game(&mut player_one, &mut player_two, recursive_round);

    winner
        .iter()
        .copied()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v as usize)
        .sum::<usize>()
}
