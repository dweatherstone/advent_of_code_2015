use std::str::FromStr;

pub fn result_day23_stage1(lines: &[String]) -> i64 {
    run_program(lines, 0)
}

pub fn result_day23_stage2(lines: &[String]) -> i64 {
    run_program(lines, 1)
}

fn run_program(lines: &[String], start_a: i64) -> i64 {
    let mut program = Program {
        instructions: lines
            .iter()
            .map(|line| Instruction::from_str(line).expect("instruction not parsed correctly"))
            .collect(),
        ..Default::default()
    };
    program.registers[Register::A as usize] = start_a;
    while let Some(&instruction) = program.instructions.get(program.cp) {
        instruction.apply(&mut program);
    }
    program.registers[Register::B as usize]
}

#[derive(Default)]
struct Program {
    instructions: Vec<Instruction>,
    registers: [i64; 2],
    cp: usize,
}

#[derive(Copy, Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl Instruction {
    fn apply(&self, program: &mut Program) {
        use Instruction::*;
        let mut jump = 1;
        match self {
            Half(reg) => program.registers[*reg as usize] /= 2,
            Triple(reg) => program.registers[*reg as usize] *= 3,
            Increment(reg) => program.registers[*reg as usize] += 1,
            Jump(offset) => jump = *offset,
            JumpIfEven(reg, offset) => {
                if program.registers[*reg as usize] % 2 == 0 {
                    jump = *offset;
                }
            }
            JumpIfOne(reg, offset) => {
                if program.registers[*reg as usize] == 1 {
                    jump = *offset;
                }
            }
        }
        program.cp = program.cp.saturating_add_signed(jump);
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let num_parts = parts.len();
        if !(num_parts == 2 || num_parts == 3) {
            return Err(format!("incorrect instruction: '{s}'"));
        }
        // First part is always the instruction, so we can match on it
        match parts[0] {
            "hlf" => {
                if num_parts != 2 {
                    return Err(format!("incorrect hlf instruction format: '{s}'"));
                }
                match parts[1].replace(',', "").as_str() {
                    "a" => Ok(Instruction::Half(Register::A)),
                    "b" => Ok(Instruction::Half(Register::B)),
                    _ => Err(format!("incorrect register: '{}'", parts[1])),
                }
            }
            "tpl" => {
                if num_parts != 2 {
                    return Err(format!("incorrect tpl instruction format: '{s}'"));
                }
                match parts[1].replace(',', "").as_str() {
                    "a" => Ok(Instruction::Triple(Register::A)),
                    "b" => Ok(Instruction::Triple(Register::B)),
                    _ => Err(format!("incorrect register: '{}'", parts[1])),
                }
            }
            "inc" => {
                if num_parts != 2 {
                    return Err(format!("incorrect inc instruction format: '{s}'"));
                }
                match parts[1].replace(',', "").as_str() {
                    "a" => Ok(Instruction::Increment(Register::A)),
                    "b" => Ok(Instruction::Increment(Register::B)),
                    _ => Err(format!("incorrect register: '{}'", parts[1])),
                }
            }
            "jmp" => {
                if num_parts != 2 {
                    return Err(format!("incorrect jmp instruction format: '{s}'"));
                }
                match parts[1].replace(',', "").parse::<isize>() {
                    Ok(value) => Ok(Instruction::Jump(value)),
                    Err(_) => Err(format!("incorrect offset integer: '{}'", parts[1])),
                }
            }
            "jie" => {
                if num_parts != 3 {
                    return Err(format!("incorrect jie instruction format: '{s}'"));
                }
                let register = match parts[1].replace(',', "").as_str() {
                    "a" => Register::A,
                    "b" => Register::B,
                    _ => return Err(format!("incorrect register: '{}'", parts[1])),
                };
                let offset = match parts[2].parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("incorrect offset integer: '{}'", parts[2])),
                };
                Ok(Instruction::JumpIfEven(register, offset))
            }
            "jio" => {
                if num_parts != 3 {
                    return Err(format!("incorrect jio instruction format: '{s}'"));
                }
                let register = match parts[1].replace(',', "").as_str() {
                    "a" => Register::A,
                    "b" => Register::B,
                    _ => return Err(format!("incorrect register: '{}'", parts[1])),
                };
                let offset = match parts[2].parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("incorrect offset integer: '{}'", parts[2])),
                };
                Ok(Instruction::JumpIfOne(register, offset))
            }
            _ => Err(format!("unknown instruction: '{}'", parts[0])),
        }
    }
}

#[derive(Copy, Clone)]
enum Register {
    A = 0,
    B = 1,
}

#[cfg(test)]
mod day23 {
    use super::*;
    fn get_example() -> Vec<String> {
        vec![
            String::from("inc a"),
            String::from("jio a, +2"),
            String::from("tpl a"),
            String::from("inc a"),
        ]
    }

    #[test]
    fn stage1() {
        let mut program = Program {
            instructions: get_example()
                .iter()
                .map(|line| Instruction::from_str(line).expect("instruction not parsed correctly"))
                .collect(),
            ..Default::default()
        };
        while let Some(&instruction) = program.instructions.get(program.cp) {
            instruction.apply(&mut program);
        }
        assert_eq!(program.registers, [2, 0])
    }

    #[test]
    fn stage2() {
        let mut program = Program {
            instructions: get_example()
                .iter()
                .map(|line| Instruction::from_str(line).expect("instruction not parsed correctly"))
                .collect(),
            ..Default::default()
        };
        program.registers[0] = 1;
        while let Some(&instruction) = program.instructions.get(program.cp) {
            instruction.apply(&mut program);
        }
        assert_eq!(program.registers, [7, 0])
    }
}
