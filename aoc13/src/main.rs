#![feature(str_split_once)]

use helper::measure;

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", part_one());
	println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
	let t = include_str!("../input/input.txt");

	let (start, times) = t.split_once('\n').unwrap();
	let start = start.parse::<usize>().unwrap();

	times.split(',')
		.filter_map(|v| v.parse::<usize>().ok())
		.map(|id| (id - (start % id), id))
		.min_by_key(|(val, id)| *val)
		.map(|(val, id)| val * id)
		.unwrap()
}

fn part_two() -> usize {
	let t = include_str!("../input/input.txt");

	let (start, times) = t.split_once('\n').unwrap();
	let start = start.parse::<usize>().unwrap();

	let ids: Vec<_> = times.split(',')
		.enumerate()
		.filter_map(|(i, v)| v.parse::<usize>().ok().map(|v| (i, v)))
		.collect();

	println!("{}", chinese_remainder0(&ids));

	let residue: Vec<i64> = ids.iter()
		.map(|&(index, id)| (id - index) as i64)
		.collect();

	let mods: Vec<i64> = ids.iter()
		.map(|&(index, id)| id as i64)
		.collect();

	chinese_remainder(&residue, &mods).unwrap() as usize
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
	if a == 0 {
		(b, 0, 1)
	} else {
		let (g, x, y) = egcd(b % a, a);
		(g, y - (b / a) * x, x)
	}
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
	let (g, x, _) = egcd(x, n);
	if g == 1 {
		Some((x % n + n) % n)
	} else {
		None
	}
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
	let prod = modulii.iter().product::<i64>();

	let mut sum = 0;

	for (&residue, &modulus) in residues.iter().zip(modulii) {
		let p = prod / modulus;
		// println!("Prod: {}", residue);
		sum += residue * mod_inv(p, modulus)? * p
	}

	Some(sum % prod)
}

// fn egcd0(a: usize, b: usize) -> (usize, usize, usize) {
// 	if a == 0 {
// 		(b, 0, 1)
// 	} else {
// 		let (g, x, y) = egcd0(b % a, a);
// 		(g, y - (b / a) * x, x)
// 	}
// }
//
// fn mod_inv0(x: usize, n: usize) -> usize {
// 	let (g, x, _) = egcd0(x, n);
// 	if g != 1 {
// 		panic!("No known factor.")
// 	}
// 	(x % n + n) % n
// }
//
// fn chinese_remainder0(ids: &[(usize, usize)]) -> usize {
// 	let prod: usize = ids.iter()
// 		.map(|&(index, id)| id)
// 		.product();
//
// 	let sum: usize = ids.iter()
// 		.map(|&(index, id)| {
// 			let p = prod / id;
//
// 			// println!("Prod: {}, {}, {}, {}", id, index, (index as isize - id as isize), );
// 			let residue = id as isize - index as isize;
//
// 			(residue * mod_inv0(p, id) as isize * p as isize) as usize
// 		})
// 		.sum();
//
// 	sum % prod
// }
