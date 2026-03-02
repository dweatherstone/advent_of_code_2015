// #![allow(dead_code)]

use std::{fmt::Display, fs::read_to_string, path::Path, time::Instant};

#[allow(unused_imports)]
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    day01::{result_day01_stage1, result_day01_stage2},
    day02::{parse_day02, result_day02_stage1, result_day02_stage2},
    day03::{result_day03_stage1, result_day03_stage2},
    day04::{result_day04_stage1, result_day04_stage2},
    day05::{result_day05_stage1, result_day05_stage2},
    day06::{result_day06_stage1, result_day06_stage2},
    day07::{parse_day07, result_day07_stage1, result_day07_stage2},
    day08::{result_day08_stage1, result_day08_stage2},
    day09::{parse_day09, result_day09_stage1, result_day09_stage2},
    day10::result_day10,
    day11::result_day11,
    day12::{result_day12_stage1, result_day12_stage2},
    day13::{result_day13_stage1, result_day13_stage2},
    day14::{result_day14_stage1, result_day14_stage2},
    day15::{result_day15_stage1, result_day15_stage2},
    day16::{result_day16_stage1, result_day16_stage2},
    day17::{result_day17_stage1, result_day17_stage2},
    day18::{result_day18_stage1, result_day18_stage2},
    day19::{result_day19_stage1, result_day19_stage2},
};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;

fn get_lines(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    // for day in Days::iter() {
    //     day.run(true);
    // }
    Days::Day19.run(true);
}

#[derive(EnumIter)]
enum Days {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
}

impl Days {
    fn get_results(&self) {
        use Days::*;
        match self {
            // Process from an input file
            Day01 | Day03 | Day05 | Day06 | Day08 | Day12 | Day13 | Day15 | Day16 | Day17
            | Day18 | Day19 => {
                let lines = get_lines(Path::new(&self.get_path_str()));
                let result1 = self.get_result1_from_lines(&lines);
                println!("{self} stage 1: {result1}");
                let result2 = self.get_result2_from_lines(&lines);
                println!("{self} stage 2: {result2}");
            } // Process from a single input
            Day04 => {
                let input = "iwrupvqb";
                let result1 = result_day04_stage1(input);
                println!("{self} stage 1: {result1}");
                let result2 = result_day04_stage2(input);
                println!("{self} stage 2: {result2}");
            }
            Day10 => {
                let input = "1321131112";
                let result1 = result_day10(input, 40);
                println!("{self} stage 1: {result1}");
                let result2 = result_day10(input, 50);
                println!("{self} stage 2: {result2}");
            }
            Day11 => {
                let input = "cqjxjnds";
                let result1 = result_day11(input);
                println!("{self} stage 1: {result1}");
                let result2 = result_day11(&result1);
                println!("{self} stage 2: {result2}");
            }
            Day14 => {
                let total_time = 2503;
                let result1 =
                    result_day14_stage1(&get_lines(Path::new("input/day14_input.txt")), total_time);
                println!("{self} stage 1: {result1}");
                let result2 =
                    result_day14_stage2(&get_lines(Path::new("input/day14_input.txt")), total_time);
                println!("{self} stage 2: {result2}");
            }
            // Produce a struct from input, then process
            Day02 => {
                let presents = parse_day02(&get_lines(Path::new("input/day02_input.txt")));
                let result1 = result_day02_stage1(&presents);
                println!("Day 2 stage 1: {result1}");
                let result2 = result_day02_stage2(&presents);
                println!("Day 2 stage 2: {result2}");
            }
            Day07 => {
                let instructions = parse_day07(&get_lines(Path::new("input/day07_input.txt")));
                let result1 = result_day07_stage1(&instructions, "a");
                println!("Day 7 stage 1: {result1}");
                let result2 = result_day07_stage2(&instructions, "a", result1);
                println!("Day 7 stage 2: {result2}");
            }
            Day09 => {
                let graph = parse_day09(&get_lines(Path::new("input/day09_input.txt")));
                let result1 = result_day09_stage1(&graph);
                println!("Day 9 stage 1: {result1}");
                let result2 = result_day09_stage2(&graph);
                println!("Day 9 stage 2: {result2}");
            }
        }
    }

