use std::str::FromStr;

const GRID_SIZE: usize = 1000;
const GRID_AREA: usize = GRID_SIZE * GRID_SIZE;

pub fn result_day06_stage1(lines: &[String]) -> usize {
    let instructions: Vec<_> = lines.iter().map(|l| parse_line(l)).collect();
    let mut lights = vec![false; GRID_AREA];
    for instruction in &instructions {
        instruction.apply(&mut lights, |light, inst| {
            *light = match inst {
                InstructionType::Toggle => !*light,
                InstructionType::TurnOn => true,
                InstructionType::TurnOff => false,
            };
        });
    }

    lights.into_iter().filter(|l| *l).count()
}

pub fn result_day06_stage2(lines: &[String]) -> u64 {
    let instructions: Vec<_> = lines.iter().map(|l| parse_line(l)).collect();
    let mut lights = vec![0u32; GRID_AREA];
    for instruction in &instructions {
        instruction.apply(&mut lights, |light, inst| match inst {
            InstructionType::Toggle => *light += 2,
            InstructionType::TurnOn => *light += 1,
            InstructionType::TurnOff => *light = light.saturating_sub(1),
        });
    }
    lights.into_iter().map(|v| v as u64).sum()
}

fn index(x: usize, y: usize) -> usize {
    y * GRID_SIZE + x
}

fn parse_line(line: &str) -> Instruction {
    let (start_part, end_part) = line.split_once(" through ").expect("Missing 'through'");

    let mut parts = start_part.split_whitespace();

    let (instruction_type, start_coords) = match parts.next().unwrap() {
        "toggle" => (InstructionType::Toggle, parts.next().unwrap()),
        "turn" => {
            let on_off = parts.next().unwrap();
            let inst = format!("turn {}", on_off);
            (
                InstructionType::from_str(&inst).unwrap(),
                parts.next().unwrap(),
            )
        }
        _ => panic!("Invalid instruction"),
    };

    let (start_x, start_y) = parse_coords(start_coords);
    let (end_x, end_y) = parse_coords(end_part);

    Instruction {
        instruction_type,
        start_x,
        start_y,
        end_x,
        end_y,
    }
}

fn parse_coords(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

struct Instruction {
    instruction_type: InstructionType,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Instruction {
    fn apply<F, T>(&self, grid: &mut [T], mut f: F)
    where
        F: FnMut(&mut T, &InstructionType),
    {
        for y in self.start_y..=self.end_y {
            for x in self.start_x..=self.end_x {
                let idx = index(x, y);
                f(&mut grid[idx], &self.instruction_type);
            }
        }
    }
}

enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for InstructionType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "turn on" => Ok(InstructionType::TurnOn),
            "turn off" => Ok(InstructionType::TurnOff),
            "toggle" => Ok(InstructionType::Toggle),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod day06 {
    use super::*;

    #[test]
    fn stage1_turn_on_all() {
        let test_line = vec![String::from("turn on 0,0 through 999,999")];
        let result = result_day06_stage1(&test_line);
        assert_eq!(result, 1_000_000);
    }

    #[test]
    fn stage1_toggle_first_line() {
        let test_line = vec![String::from("toggle 0,0 through 999,0")];
        let result = result_day06_stage1(&test_line);
        assert_eq!(result, 1_000);
    }

    #[test]
    fn stage1_turn_off_middle_4() {
        let test_lines = vec![
            String::from("turn on 0,0 through 999,999"),
            String::from("turn off 499,499 through 500,500"),
        ];
        let result = result_day06_stage1(&test_lines);
        assert_eq!(result, 999_996);
    }

    #[test]
    fn stage2_turn_on() {
        let test_line = vec![String::from("turn on 0,0 through 0,0")];
        let result = result_day06_stage2(&test_line);
        assert_eq!(result, 1);
    }

    #[test]
    fn stage2_turn_off() {
        let test_lines = vec![
            String::from("turn on 0,0 through 0,0"),
            String::from("turn off 0,0 through 1,1"),
        ];
        let result = result_day06_stage2(&test_lines);
        assert_eq!(result, 0);
    }

    #[test]
    fn stage2_toggle() {
        let test_line = vec![String::from("toggle 0,0 through 999,999")];
        let result = result_day06_stage2(&test_line);
        assert_eq!(result, 2_000_000);
    }
}
