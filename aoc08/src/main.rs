#![feature(test)]
#![feature(str_split_once)]

extern crate test;

use std::collections::{HashMap, VecDeque};

macro_rules! measure {
    ($expr:expr) => {{
    	let stats = test::bench::iter(&mut || $expr);
		let median = stats.median as usize;
		let deviation = (stats.max - stats.min) as usize;
		println!("test {:<36}\tbench:\t{:>11} ns/iter (+/- {})", stringify!($expr), median, deviation);
		$expr
    }};
}

fn main() {
	let mut instructions = include_str!("../input/input.txt")
		.lines()
		.map(|line| {
			let (instruction, value) = line.split_once(' ').unwrap();

			let value = value.parse::<isize>().unwrap();

			match instruction {
				"nop" => Instruction::Nop(value),
				"acc" => Instruction::Acc(value),
				"jmp" => Instruction::Jmp(value),
				_ => panic!("Unsupported instruction: {:?}", instruction),
			}
		})
		.collect::<Box<[Instruction]>>();

	println!("Part One: {}", measure!(part_one(instructions.clone())));
	println!("Part Two: {}", measure!(part_two(instructions.clone())));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

fn part_one(ops: Box<[Instruction]>) -> isize {
	handle_input(ops, |ops, ip, _| {
		let op = *ops.get(ip)?;
		ops[ip] = Instruction::Trap;
		Some(op)
	})
}

fn part_two(ops: Box<[Instruction]>) -> isize {
	let mut branches = VecDeque::new();
	let mut branching = false;

	handle_input(ops, |ops, ip, acc| {
		let op = *ops.get(ip)?;

		match op {
			Instruction::Jmp(_) => if !branching {
				branches.push_back((ip + 1, acc))
			},
			Instruction::Nop(0) => (),
			Instruction::Nop(n) => if !branching {
				branches.push_back(((ip as isize + n) as usize, acc))
			},
			Instruction::Trap => {
				branching = true;
				let (ip, acc) = branches.pop_back()?;
				return Some(Instruction::Reset(ip, acc));
			}
			_ => (),
		}

		ops[ip] = Instruction::Trap;

		Some(op)
	})
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
	Trap,
	Reset(usize, isize),
	Nop(isize),
	Acc(isize),
	Jmp(isize),
}

fn handle_input(mut ops: Box<[Instruction]>, mut decoder: impl FnMut(&mut [Instruction], usize, isize) -> Option<Instruction>) -> isize {
	let mut ip = 0usize;
	let mut acc = 0isize;

	while let Some(op) = decoder(&mut ops, ip, acc) {
		match op {
			Instruction::Nop(_) => ip += 1,
			Instruction::Acc(value) => {
				acc += value;
				ip += 1
			}
			Instruction::Jmp(value) => {
				ip = (ip as isize + value) as usize
			},
			Instruction::Reset(new_ip, new_acc) => {
				ip = new_ip;
				acc = new_acc;
			}
			Instruction::Trap => break,
		}
	}

	acc
}