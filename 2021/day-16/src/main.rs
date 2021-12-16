use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const TEST_2: &str = include_str!("../input/test-2.txt");
	const TEST_3: &str = include_str!("../input/test-3.txt");
	const TEST_4: &str = include_str!("../input/test-4.txt");
	const TEST_5: &str = include_str!("../input/test-5.txt");
	const TEST_6: &str = include_str!("../input/test-6.txt");
	const TEST_7: &str = include_str!("../input/test-7.txt");
	const TEST_8: &str = include_str!("../input/test-8.txt");
	const TEST_9: &str = include_str!("../input/test-9.txt");
	const TEST_10: &str = include_str!("../input/test-10.txt");
	const TEST_11: &str = include_str!("../input/test-11.txt");
	const TEST_12: &str = include_str!("../input/test-12.txt");
	const INPUT: &str = include_str!("../input/input.txt");

	aoc(part_one,
		vec![
			(TEST_1, 16),
			(TEST_2, 12),
			(TEST_3, 23),
			(TEST_4, 31),
			(INPUT, 891),
		],
	);
	aoc(part_two,
		vec![
			(TEST_5, 3),
			(TEST_6, 54),
			(TEST_7, 7),
			(TEST_8, 9),
			(TEST_9, 1),
			(TEST_10, 0),
			(TEST_11, 0),
			(TEST_12, 1),
			(INPUT, 673042777597),
		],
	);
}

#[derive(Debug)]
struct Packet {
	version: u8,
	ty: u8,
	kind: PacketKind,
}

#[derive(Debug)]
enum PacketKind {
	Literal(usize),
	Op(Vec<Packet>),
}

impl Packet {
	fn parse(bits: &mut &[u8]) -> Self {
		let version = parse_binary(take(bits, 3)) as u8;
		let ty = parse_binary(take(bits, 3)) as u8;

		let kind = if ty == 4 {
			let lit = parse_literal(bits);
			PacketKind::Literal(lit)
		} else {
			let length_ty_id = take(bits, 1)[0];
			if length_ty_id == 0 {
				let length = parse_binary(take(bits, 15));
				let mut bits = take(bits, length);
				let mut packets = vec![];
				while bits.len() > 0 {
					packets.push(Self::parse(&mut bits));
				}
				PacketKind::Op(packets)
			} else {
				let count = parse_binary(take(bits, 11));
				let packets = (0..count)
					.map(|_| Self::parse(bits))
					.collect::<Vec<_>>();
				PacketKind::Op(packets)
			}
		};

		Packet {
			ty,
			version,
			kind,
		}
	}

	fn sum_versions(&self) -> usize {
		let mut sum = self.version as usize;
		match &self.kind {
			PacketKind::Op(v) => {
				for p in v {
					sum += p.sum_versions();
				}
			}
			_ => (),
		}
		sum
	}

	fn execute(&self) -> usize {
		match &self.kind {
			PacketKind::Literal(literal) => *literal,
			PacketKind::Op(packets) => match self.ty {
				0 => packets.iter().map(Packet::execute).sum(),
				1 => packets.iter().map(Packet::execute).product(),
				2 => packets.iter().map(Packet::execute).min().unwrap(),
				3 => packets.iter().map(Packet::execute).max().unwrap(),
				5 => {
					let left = packets[0].execute();
					let right = packets[1].execute();
					(left > right) as _
				}
				6 => {
					let left = packets[0].execute();
					let right = packets[1].execute();
					(left < right) as _
				}
				7 => {
					let left = packets[0].execute();
					let right = packets[1].execute();
					(left == right) as _
				}
				_ => unreachable!(),
			}
		}
	}
}

fn take<'a>(bits: &mut &'a [u8], at: usize) -> &'a [u8] {
	let (left, rest) = bits.split_at(at);
	*bits = rest;
	left
}

fn parse_literal(bits: &mut &[u8]) -> usize {
	let mut lit = vec![];
	loop {
		let int = take(bits, 5);
		lit.extend_from_slice(&int[1..]);
		if int[0] == 0 {
			break;
		}
	}
	parse_binary(&lit)
}

fn parse_binary(bits: &[u8]) -> usize {
	bits.iter()
		.fold(0, |acc, item| acc * 2 + *item as usize)
}

fn parse_input(s: &str) -> Vec<u8> {
	s.bytes()
		.flat_map(|b| match b {
			b'0' => [0, 0, 0, 0],
			b'1' => [0, 0, 0, 1],
			b'2' => [0, 0, 1, 0],
			b'3' => [0, 0, 1, 1],
			b'4' => [0, 1, 0, 0],
			b'5' => [0, 1, 0, 1],
			b'6' => [0, 1, 1, 0],
			b'7' => [0, 1, 1, 1],
			b'8' => [1, 0, 0, 0],
			b'9' => [1, 0, 0, 1],
			b'A' => [1, 0, 1, 0],
			b'B' => [1, 0, 1, 1],
			b'C' => [1, 1, 0, 0],
			b'D' => [1, 1, 0, 1],
			b'E' => [1, 1, 1, 0],
			b'F' => [1, 1, 1, 1],
			_ => unreachable!(),
		})
		.collect()
}

fn part_one(input: &str) -> usize {
	let bits = parse_input(input);
	Packet::parse(&mut &bits[..]).sum_versions()
}

fn part_two(input: &str) -> usize {
	let bits = parse_input(input);
	Packet::parse(&mut &bits[..]).execute()
}
