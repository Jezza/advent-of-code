use commons::*;

fn main() {
	const TEST_1: &str = include_str!("../input/test-1.txt");
	const INPUT: &str = include_str!("../input/input.txt");
	const LE_DIRK: &str = include_str!("../input/le-dirk.txt");

	aoc(part_one,
		vec![
			(TEST_1, 739785),
			(INPUT, 913560),
			(LE_DIRK, 504972),
		],
	);
	aoc(part_two,
		vec![
			(TEST_1, 444356092776315),
			(INPUT, 110271560863819),
			(LE_DIRK, 446968027750017),
		],
	);
}

fn parse_input<T>(input: &str) -> (T, T)
	where
		T: std::str::FromStr,
		<T as std::str::FromStr>::Err: std::fmt::Debug,
{
	let (left, right) = input.split_once("\n").unwrap();
	(left[28..].parse::<T>().unwrap(), right[28..].parse::<T>().unwrap())
}

fn part_one(input: &str) -> u64 {
	let (player_one, player_two) = parse_input::<u8>(input);
	let mut player_one = (player_one - 1, 0);
	let mut player_two = (player_two - 1, 0);

	let mut die = 0u64;
	let mut turn = false;

	let loser = loop {
		turn = !turn;
		let ((position, score), other) = if turn {
			(&mut player_one, &mut player_two)
		} else {
			(&mut player_two, &mut player_one)
		};

		let roll = die * 3 + 6;
		die += 3;
		*position = ((*position as u64 + roll) % 10) as u8;
		*score = *score + *position as u64 + 1;
		// println!("Player {} rolled {}+{}+{}={} and moves to space {} for a total score of {}", name, die - 2, die - 1, die, roll, position, score);

		if *score >= 1000 {
			break other;
		}
	};

	(die * loser.1) as u64
}

fn part_two(input: &str) -> u128 {
	let (player_one, player_two) = parse_input::<u8>(input);
	let player_one = (player_one - 1, 0);
	let player_two = (player_two - 1, 0);

	// 3d3 = 27 possible combinations
	const FREQUENCIES: [(u8, u128); 7] = [
		(3, 1), // 1, 1, 1 (1/27)
		(4, 3), // 1, 1, 2 (3/27)
		(5, 6), // 1, 2, 2 (6/27)
		(6, 7), // 1, 2, 3 (7/27)
		(7, 6), // 1, 3, 3 (6/27)
		(8, 3), // 2, 3, 3 (3/27)
		(9, 1), // 3, 3, 3 (1/27)
	];

	#[cached::proc_macro::cached]
	fn split_universe(left: (u8, i128), right: (u8, i128)) -> (u128, u128) {
		FREQUENCIES.into_iter()
			.map(|(position, frequency)| {
				let position = (left.0 + position) % 10;
				let score = left.1 + position as i128 + 1;
				let wins = if score >= 21 {
					(0, 1)
				} else {
					split_universe(right, (position, score))
				};
				(frequency, wins)
			})
			.fold((0, 0), |(left_total_wins, right_total_wins), (frequency, (right_wins, left_wins))| {
				(
					left_total_wins + (left_wins * frequency),
					right_total_wins + (right_wins * frequency),
				)
			})
	}

	{
		use cached::Cached;
		SPLIT_UNIVERSE.lock().unwrap().cache_clear();
	}

	let (player_one_score, player_two_score) = split_universe(player_one, player_two);

	player_one_score.max(player_two_score) as u128
}
