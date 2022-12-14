use inputs::get_input;

mod cpu_sim {
    use std::{cmp::Ordering, str::FromStr};

    pub struct CPU {
        accumulator: isize,
        pc: usize,
    }
    pub enum ExecutionErr {
        Ok,
        Finished,
        OutOfBounds,
    }

    impl CPU {
        pub const fn new() -> Self {
            Self {
                accumulator: 0,
                pc: 0,
            }
        }

        pub fn reset(&mut self) {
            self.accumulator = 0;
            self.pc = 0;
        }

        pub fn run_one(&mut self, opcodes: &[Instruction]) -> ExecutionErr {
            let num_ins = opcodes.len();

            match self.pc.cmp(&num_ins) {
                Ordering::Equal => return ExecutionErr::Finished,
                Ordering::Greater => {
                    eprintln!("Invalid address loaded into PC!");
                    return ExecutionErr::OutOfBounds;
                }
                Ordering::Less => (),
            }

            match opcodes[self.pc] {
                Instruction::NOP(_) => self.pc += 1,
                Instruction::ACC(op) => {
                    self.accumulator += op;
                    self.pc += 1;
                }
                Instruction::JMP(op) => {
                    if op < 0 {
                        self.pc -= -op as usize;
                    } else {
                        self.pc += op as usize;
                    }
                }
            }

            ExecutionErr::Ok
        }

        pub const fn accumulator(&self) -> isize {
            self.accumulator
        }

        pub const fn pc(&self) -> usize {
            self.pc
        }
    }

    #[derive(Debug)]
    pub enum Instruction {
        NOP(isize),
        ACC(isize),
        JMP(isize),
    }

    impl FromStr for Instruction {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let x: Vec<&str> = s.split(' ').collect();

            match x.first() {
                Some(&"nop") => Ok(Self::NOP(x[1].parse().unwrap())),
                Some(&"acc") => Ok(Self::ACC(x[1].parse().unwrap())),
                Some(&"jmp") => Ok(Self::JMP(x[1].parse().unwrap())),
                _ => Err(String::from("Invalid opcode passed")),
            }
        }
    }

    impl Clone for Instruction {
        fn clone(&self) -> Self {
            match self {
                Self::ACC(op) => Self::ACC(*op),
                Self::JMP(op) => Self::JMP(*op),
                Self::NOP(op) => Self::NOP(*op),
            }
        }
    }
}

fn part1(opcodes: &[cpu_sim::Instruction]) -> isize {
    let mut cpu = cpu_sim::CPU::new();
    let mut pc_vals = vec![];

    while !pc_vals.contains(&cpu.pc()) {
        pc_vals.push(cpu.pc());
        cpu.run_one(opcodes);
    }

    cpu.accumulator()
}

fn test_sequence(cpu: &mut cpu_sim::CPU, opcodes: &[cpu_sim::Instruction]) -> bool {
    let mut pc_vals = vec![];

    loop {
        if pc_vals.contains(&cpu.pc()) {
            return false;
        }

        pc_vals.push(cpu.pc());

        match cpu.run_one(opcodes) {
            cpu_sim::ExecutionErr::Ok => (),
            cpu_sim::ExecutionErr::Finished => {
                return true;
            }
            cpu_sim::ExecutionErr::OutOfBounds => {
                eprintln!("Out of bounds error occurred!");
                return false;
            }
        }
    }
}

fn part2(opcodes: &[cpu_sim::Instruction]) -> isize {
    let mut cpu = cpu_sim::CPU::new();
    let mut opcodes2 = opcodes.to_vec();

    for i in 0..opcodes2.len() {
        match opcodes2[i] {
            cpu_sim::Instruction::NOP(op) => {
                opcodes2[i] = cpu_sim::Instruction::JMP(op);

                if test_sequence(&mut cpu, &opcodes2) {
                    return cpu.accumulator();
                }

                opcodes2[i] = cpu_sim::Instruction::NOP(op);
            }
            cpu_sim::Instruction::ACC(_) => continue,
            cpu_sim::Instruction::JMP(op) => {
                opcodes2[i] = cpu_sim::Instruction::NOP(op);

                if test_sequence(&mut cpu, &opcodes2) {
                    return cpu.accumulator();
                }

                opcodes2[i] = cpu_sim::Instruction::JMP(op);
            }
        }

        cpu.reset();
    }

    panic!("All possibilities exhausted!");
}

fn main() {
    let inputs =
        get_input::<cpu_sim::Instruction>("inputs/day-8.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs =
        get_input::<cpu_sim::Instruction>("../inputs/day-8.txt").expect("Could not parse path!");

    assert_eq!(part1(&inputs), 1723);
    assert_eq!(part2(&inputs), 846);
}
