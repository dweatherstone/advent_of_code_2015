use std::collections::HashSet;

pub fn result_day03_stage1(lines: &[String]) -> usize {
    let line = lines.join("");
    let mut position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(position);
    for ch in line.chars() {
        position = new_position(ch, position);
        visited.insert(position);
    }
    visited.len()
}

pub fn result_day03_stage2(lines: &[String]) -> usize {
    let line: Vec<char> = lines.join("").chars().collect();
    let mut santa_position = (0, 0);
    let mut robo_position = (0, 0);
    let mut santa_visited = HashSet::new();
    let mut robo_visited = HashSet::new();
    santa_visited.insert(santa_position);
    robo_visited.insert(robo_position);
    for chars in line.chunks(2) {
        santa_position = new_position(chars[0], santa_position);
        santa_visited.insert(santa_position);
        robo_position = new_position(chars[1], robo_position);
        robo_visited.insert(robo_position);
    }

    let all_visited: HashSet<(i32, i32)> = santa_visited.union(&robo_visited).cloned().collect();
    all_visited.len()
}

fn new_position(direction: char, position: (i32, i32)) -> (i32, i32) {
    match direction {
        '>' => (position.0 + 1, position.1),
        '<' => (position.0 - 1, position.1),
        'v' => (position.0, position.1 + 1),
        '^' => (position.0, position.1 - 1),
        _ => panic!("unexpected character: {direction}"),
    }
}

#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            ([">".to_string()], 2),
            (["^>v<".to_string()], 4),
            (["^v^v^v^v^v".to_string()], 2),
        ];
        for (input, expected) in tests.iter() {
            let result = result_day03_stage1(input);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn stage2() {
        let tests = [
            (["^v".to_string()], 3),
            (["^>v<".to_string()], 3),
            (["^v^v^v^v^v".to_string()], 11),
        ];
        for (input, expected) in tests.iter() {
            let result = result_day03_stage2(input);
            assert_eq!(result, *expected);
        }
    }
}
