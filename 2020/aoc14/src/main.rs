#![feature(str_split_once)]

use std::collections::HashMap;

use helper::measure;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", part_one());
	// println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	let parser = |mask: &str| {
		mask.as_bytes()
			.iter()
			.rev()
			.enumerate()
			.fold((0u64, 0u64), |(mut mask, mut bits), (i, &v)| {
				match v {
					b'0' => {
						mask |= 1 << i;
					}
					b'1' => {
						mask |= 1 << i;
						bits |= 1 << i;
					}
					_ => (),
				}
				(mask, bits)
			})
	};

	let decoder = |address, value, (mask, bits): (u64, u64)| {
		let value = (value & !mask) | (bits & mask);

		std::iter::once((address, value))
	};

	decode(parser, decoder)
}

fn part_two() -> usize {
	let parser = |mask: &str| {
		let (mut mask0, mut kill_list, mut bits) = (0u64, 0u64, vec![]);

		mask.as_bytes()
			.iter()
			.rev()
			.enumerate()
			.for_each(|(i, &v)| match v {
				b'X' => {
					kill_list |= (1 << i);
					bits.push(i)
				},
				b'1' => mask0 |= (1 << i),
				_ => (),
			});

		let mask = mask0;

		let result = (0..2u64.pow(bits.len() as u32))
			.map(|iter| bits.iter()
				.enumerate()
				.fold(mask, |acc, (i, &bit)| acc | (iter & 1 << i) << bit - i))
			.collect::<Vec<u64>>();

		(kill_list, result)
	};

	let decoder = |address: u64, value, (kill_list, addresses): (u64, Vec<u64>)| {
		addresses.into_iter()
			.map(move |addr| ((address & !kill_list) | addr, value))
	};

	decode(parser, decoder)
}

fn decode<Mask, Parser, Decoder, Addresses>(parser: Parser, decoder: Decoder) -> usize
	where
		Mask: Clone,
		Parser: Fn(&str) -> Mask,
		Decoder: Fn(u64, u64, Mask) -> Addresses,
		Addresses: Iterator<Item = (u64, u64)>,
{
	include_str!("../input/input.txt")
		.split("mask = ")
		.filter(|section| !section.is_empty())
		.flat_map(|section| {
			let mut lines = section.lines();

			let mask = lines.next().unwrap();
			let mask = parser(mask);

			let decoder = &decoder;

			lines.flat_map(move |part| {
				let (address, value) = part.split_once("] = ").unwrap();
				let address = address[4..].parse::<u64>().unwrap();
				let value = value.parse::<u64>().unwrap();

				decoder(address, value, mask.clone())
			})
		})
		.collect::<HashMap<u64, u64>>()
		.values()
		.copied()
		.sum::<u64>() as usize
}

trait Decoder {
	type Mask;


}

fn decode0<Mask, Parser, Decoder, Addresses>(parser: Parser, decoder: Decoder) -> usize
	where
		Mask: Clone,
		Parser: Fn(&str) -> Mask,
		Decoder: Fn(u64, u64, Mask) -> Addresses,
		Addresses: Iterator<Item = (u64, u64)>,
{
	include_str!("../input/input.txt")
		.split("mask = ")
		.filter(|section| !section.is_empty())
		.flat_map(|section| {
			let mut lines = section.lines();

			let mask = lines.next().unwrap();
			let mask = parser(mask);

			let decoder = &decoder;

			lines.flat_map(move |part| {
				let (address, value) = part.split_once("] = ").unwrap();
				let address = address[4..].parse::<u64>().unwrap();
				let value = value.parse::<u64>().unwrap();

				decoder(address, value, mask.clone())
			})
		})
		.collect::<HashMap<u64, u64>>()
		.values()
		.copied()
		.sum::<u64>() as usize
}
