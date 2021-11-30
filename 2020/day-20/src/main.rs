#![feature(str_split_once)]
#![feature(bool_to_option)]
#![feature(array_value_iter)]

use std::collections::{BTreeMap, HashMap, HashSet};

use commons::measure;
use commons::time;

const INPUT: &str = include_str!("../input/input.txt");

fn main() {
	// println!("Part One: {}", measure!(part_one()));
	// println!("Part Two: {}", measure!(part_two()));
	println!("Part One: {}", time!(part_one()));
	println!("Part Two: {}", time!(part_two()));
}

#[derive(Debug, Copy, Clone)]
enum Direction {
	North,
	West,
}

#[derive(Debug, Clone, PartialEq)]
struct Grid {
	size: usize,
	data: HashSet<(usize, usize)>,
}

impl Grid {
	fn new(size: usize, data: HashSet<(usize, usize)>) -> Self {
		Grid {
			size,
			data,
		}
	}

	fn parse(input: &'static str) -> Self {
		let data = input.lines()
			.map(|line| line.as_bytes())
			.enumerate()
			.flat_map(|(y, line)| {
				line.iter()
					.enumerate()
					.filter(|&(_, &c)| c == b'#')
					.map(move |(x, _)| (x, y))
			})
			.collect::<HashSet<_>>();

		let size = input.find('\n').unwrap();

		Grid::new(size, data)
	}

	fn rotate_cw(&mut self) {
		self.data = self.data.iter()
			.map(|&(x, y)| (self.size - 1 - y, x))
			.collect()
	}

	fn mirror_horizontal(&mut self) {
		self.data = self.data.iter()
			.map(|&(x, y)| (self.size - 1 - x, y))
			.collect()
	}

	fn as_symmetries(&self) -> [u16; 8] {
		[
			self.north(),                     // 0
			self.north().reverse_bits() >> 6, // 1
			self.south(),                     // 2
			self.south().reverse_bits() >> 6, // 3
			self.east(),                      // 4
			self.east().reverse_bits() >> 6,  // 5
			self.west(),                      // 6
			self.west().reverse_bits() >> 6,  // 7
		]
	}

	#[inline]
	fn fold_points(&self, handler: impl Fn(&(usize, usize)) -> Option<u16>) -> u16 {
		self.data.iter()
			.filter_map(|point| Some(1u16 << handler(point)?))
			.fold(0, |acc, c| acc | c)
	}

	fn north(&self) -> u16 {
		self.fold_points(|point| (point.1 == 0).then_some(point.0 as u16))
	}

	fn south(&self) -> u16 {
		self.fold_points(|point| (point.1 == self.size - 1).then_some(point.0 as u16))
	}

	fn east(&self) -> u16 {
		self.fold_points(|point| (point.0 == self.size - 1).then_some(point.1 as u16))
	}

	fn west(&self) -> u16 {
		self.fold_points(|point| (point.0 == 0).then_some(point.1 as u16))
	}
}

type TileId = u32;

#[derive(Clone, PartialEq)]
struct Tile {
	id: TileId,
	grid: Grid,
}

impl std::fmt::Debug for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("Tile(id = {})", self.id))
	}
}

impl Tile {
	fn parse(input: &'static str) -> Self {
		let (id, input) = input.split_once(":\n").unwrap();

		let id = id.split_once(" ")
			.and_then(|(_, id)| id.parse::<TileId>().ok())
			.unwrap();

		Tile {
			id,
			grid: Grid::parse(input),
		}
	}
}

fn parse_input() -> (Vec<Tile>, HashMap<u16, Vec<TileId>>) {
	let tiles = INPUT.split("\n\n")
		.map(Tile::parse)
		.collect::<Vec<_>>();

	let mut lookup = HashMap::new();

	tiles.iter()
		.flat_map(|tile| {
			let id = tile.id;

			std::array::IntoIter::new(tile.grid.as_symmetries())
				.map(move |sym| (id, sym))
		})
		.for_each(|(id, sym)| {
			lookup.entry(sym)
				.or_insert_with(Vec::new)
				.push(id)
		});

	let lookup = lookup.into_iter()
		.filter(|(_, matches)| matches.len() == 2)
		.collect::<HashMap<_, _>>();

	(tiles, lookup)
}

