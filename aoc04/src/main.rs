#![feature(str_split_once)]

use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let count = check_details(|opts| opts.contains_key("byr")
        && opts.contains_key("iyr")
        && opts.contains_key("eyr")
        && opts.contains_key("hgt")
        && opts.contains_key("hcl")
        && opts.contains_key("ecl")
        && opts.contains_key("pid"));

    println!("Count: {}", count);
}

fn check_range(value: &'static str, min: usize, max: usize) -> bool {
    let value: usize = value.parse().unwrap();
    min <= value && value <= max
}

fn check_height(value: &'static str) -> bool {
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.

    if value.ends_with("cm") {
        check_range(&value[..value.len() - 2], 150, 193)
    } else if value.ends_with("in") {
        check_range(&value[..value.len() - 2], 59, 76)
    } else {
        false
    }
}

fn check_hex(input: &'static str) -> bool {
    // a # followed by exactly six characters 0-9 or a-f.
    input.starts_with("#") && input.as_bytes()
        .iter()
        .skip(1)
        .all(|c| b'0' <= *c && *c <= b'9' || b'a' <= *c && *c <= b'z')
}

fn check_eyes(input: &'static str) -> bool {
    match input {
        | "amb"
        | "blu"
        | "brn"
        | "gry"
        | "grn"
        | "hzl"
        | "oth" => true,
        _ => false,
    }
}

fn check_passport(input: &'static str) -> bool {
    // a nine-digit number, including leading zeroes.
    input.len() == 9 && input.as_bytes()
        .iter()
        .all(|c| b'0' <= *c && *c <= b'9')
}

fn part_two() {
    let count = check_details(|opts| {
        opts.get("byr").map_or(false, |value| check_range(value, 1920, 2002))
            && opts.get("iyr").map_or(false, |value| check_range(value, 2010, 2020))
            && opts.get("eyr").map_or(false, |value| check_range(value, 2020, 2030))
            && opts.get("hgt").map_or(false, |value| check_height(value))
            && opts.get("hcl").map_or(false, |value| check_hex(value))
            && opts.get("ecl").map_or(false, |value| check_eyes(value))
            && opts.get("pid").map_or(false, |value| check_passport(value))
    });

    println!("Count: {}", count);
}

fn check_details(handler: impl Fn(&HashMap<&'static str, &'static str>) -> bool) -> usize {
    include_str!("../input/input.txt")
        .split("\n\n")
        .map(|line| line.split('\n')
            .flat_map(|line| line.split(' '))
            .filter_map(|line| line.split_once(':'))
            .collect::<HashMap<&'static str, &'static str>>())
        .filter(|opts| handler(&opts))
        .count()
}

