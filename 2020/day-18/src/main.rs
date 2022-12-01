use logos::Logos;

use commons::*;

fn main() {
	println!("Part One: {}", measure!(part_one()));
	println!("Part Two: {}", measure!(part_two()));
	// println!("Part One: {}", time!(part_one()));
	// println!("Part Two: {}", time!(part_two()));
}

#[derive(Logos, Debug, Copy, Clone, Eq, PartialEq)]
enum Expr {
	#[regex(r"[ \t\n\f\r]+", logos::skip)]
	#[error]
	Error,

	#[regex("[0-9][_0-9]*", |lex| lex.slice().parse())]
	Number(u64),

	#[token("+", |_| 0)]
	Plus(u8),

	#[token("*", |_| 0)]
	Multiply(u8),

	#[token("(")]
	LeftParentheses,

	#[token(")")]
	RightParentheses,
}

fn parse_expr(input: &str, plus_precedence: u8) -> Vec<Expr> {
	let mut output = vec![];
	let mut operators = vec![];

	Expr::lexer(input)
		.into_iter()
		.for_each(|mut token: Expr| {
			if let Expr::Plus(precedence) = &mut token {
				*precedence = plus_precedence;
			}

			match token {
				Expr::Number(_) => output.push(token),
				Expr::Plus(precedence) | Expr::Multiply(precedence) => {
					while let Some(op) = operators.last() {
						if let Expr::Plus(prec) | Expr::Multiply(prec) = *op {
							if precedence <= prec {
								output.push(operators.pop().unwrap());
							} else {
								break;
							}
						} else if *op != Expr::LeftParentheses {
							output.push(operators.pop().unwrap());
						} else {
							break;
						}
					}
					operators.push(token);
				}
				Expr::LeftParentheses => operators.push(token),
				Expr::RightParentheses => {
					loop {
						let last = operators.last().unwrap();
						if *last == Expr::LeftParentheses {
							operators.pop().unwrap();
							break;
						}
						output.push(operators.pop().unwrap());
					}
				}
				_ => panic!("Error thing"),
			}
		});

	while let Some(op) = operators.pop() {
		output.push(op);
	}

	output
}

fn eval(exprs: Vec<Expr>) -> u64 {
	let mut stack = Vec::new();

	for expr in exprs {
		match expr {
			Expr::Number(_) => stack.push(expr),
			Expr::Plus(_) | Expr::Multiply(_) => {
				let right = stack.pop().unwrap();
				let left = stack.pop().unwrap();

				match (left, right) {
					(Expr::Number(left), Expr::Number(right)) => {
						stack.push(Expr::Number(if let Expr::Plus(_) = expr {
							left + right
						} else {
							left * right
						}))
					}
					_ => panic!("Something weird"),
				}
			}
			_ => todo!(),
		}
	}

	if let Some(Expr::Number(value)) = stack.last() {
		*value
	} else {
		panic!("Something went wrong.")
	}
}

fn part_one() -> u64 {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| parse_expr(line, 0))
		.map(eval)
		.sum()
}

fn part_two() -> u64 {
	include_str!("../input/input.txt")
		.lines()
		.map(|line| parse_expr(line, 1))
		.map(eval)
		.sum()
}
