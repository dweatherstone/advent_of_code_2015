pub fn result_day01_stage1(lines: &[String]) -> i64 {
    let mut floor = 0;
    let line = lines.join("");
    for ch in line.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unknown character"),
        }
    }

    floor
}

pub fn result_day01_stage2(lines: &[String]) -> usize {
    let mut floor = 0;
    let line = lines.join("");
    for (idx, ch) in line.char_indices() {
        if floor < 0 {
            return idx;
        }
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unknown character"),
        }
    }
    line.len()
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            (vec!["(())".to_string()], 0i64),
            (vec!["()()".to_string()], 0i64),
            (vec!["(((".to_string()], 3i64),
            (vec!["(()(()(".to_string()], 3i64),
            (vec!["))(((((".to_string()], 3i64),
            (vec!["())".to_string()], -1i64),
            (vec!["))(".to_string()], -1i64),
            (vec![")))".to_string()], -3i64),
            (vec![")())())".to_string()], -3i64),
        ];
        for (input, expected) in tests.iter() {
            assert_eq!(result_day01_stage1(input), *expected);
        }
    }

    #[test]
    fn stage2() {
        let tests = [
            (vec![")".to_string()], 1usize),
            (vec!["()())".to_string()], 5usize),
        ];
        for (input, expected) in tests.iter() {
            assert_eq!(result_day01_stage2(input), *expected);
        }
    }
}