    fn get_path_str(&self) -> String {
        use Days::*;
        let filename = match self {
            Day01 => "day01_input.txt",
            Day03 => "day03_input.txt",
            Day05 => "day05_input.txt",
            Day06 => "day06_input.txt",
            Day08 => "day08_input.txt",
            Day12 => "day12_input.txt",
            Day13 => "day13_input.txt",
            Day14 => "day14_input.txt",
            Day15 => "day15_input.txt",
            Day16 => "day16_input.txt",
            Day17 => "day17_input.txt",
            Day18 => "day18_input.txt",
            Day19 => "day19_input.txt",
            _ => panic!("undefined path string"),
        };
        format!("input/{filename}")
    }

    fn get_result1_from_lines(&self, lines: &[String]) -> Box<dyn Display> {
        use Days::*;
        match self {
            Day01 => Box::new(result_day01_stage1(lines)),
            Day03 => Box::new(result_day03_stage1(lines)),
            Day05 => Box::new(result_day05_stage1(lines)),
            Day06 => Box::new(result_day06_stage1(lines)),
            Day08 => Box::new(result_day08_stage1(lines)),
            Day12 => Box::new(result_day12_stage1(lines)),
            Day13 => Box::new(result_day13_stage1(lines)),
            Day15 => Box::new(result_day15_stage1(lines)),
            Day16 => Box::new(result_day16_stage1(lines)),
            Day17 => Box::new(result_day17_stage1(lines)),
            Day18 => Box::new(result_day18_stage1(lines)),
            Day19 => Box::new(result_day19_stage1(lines)),
            _ => panic!("undefined result1 function"),
        }
    }

    fn get_result2_from_lines(&self, lines: &[String]) -> Box<dyn Display> {
        use Days::*;
        match self {
            Day01 => Box::new(result_day01_stage2(lines)),
            Day03 => Box::new(result_day03_stage2(lines)),
            Day05 => Box::new(result_day05_stage2(lines)),
            Day06 => Box::new(result_day06_stage2(lines)),
            Day08 => Box::new(result_day08_stage2(lines)),
            Day12 => Box::new(result_day12_stage2(lines)),
            Day13 => Box::new(result_day13_stage2(lines)),
            Day15 => Box::new(result_day15_stage2(lines)),
            Day16 => Box::new(result_day16_stage2(lines)),
            Day17 => Box::new(result_day17_stage2(lines)),
            Day18 => Box::new(result_day18_stage2(lines)),
            Day19 => Box::new(result_day19_stage2(lines)),
            _ => panic!("undefined result2 function"),
        }
    }

    fn run(&self, with_timing: bool) {
        let start = if with_timing {
            Some(Instant::now())
        } else {
            None
        };
        self.get_results();
        if with_timing {
            let duration = start.unwrap().elapsed();
            println!("{self} time taken: {:?}\n", duration);
        }
    }
}

impl Display for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Days::*;
        match self {
            Day01 => write!(f, "Day 1"),
            Day02 => write!(f, "Day 2"),
            Day03 => write!(f, "Day 3"),
            Day04 => write!(f, "Day 4"),
            Day05 => write!(f, "Day 5"),
            Day06 => write!(f, "Day 6"),
            Day07 => write!(f, "Day 7"),
            Day08 => write!(f, "Day 8"),
            Day09 => write!(f, "Day 9"),
            Day10 => write!(f, "Day 10"),
            Day11 => write!(f, "Day 11"),
            Day12 => write!(f, "Day 12"),
            Day13 => write!(f, "Day 13"),
            Day14 => write!(f, "Day 14"),
            Day15 => write!(f, "Day 15"),
            Day16 => write!(f, "Day 16"),
            Day17 => write!(f, "Day 17"),
            Day18 => write!(f, "Day 18"),
            Day19 => write!(f, "Day 19"),
        }
    }
}
