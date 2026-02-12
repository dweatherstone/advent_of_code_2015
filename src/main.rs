#![allow(dead_code)]

use std::{fs::read_to_string, path::Path};

use crate::day01::{result_day01_stage1, result_day01_stage2};

pub mod day01;

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

fn main() {
    day01();
}
