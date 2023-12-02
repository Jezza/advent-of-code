use commons::*;

const INPUT: &str = { include_str!("../input/input.txt") };

fn main() {
    measure!(part_one());
    println!("Part One: {}", part_one());

    measure!(part_two());
    println!("Part Two: {}", part_two());
}

fn run_program(memory: &mut [usize]) {
    let mut ip = 0;

    loop {
        let op = memory[ip];
        // println!("OP: {}", op);

        match op {
            1 => {
                let left_addr = memory[ip + 1];
                let right_addr = memory[ip + 2];
                let out_addr = memory[ip + 3];

                let left = memory[left_addr];
                let right = memory[right_addr];

                // println!("{}({}) + {}({}) (={}) => [{}]", left_addr, program[left_addr], right_addr, program[right_addr], left + right, out_addr);
                memory[out_addr] = left + right;

                ip += 1 + 3;
            }
            2 => {
                let left_addr = memory[ip + 1];
                let right_addr = memory[ip + 2];
                let out_addr = memory[ip + 3];

                let left = memory[left_addr];
                let right = memory[right_addr];

                // println!("{}({}) * {}({}) (={}) => [{}]", left_addr, program[left_addr], right_addr, program[right_addr], left * right, out_addr);
                memory[out_addr] = left * right;

                ip += 1 + 3;
            }
            99 => break,
            _ => panic!("Unknown OP code: {}", op),
        }
    }
}

fn part_one() -> u64 {
    let mut memory = INPUT
        .split(',')
        .filter_map(|op| op.parse::<usize>().ok())
        .collect::<Vec<_>>();

    memory[1] = 12;
    memory[2] = 2;

    run_program(&mut memory);

    memory[0] as u64
}

fn part_two() -> u64 {
    let memory = INPUT
        .split(',')
        .filter_map(|op| op.parse::<usize>().ok())
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;

            run_program(&mut memory);

            if memory[0] == 19690720 {
                return (100 * noun + verb) as u64;
            }
        }
    }

    panic!("Unable to find pair")
}
