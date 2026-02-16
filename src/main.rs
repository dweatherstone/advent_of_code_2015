#![allow(dead_code)]

use std::{fs::read_to_string, path::Path};

use crate::{
    day01::{result_day01_stage1, result_day01_stage2},
    day02::{parse_day02, result_day02_stage1, result_day02_stage2},
    day03::{result_day03_stage1, result_day03_stage2},
    day04::{result_day04_stage1, result_day04_stage2},
    day05::{result_day05_stage1, result_day05_stage2},
    day06::{result_day06_stage1, result_day06_stage2},
    day07::{parse_day07, result_day07_stage1, result_day07_stage2},
};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

fn get_lines(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn day01() {
    let lines = get_lines(Path::new("input/day01_input.txt"));
    let result1 = result_day01_stage1(&lines);
    println!("Day 1 stage 1: {result1}");
    let result2 = result_day01_stage2(&lines);
    println!("Day 1 stage 2: {result2}");
}

fn day02() {
    let presents = parse_day02(&get_lines(Path::new("input/day02_input.txt")));
    let result1 = result_day02_stage1(&presents);
    println!("Day 2 stage 1: {result1}");
    let result2 = result_day02_stage2(&presents);
    println!("Day 2 stage 2: {result2}");
}

fn day03() {
    let result1 = result_day03_stage1(&get_lines(Path::new("input/day03_input.txt")));
    println!("Day 3 stage 1: {result1}");
    let result2 = result_day03_stage2(&get_lines(Path::new("input/day03_input.txt")));
    println!("Day 3 stage 2: {result2}");
}

fn day04() {
    let result1 = result_day04_stage1("iwrupvqb");
    println!("Day 4 stage 1: {result1}");
    let result2 = result_day04_stage2("iwrupvqb");
    println!("Day 4 stage 2: {result2}");
}

fn day05() {
    let result1 = result_day05_stage1(&get_lines(Path::new("input/day05_input.txt")));
    println!("Day 5 stage 1: {result1}");
    let result2 = result_day05_stage2(&get_lines(Path::new("input/day05_input.txt")));
    println!("Day 5 stage 2: {result2}");
}

fn day06() {
    let result1 = result_day06_stage1(&get_lines(Path::new("input/day06_input.txt")));
    println!("Day 6 stage 1: {result1}");
    let result2 = result_day06_stage2(&get_lines(Path::new("input/day06_input.txt")));
    println!("Day 6 stage 2: {result2}");
}

fn day07() {
    let instructions = parse_day07(&get_lines(Path::new("input/day07_input.txt")));
    let result1 = result_day07_stage1(&instructions, "a");
    println!("Day 7 stage 1: {result1}");
    let result2 = result_day07_stage2(&instructions, "a", result1);
    println!("Day 7 stage 2: {result2}");
}

fn main() {
    day07();
}