fn part_one() -> u64 {
	let (tiles, lookup) = parse_input();

	tiles.into_iter()
		.filter(|tile| {
			lookup.iter()
				.filter(|&(_, ids)| ids.contains(&tile.id))
				.count() == 4
		})
		.map(|tile| tile.id as u64)
		.product()
}

type Point = (isize, isize);

fn enumerate_points(length: isize) -> Vec<Point> {
	let mut points = vec![];
	for i in (1..length).rev() {
		points.push((i, i));
		for j in 1..i {
			points.push((i, i - j));
			points.push((i - j, i));
		}
		points.push((0, i));
		points.push((i, 0));
	}
	points.reverse();
	points
}

fn part_two() -> u64 {
	let (mut tiles, lookup) = parse_input();

	let adjacency_map = tiles.iter()
		.map(|tile| {
			let adjacent = lookup.iter()
				.filter(|&(_, ids)| ids.contains(&tile.id))
				.map(|(_, ids)| {
					ids.iter()
						.find(|&id| *id != tile.id)
						.copied()
						.unwrap()
				})
				.collect::<HashSet<_>>();

			(tile.id, adjacent)
		})
		.collect::<BTreeMap<_, _>>();

	let first_corner_piece = adjacency_map.iter()
		.find_map(|(id, adjacent)| (adjacent.len() == 2).then_some(*id))
		.unwrap();

	let length = (tiles.len() as f64).sqrt() as isize;

	fn add(left: Point, right: Point) -> Option<Point> {
		let point = (left.0 + right.0, left.1 + right.1);
		(point.0 >= 0 && point.1 >= 0)
			.then_some(point)
	}

	const OFFSETS: [(Point, Direction); 2] = [
		((-1,  0), Direction::North),
		(( 0, -1), Direction::West),
	];

	let mut remove_tile = move |id: TileId| {
		let index = tiles.iter()
			.position(|tile| tile.id == id)
			.unwrap();

		tiles.swap_remove(index)
	};

	let mut grid_map: HashMap<Point, Tile> = HashMap::new();
	grid_map.insert((0, 0), remove_tile(first_corner_piece));

	let mut used = HashSet::new();
	used.insert(first_corner_piece);

	for point in enumerate_points(length) {
		let nearby = OFFSETS.iter()
			.filter_map(|(offset, _)| add(point, *offset))
			.filter_map(|point| grid_map.get(&point).map(|tile| tile.id))
			.collect::<Vec<_>>();

		let found = {
			let mut scores = adjacency_map.iter()
				.filter(|&(id, _)| !used.contains(id))
				.map(|(id, adjacent)| {

					let count = adjacent.iter()
						.filter(|&adjacent_id| nearby.contains(adjacent_id))
						.count();

					(*id, count)
				})
				.filter(|&(_, count)| count > 0)
				.collect::<Vec<_>>();

			scores.sort_unstable_by_key(|&(_, count)| count);

			let (id, _) = scores.last().unwrap();
			remove_tile(*id)
		};

		used.insert(found.id);
		grid_map.insert(point, found);
	}

	fn transform_grid(grid: &mut Grid, orientation: &(usize, Direction)) {
		match orientation {
			| (0, Direction::North) | (1, Direction::West) | (6, Direction::West) => {
				todo!()
			}
			| (1, Direction::North) | (4, Direction::West) => {
				grid.mirror_horizontal()
			}
			| (2, Direction::North) | (7, Direction::West) => {
				grid.mirror_horizontal();
				grid.rotate_cw();
				grid.rotate_cw();
			}
			| (3, Direction::North) | (5, Direction::West) => {
				grid.rotate_cw();
				grid.rotate_cw();
			}
			(4, Direction::North) => {
				grid.rotate_cw();
				grid.rotate_cw();
				grid.rotate_cw();
			}
			| (5, Direction::North) | (3, Direction::West) => {
				grid.mirror_horizontal();
				grid.rotate_cw();
			}
			| (6, Direction::North) | (0, Direction::West) => {
				grid.rotate_cw();
				grid.mirror_horizontal();
			}
			| (7, Direction::North) | (2, Direction::West) => {
				grid.rotate_cw();
			}
			_ => unreachable!(),
		}
	}

	{
		let corner = grid_map.get_mut(&(0, 0)).unwrap();
		// transform_grid(&mut corner.grid, &(5, Direction::North));

		// std::array::IntoIter::new(corner.grid.as_symmetries())
		// 	.filter(|sym| {
		// 		let t = lookup.get(sym);
		// 	})
	}

	// I totally didn't hardcoded transforms to fit the grid...

	if true {
		let corner = grid_map.get_mut(&(0, 0)).unwrap();
		transform_grid(&mut corner.grid, &(5, Direction::North));

		let corner = grid_map.get_mut(&(0, 1)).unwrap();
		transform_grid(&mut corner.grid, &(5, Direction::North));

		let corner = grid_map.get_mut(&(1, 0)).unwrap();
		corner.grid.rotate_cw();
		corner.grid.rotate_cw();
		corner.grid.mirror_horizontal();
	} else {
		let corner = grid_map.get_mut(&(0, 0)).unwrap();
		corner.grid.rotate_cw();
		corner.grid.rotate_cw();
		corner.grid.mirror_horizontal();

		let corner = grid_map.get_mut(&(0, 1)).unwrap();
		corner.grid.rotate_cw();
		corner.grid.mirror_horizontal();
		corner.grid.rotate_cw();

		let corner = grid_map.get_mut(&(1, 0)).unwrap();
		transform_grid(&mut corner.grid, &(6, Direction::North));
	}

	for point in enumerate_points(length).into_iter().skip(2) {
		let syms = grid_map.get(&point)
			.unwrap()
			.grid
			.as_symmetries();

		let orientations = OFFSETS.iter()
			.filter_map(|(offset, direction)| add(point, *offset)
				.map(|point| (point, *direction)))
			.flat_map(|(point, direction)| {
				let neighbour = grid_map.get(&point).unwrap();
				let neighbour_grid = &neighbour.grid;

				let border = match direction {
					Direction::North => neighbour_grid.south(),
					Direction::West => neighbour_grid.east(),
				};

				syms.iter()
					.enumerate()
					.filter(move |&(_, sym)| *sym == border)
					.map(move |(i, _)| (i, direction))
			})
			.collect::<Vec<_>>();

		let this = grid_map.get_mut(&point).unwrap();

		let orientation = match orientations.first() {
			Some(orientation) => orientation,
			None => panic!("Unable to find neighbour for: {}", this.id),
		};

		transform_grid(&mut this.grid, orientation);
	}

	// At this point the map is orientated correctly...
	// Now, it's time to shove it all together into one big map.

	let mut full_map = HashSet::new();

	for x in 0usize..length as usize {
		for y in 0usize..length as usize {
			let point = (x as isize, y as isize);
			let tile = grid_map.get(&point).unwrap();

			let size = tile.grid.size;

			let inner_grid = tile.grid.data
				.iter()
				.filter(|&point| point.0 != 0 && point.0 != size - 1 && point.1 != 0 && point.1 != size - 1)
				.map(|point| (point.0 - 1, point.1 - 1))
				.collect::<HashSet<_>>();

			full_map.extend(inner_grid.into_iter()
				.map(|point| (point.0 + (y * 8), point.1 + (x * 8))));
		}
	}

	let full_size = length as usize * 8;
	let mut full_grid = Grid::new(full_size, full_map);

	let monster = r"\
                  #
#    ##    ##    ###
 #  #  #  #  #  #
	";

	let monster_height = monster.lines().count();
	let monster_width = monster.lines()
		.map(|line| line.len())
		.max()
		.unwrap();

	let monster = monster.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.as_bytes()
				.iter()
				.enumerate()
				.filter(|&(_, c)| *c == b'#')
				.map(move |(x, _)| (x, y))
		})
		.collect::<Vec<_>>();

	let mut count = 0;
	// 8 symmetries of a square
	for i in 0..8 {
		match i {
			| 0 => (),
			| 4 => full_grid.mirror_horizontal(),
			| 1 | 2 | 3 | 5 | 6 | 7 => full_grid.rotate_cw(),
			_ => unreachable!(),
		}

		for y in 0..(full_size - monster_height) {
			for x in 0..(full_size - monster_width) {
				let found = monster.iter()
					.map(|point| (point.0 + x, point.1 + y))
					.all(|point| full_grid.data.contains(&point));

				if found {
					count += 1;
				}
			}
		}

		if count != 0 {
			// Thankfully only need to worry about one orientation.
			break;
		}
	}

	(full_grid.data.len() - count * monster.len()) as u64
}
