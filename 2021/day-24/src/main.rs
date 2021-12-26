use commons::*;

use crate::Op::Input;

fn main() {
	const INPUT: &str = include_str!("../input/input.txt");
	const LE_DIRK: &str = include_str!("../input/le-dirk.txt");

	aoc(part_one,
		vec![
			(INPUT, 69914999975369),
			(LE_DIRK, 99911993949684),
		],
	);
	aoc(part_two,
		vec![
			(INPUT, 14911675311114),
			(LE_DIRK, 62911941716111),
		],
	);
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Reg {
	W,
	X,
	Y,
	Z,
}

impl Reg {
	fn from(input: &str) -> Self {
		match input.as_bytes()[0] {
			b'w' | b'W' => Reg::W,
			b'x' | b'X' => Reg::X,
			b'y' | b'Y' => Reg::Y,
			b'z' | b'Z' => Reg::Z,
			_ => panic!("Unknown register: {}", input),
		}
	}
}

#[derive(Debug, Copy, Clone)]
enum Operand {
	Reg(Reg),
	Immediate(i8),
}

impl Operand {
	fn from(input: &str) -> Self {
		if input.is_empty() {
			return Operand::Immediate(0);
		}
		match input.as_bytes()[0] {
			b'w' | b'W' => Operand::Reg(Reg::W),
			b'x' | b'X' => Operand::Reg(Reg::X),
			b'y' | b'Y' => Operand::Reg(Reg::Y),
			b'z' | b'Z' => Operand::Reg(Reg::Z),
			_ => Operand::Immediate(input.parse().expect(&format!("Unknown number: {}", input))),
		}
	}
}

#[derive(Debug, Copy, Clone)]
enum Op {
	Input(Reg),
	Add(Reg, Operand),
	Mul(Reg, Operand),
	Div(Reg, Operand),
	Mod(Reg, Operand),
	Equal(Reg, Operand),
}

type Word = i64;

fn digits(mut num: Word) -> impl Iterator<Item = Word> {
	let mut div = 1;
	while num >= div * 10 {
		div *= 10;
	}

	std::iter::from_fn(move || {
		if div == 0 {
			None
		} else {
			let value = num / div;
			num %= div;
			div /= 10;
			Some(value)
		}
	})
}

fn run_program(operations: &[Op], input: Word) -> bool {
	let mut input = digits(input);
	let mut state = [0, 0, 0, 0];

	for op in operations {
		let before = state.clone();

		match *op {
			Input(reg) => state[reg as usize] = match input.next() {
				Some(value) => value,
				None => {
					println!("No more input!!!");
					return false;
				}
			},
			Op::Add(a, b) => {
				state[a as usize] = state[a as usize] + match b {
					Operand::Reg(b) => state[b as usize],
					Operand::Immediate(b) => b as Word,
				}
			}
			Op::Mul(a, Operand::Immediate(0)) => state[a as usize] = 0,
			Op::Mul(a, b) => {
				state[a as usize] = state[a as usize] * match b {
					Operand::Reg(b) => state[b as usize],
					Operand::Immediate(b) => b as Word,
				}
			}
			Op::Div(a, Operand::Immediate(1)) => {}
			Op::Div(a, b) => {
				state[a as usize] = state[a as usize] / match b {
					Operand::Reg(b) => state[b as usize],
					Operand::Immediate(b) => b as Word,
				}
			}
			Op::Mod(a, b) => {
				state[a as usize] = state[a as usize] % match b {
					Operand::Reg(b) => state[b as usize],
					Operand::Immediate(b) => b as Word,
				}
			}
			Op::Equal(a, b) => {
				state[a as usize] = (state[a as usize] == match b {
					Operand::Reg(b) => state[b as usize],
					Operand::Immediate(b) => b as Word,
				}) as Word
			}
		}

		println!("{:<25}: {:?} => {:?}", format!("{:?}", *op), before, state);
	}

	state[Reg::Z as usize] == 0
}

fn validate_model(
	numbers: impl IntoIterator<Item = Word>,
	pop: Vec<bool>,
	checks: Vec<i8>,
	offsets: Vec<i8>,
) -> Vec<Word> {
	/*
	The program itself consists of 14 blocks that each perform very similar operations (The differences are highlighted here).

		inp w
		mul x 0
		add x z
		mod x 26
		div z {DIV}
		add x {CHECK}
		eql x w
		eql x 0
		mul y 0
		add y 25
		mul y x
		add y 1
		mul z y
		mul y 0
		add y w
		add y {OFFSET}
		mul y x
		add z y

	Going through, instruction by instruction, and rewrite each one in a higher level language.

	let w = read();
	x *= 0;
	x += z;
	x %= 26;
	z /= {DIV};
	x += {CHECK};
	x = (x == w) as Word;
	x = (x == 0) as Word;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += {OFFSET};
	y *= x;
	z += y;

	It doesn't improve readability much, but there's still more we can do.

	Something multiplied by 0 is 0. (x *= 0) => (x = 0)
	We can merge instructions operating on the same register. {
		x *= 0
		x += 25
	} => {
		x = 25;
	}

	Checking if an equality instruction is checked if it was equal to 0 is the same as not equal. {
		x = (x == w) as Word;
		x = (x == 0) as Word;
	} => {
		x = (x != w) as Word;
	}

	let w = read();
	x = (z % 26) + {CHECK};
	z /= {DIV};
	x = (x != w) as Word;
	y = (25 * x) + 1;
	z *= y;
	y = w + {OFFSET};
	z += (y * x);

	The three placeholders that differ between loops are {CHECK}, {DIV}, and {OFFSET}
	[
		[ {CHECK}, {DIV}, {OFFSET} ],
		..
	]

	{DIV} is always 1 or 26

	Using this knowledge, we can simplify further.

	z /= {DIV};

	This would either `/ 1`, which does nothing, or `/ 26`.

	If we were to do that with a 10 instead of a 26, we can easily see what effect it would have.

	101 / 10 = 10;
	196 / 10 = 19;

	We can see that it "pops" off the last digit, but instead of it being in base 10, like we're used to, it's base 26.

	So this would be more akin to a `pop` operation on a stack.

	Looking at some of the other instructions, we see a similar pattern.

	z % 26

	Trying it out with 10, instead of 26, we can once again, easily see the effect.

	102 % 10 = 2
	121 % 10 = 1

	We "peek" at the last digit.


	Looking at the equality check, we can see something going on with the y register:

	// If it's not equal, x = 1 otherwise 0
	x = (x != w) as Word;

 	// This gives two possible cases y = 26 or 1
	y = (25 * x) + 1;
		`(25 * 1) + 1 = 26`
		`(25 * 0) + 1 = 1`

	// A number times 1, is the same, so in the case of 26 we must once again look at what happens in base 10
	z *= y;

	111 * 10 = 1110
	121 * 10 = 1210

	// We "pushed" a new 0 at the end of the digits.

	This effectively acts as a "push" for a stack.

	But because it's "conditionally" pushed, this acts as if it's behind an if.

	// We have the same kind of thing here:

	y = w + {OFFSET};
	// x is either 1 or 0, depending on the equality check.
	// It'll conditionally do nothing.
	z += (y * x);

	Bringing it all together (pop, push, peek), we have something like this:

	 * w renamed to input
	 * z renamed to stack
	 * x renamed to value
	 * {DIV} renamed to {POP}

	let input = read();
	let value = stack.peek() + {CHECK};
	if {POP} {
		stack.pop()
	}
	if value != input {
		stack.push(w + {OFFSET})
	}

	To "run" the program, all we need to do is extract those parts ({CHECK}, {POP}, {OFFSET}) for each loop,
	and use that to validate an input.
	 */

	let mut input = numbers.into_iter();
	let mut stack: Vec<Word> = vec![];

	for step in 0..14 {
		let x = stack.last()
			.copied()
			.unwrap_or_default()
			+ checks[step] as Word;

		if pop[step] {
			stack.pop();
		}

		let w = match input.next() {
			Some(value) => value,
			None => {
				println!("No more input!");
				break;
			}
		};
		if x != w {
			stack.push(w + offsets[step] as Word)
		}
	}

	stack
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<i8>, Vec<i8>) {
	let validation_program: Vec<Op> = input.lines()
		.map(|line| {
			let (op, a, b) = split!(line, " ", " ");
			match (op, Reg::from(a), Operand::from(b)) {
				("inp", a, _) => Op::Input(a),
				("add", a, b) => Op::Add(a, b),
				("mul", a, b) => Op::Mul(a, b),
				("div", a, b) => Op::Div(a, b),
				("mod", a, b) => Op::Mod(a, b),
				("eql", a, b) => Op::Equal(a, b),
				unknown => panic!("Unknown op: {:?}", unknown),
			}
		})
		.collect();

	/*

	The validation program effectively contains 14 loops, each differing only slightly from loop to loop.
	This collects the differences, as it's relevant for later.

	 */

	let mut pop = vec![];
	let mut checks = vec![];
	let mut offsets = vec![];

	validation_program.split(|op| matches!(op, Op::Input(_)))
		.filter(|ops| !ops.is_empty())
		.for_each(|ops| {
			pop.push(matches!(ops[3], Op::Div(Reg::Z, Operand::Immediate(26))));
			if let Op::Add(Reg::X, Operand::Immediate(value)) = ops[4] {
				checks.push(value);
			} else {
				panic!();
			}
			if let Op::Add(Reg::Y, Operand::Immediate(value)) = ops[14] {
				offsets.push(value);
			} else {
				panic!();
			}
		});

	(pop, checks, offsets)
}

fn part_one(input: &str) -> u64 {
	let (pop, checks, offsets) = parse_input(input);

	/*
	We're trying to end with an empty stack, so we can see that as long as we balance the PUSHES and POPS, we'll be fine.

	let input = read();
	let value = stack.peek() + {CHECK};
	if {POP} {
		stack.pop()
	}
	if value != input {
		stack.push(w + {OFFSET})
	}

	Getting the differences, and organising them will help us identify what we need to do.

	let step = ...;
	if {POP} {
		input[step] == stack.pop() + {CHECK};
	} else {
		stack.push(input[step] + {OFFSET})
	}

	We can build up the answer by altering this structure only slightly.

	input[left_step] + {OFFSET[left_step]} = input[right_step] + {CHECK[right_step]}

	Set the leftmost value to the largest you can manage, and start balancing.

	 */


	let mut stack = vec![];
	let mut answer = vec![];

	for digit in 0..14 {

		// {right} = {left} + left_check - right_offset

		let value = if pop[digit] {
			let (index, value): (usize, Word) = stack.pop().unwrap();

			let value = value + checks[digit] as Word;
			// If we're larger than 9, then the left side _can't_ be a 9, so replace it.
			if value > 9 {
				let value = 9;

				answer[index] -= value - value;
				value
			} else {
				value
			}
		} else {
			// Push the maximum possible value
			let value = 9;

			stack.push((answer.len(), value + offsets[digit] as Word));
			value
		};
		answer.push(value);
	}

	answer.into_iter()
		.fold(0, |acc, item| acc * 10 + item as u64)
}

fn part_two(input: &str) -> u64 {
	let (pop, checks, offsets) = parse_input(input);

	// Part two is more or less the same, but with the smallest possible value we can manage.

	let mut stack = vec![];
	let mut answer = vec![];

	for step in 0..14 {
		let value = if pop[step] {
			let (index, value): (usize, Word) = stack.pop().unwrap();

			let value = value + checks[step] as Word;
			if value <= 1 {
				answer[index] += -value + 1;
				1
			} else {
				value
			}
		} else {
			stack.push((answer.len(), 1 + offsets[step] as Word));
			1
		};
		answer.push(value);
	}

	answer.into_iter()
		.fold(0, |acc, item| acc * 10 + item as u64)
}
