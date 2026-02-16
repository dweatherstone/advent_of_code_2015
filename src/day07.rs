use std::collections::{HashMap, VecDeque};

pub fn parse_day07(lines: &[String]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        let (lhs, output) = line.split_once(" -> ").unwrap();
        let tokens: Vec<&str> = lhs.split_whitespace().collect();
        match tokens.len() {
            1 => instructions.push(Instruction::Assign {
                input: parse_operand(tokens[0]),
                output: output.to_string(),
            }),
            2 => instructions.push(Instruction::Not {
                input: parse_operand(tokens[1]),
                output: output.to_string(),
            }),
            3 => match tokens[1] {
                "AND" => instructions.push(Instruction::And {
                    left: parse_operand(tokens[0]),
                    right: parse_operand(tokens[2]),
                    output: output.to_string(),
                }),
                "OR" => instructions.push(Instruction::Or {
                    left: parse_operand(tokens[0]),
                    right: parse_operand(tokens[2]),
                    output: output.to_string(),
                }),
                "LSHIFT" => instructions.push(Instruction::Lshift {
                    left: parse_operand(tokens[0]),
                    amount: parse_operand(tokens[2]),
                    output: output.to_string(),
                }),
                "RSHIFT" => instructions.push(Instruction::Rshift {
                    left: parse_operand(tokens[0]),
                    amount: parse_operand(tokens[2]),
                    output: output.to_string(),
                }),
                _ => panic!("unknown operator"),
            },
            _ => panic!("malformed instruction"),
        }
    }
    instructions
}

pub fn result_day07_stage1(instructions: &[Instruction], output_wire: &str) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut queue: VecDeque<&Instruction> = VecDeque::from_iter(instructions);
    while let Some(instruction) = queue.pop_front() {
        if let Some(value) = instruction.try_process(&wires) {
            wires.insert(instruction.get_output().to_string(), value);
        } else {
            queue.push_back(instruction);
        }
    }

    *wires.get(output_wire).unwrap()
}

pub fn result_day07_stage2(instructions: &[Instruction], output_wire: &str, initial_b: u16) -> u16 {
    let mut instructions2 = instructions.to_vec();
    instructions2.retain(|instr| instr.get_output() != "b");
    instructions2.push(Instruction::Assign {
        input: Operand::Value(initial_b),
        output: "b".to_string(),
    });
    result_day07_stage1(&instructions2, output_wire)
}

fn parse_operand(operand: &str) -> Operand {
    operand
        .parse()
        .map(Operand::Value)
        .unwrap_or(Operand::Wire(operand.to_string()))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Wire(String),
    Value(u16),
}

impl Operand {
    fn can_process(&self, wires: &HashMap<String, u16>) -> bool {
        match self {
            Operand::Value(_) => true,
            Operand::Wire(wire) => wires.contains_key(wire.as_str()),
        }
    }

    fn value(&self, wires: &HashMap<String, u16>) -> u16 {
        match self {
            Operand::Value(v) => *v,
            Operand::Wire(wire) => *wires.get(wire.as_str()).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Assign {
        input: Operand,
        output: String,
    },
    Not {
        input: Operand,
        output: String,
    },
    And {
        left: Operand,
        right: Operand,
        output: String,
    },
    Or {
        left: Operand,
        right: Operand,
        output: String,
    },
    Lshift {
        left: Operand,
        amount: Operand,
        output: String,
    },
    Rshift {
        left: Operand,
        amount: Operand,
        output: String,
    },
}

impl Instruction {
    fn try_process(&self, wires: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Instruction::Assign { input, .. } => {
                if input.can_process(wires) {
                    Some(input.value(wires))
                } else {
                    None
                }
            }
            Instruction::Not { input, .. } => {
                if input.can_process(wires) {
                    Some(!input.value(wires))
                } else {
                    None
                }
            }
            Instruction::And { left, right, .. } => {
                if left.can_process(wires) && right.can_process(wires) {
                    Some(left.value(wires) & right.value(wires))
                } else {
                    None
                }
            }
            Instruction::Or { left, right, .. } => {
                if left.can_process(wires) && right.can_process(wires) {
                    Some(left.value(wires) | right.value(wires))
                } else {
                    None
                }
            }
            Instruction::Lshift { left, amount, .. } => {
                if left.can_process(wires) && amount.can_process(wires) {
                    Some(left.value(wires) << amount.value(wires))
                } else {
                    None
                }
            }
            Instruction::Rshift { left, amount, .. } => {
                if left.can_process(wires) && amount.can_process(wires) {
                    Some(left.value(wires) >> amount.value(wires))
                } else {
                    None
                }
            }
        }
    }

    fn get_output(&self) -> &str {
        match self {
            Instruction::Assign { output, .. } => output,
            Instruction::Not { output, .. } => output,
            Instruction::And { output, .. } => output,
            Instruction::Or { output, .. } => output,
            Instruction::Lshift { output, .. } => output,
            Instruction::Rshift { output, .. } => output,
        }
    }
}

#[cfg(test)]
mod day07 {
    use super::*;

    fn get_example1() -> Vec<String> {
        vec![
            String::from("123 -> x"),
            String::from("456 -> y"),
            String::from("x AND y -> d"),
            String::from("x OR y -> e"),
            String::from("x LSHIFT 2 -> f"),
            String::from("y RSHIFT 2 -> g"),
            String::from("NOT x -> h"),
            String::from("NOT y -> i"),
        ]
    }

    #[test]
    fn parse_example() {
        use Instruction::*;
        use Operand::*;
        let expected = vec![
            Assign {
                input: Value(123),
                output: "x".to_string(),
            },
            Assign {
                input: Value(456),
                output: "y".to_string(),
            },
            And {
                left: Wire("x".to_string()),
                right: Wire("y".to_string()),
                output: "d".to_string(),
            },
            Or {
                left: Wire("x".to_string()),
                right: Wire("y".to_string()),
                output: "e".to_string(),
            },
            Lshift {
                left: Wire("x".to_string()),
                amount: Value(2),
                output: "f".to_string(),
            },
            Rshift {
                left: Wire("y".to_string()),
                amount: Value(2),
                output: "g".to_string(),
            },
            Not {
                input: Wire("x".to_string()),
                output: "h".to_string(),
            },
            Not {
                input: Wire("y".to_string()),
                output: "i".to_string(),
            },
        ];
        let instructions = parse_day07(&get_example1());
        assert_eq!(instructions, expected);
    }

    #[test]
    fn stage1() {
        let instructions = parse_day07(&get_example1());
        let expected: HashMap<&str, u16> = HashMap::from_iter([
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ]);
        for (&output_wire, &value) in expected.iter() {
            let result = result_day07_stage1(&instructions, output_wire);
            assert_eq!(result, value);
        }
    }
}
