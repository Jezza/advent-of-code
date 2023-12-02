#![feature(array_windows)]
#![feature(box_into_boxed_slice)]

fn main() {
    // println!("Part One: {}", measure!(part_one()));
    // println!("Part Two: {}", measure!(part_two()));
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

macro_rules! point {
    ($pos:expr, $width:expr) => {{
        let x = $pos % $width;
        let y = $pos / $width;
        (x, y)
    }};
}

macro_rules! diff {
    ($left:expr, $right:expr) => {{
        let left = $left as isize;
        let right = $right as isize;
        if left > right {
            left - right
        } else {
            right - left
        }
    }};
}

fn part_one() -> usize {
    fn tick(offsets: &[isize], current: &Grid, next: &mut Grid) {
        for (i, &v) in current.data.iter().enumerate() {
            if v == b'.' {
                continue;
            }

            let (x, y) = point!(i, current.width);

            let mut count = 0;
            for offset in offsets {
                let pos = i as isize + offset;
                if pos < 0 {
                    continue;
                }
                let pos = pos as usize;
                if pos >= current.len() {
                    continue;
                }

                let (their_x, their_y) = point!(pos, current.width);

                if diff!(their_x, x) > 1 || diff!(their_y, y) > 1 {
                    continue;
                }

                if current.data[pos] == b'#' {
                    count += 1;
                }
            }

            if v == b'L' {
                if count == 0 {
                    next.data[i] = b'#';
                }
            } else if v == b'#' {
                if count >= 4 {
                    next.data[i] = b'L';
                }
            }
        }
    }

    stabilise_table(tick)
}

fn part_two() -> usize {
    fn find_neighbour(current: &Grid, current_pos: (usize, usize), offset: isize) -> bool {
        // let (mut prev_x, mut prev_y) = (x, y);
        let (mut prev_x, mut prev_y) = current_pos;
        let mut pos = offset as isize;
        loop {
            pos += offset;
            if pos < 0 {
                break false;
            }
            let pos = pos as usize;
            if pos >= current.len() {
                break false;
            }

            let (their_x, their_y) = point!(pos, current.width);

            if diff!(their_x, prev_x) > 1 || diff!(their_y, prev_y) > 1 {
                break false;
            }

            prev_x = their_x;
            prev_y = their_y;

            let thing = current.data[pos];
            if thing != b'.' {
                if thing == b'#' {
                    // count += 1;
                    break true;
                }
                break false;
            }
        }
    }

    fn tick(offsets: &[isize], current: &Grid, next: &mut Grid) {
        for (i, &v) in current.data.iter().enumerate() {
            if v == b'.' {
                continue;
            }

            let (x, y) = point!(i, current.width);

            let mut count = 0;
            for offset in offsets {
                let (mut prev_x, mut prev_y) = (x, y);
                let mut pos = i as isize;
                loop {
                    pos += offset;
                    if pos < 0 {
                        break;
                    }
                    let pos = pos as usize;
                    if pos >= current.len() {
                        break;
                    }

                    let (their_x, their_y) = point!(pos, current.width);

                    if diff!(their_x, prev_x) > 1 || diff!(their_y, prev_y) > 1 {
                        break;
                    }

                    prev_x = their_x;
                    prev_y = their_y;

                    let thing = current.data[pos];
                    if thing != b'.' {
                        if thing == b'#' {
                            count += 1;
                        }
                        break;
                    }
                }
            }

            if v == b'L' {
                if count == 0 {
                    next.data[i] = b'#';
                }
            } else if v == b'#' {
                if count >= 5 {
                    next.data[i] = b'L';
                }
            }
        }
    }

    stabilise_table(tick)
}

fn standard_tick(
    find_neighbour: impl Fn(&Grid, (usize, usize), isize) -> bool,
    threshold: usize,
    offsets: &[isize],
    current: &Grid,
    next: &mut Grid,
) {
    // let threshold = 4;

    for (i, &v) in current.data.iter().enumerate() {
        if v == b'.' {
            continue;
        }

        let pos = point!(i, current.width);

        let mut count = 0;
        for &offset in offsets {
            if find_neighbour(current, pos, offset) {
                count += 1;
            }
        }

        if v == b'L' {
            if count == 0 {
                next.data[i] = b'#';
            }
        } else if v == b'#' {
            if count >= threshold {
                next.data[i] = b'L';
            }
        }
    }
}

fn stabilise_table0(
    find_neighbour: impl Fn(&Grid, (usize, usize), isize) -> bool,
    threshold: usize,
) -> usize {
    //  find_neighbour(current: &Grid, current_pos: (usize, usize), offset: isize) -> bool
    let mut last = Grid::new();
    let mut current = last.clone();

    // let tick = || {
    // 	for (i, &v) in current.data.iter().enumerate() {
    // 		if v == b'.' {
    // 			continue;
    // 		}
    //
    // 		let (x, y) = point!(i, current.width);
    //
    // 		let mut count = 0;
    // 		for offset in offsets {
    // 			let pos = i as isize + offset;
    // 			if pos < 0 {
    // 				continue
    // 			}
    // 			let pos = pos as usize;
    // 			if pos >= current.len() {
    // 				continue;
    // 			}
    //
    // 			let (their_x, their_y) = point!(pos, current.width);
    //
    // 			if diff!(their_x, x) > 1 || diff!(their_y, y) > 1 {
    // 				continue;
    // 			}
    //
    // 			if current.data[pos] == b'#' {
    // 				count += 1;
    // 			}
    // 		}
    //
    // 		if v == b'L' {
    // 			if count == 0 {
    // 				next.data[i] = b'#';
    // 			}
    // 		} else if v == b'#' {
    // 			if count >= threshold {
    // 				next.data[i] = b'L';
    // 			}
    // 		}
    // 	}
    // }

    let offsets: [isize; 8] = {
        let w = current.width as isize;

        [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1]
    };

    standard_tick(&find_neighbour, threshold, &offsets, &last, &mut current);

    while current != last {
        last.data.as_mut().copy_from_slice(&current.data);
        standard_tick(&find_neighbour, threshold, &offsets, &last, &mut current)
    }

    let Grid { data, .. } = current;

    data.iter().filter(|&&c| c == b'#').count()
}

fn stabilise_table(tick: impl Fn(&[isize], &Grid, &mut Grid)) -> usize {
    let mut last = Grid::new();
    let mut current = last.clone();

    let offsets: [isize; 8] = {
        let w = current.width as isize;

        [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1]
    };

    tick(&offsets, &last, &mut current);

    while current != last {
        last.data.as_mut().copy_from_slice(&current.data);
        tick(&offsets, &last, &mut current)
    }

    let Grid { data, .. } = current;

    data.iter().filter(|&&c| c == b'#').count()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
    data: Box<[u8]>,
    width: usize,
}

impl Grid {
    fn new() -> Self {
        let input = include_str!("../input/input.txt");

        let width = input.find('\n').unwrap();
        let data = input
            .lines()
            .collect::<String>()
            .into_bytes()
            .into_boxed_slice();

        Grid { data, width }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
