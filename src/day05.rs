use std::collections::HashMap;

pub fn result_day05_stage1(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|line| {
            has_enough_vowels(line) && has_double_letter(line) && !contains_naughty_string(line)
        })
        .count()
}

pub fn result_day05_stage2(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|line| {
            has_two_double_letters(line.as_bytes()) && has_double_letter_with_gap(line.as_bytes())
        })
        .count()
}

fn has_enough_vowels(line: &str) -> bool {
    let mut count = 0;
    for b in line.as_bytes() {
        if matches!(b, b'a' | b'e' | b'i' | b'o' | b'u') {
            count += 1;
            if count >= 3 {
                return true;
            }
        }
    }
    false
}

fn has_double_letter(line: &str) -> bool {
    line.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn contains_naughty_string(line: &str) -> bool {
    line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
}

fn has_two_double_letters(chars: &[u8]) -> bool {
    let mut seen: HashMap<(u8, u8), usize> = HashMap::new();

    for (i, window) in chars.windows(2).enumerate() {
        let pair = (window[0], window[1]);

        if let Some(&prev_index) = seen.get(&pair) {
            if i >= prev_index + 2 {
                return true;
            }
        } else {
            seen.insert(pair, i);
        }
    }

    false
}

fn has_double_letter_with_gap(chars: &[u8]) -> bool {
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod day05 {
    use super::*;
    fn get_example1() -> Vec<String> {
        vec![
            String::from("ugknbfddgicrmopn"),
            String::from("aaa"),
            String::from("jchzalrnumimnmhp"),
            String::from("haegwjzuvuyypxyu"),
            String::from("dvszwmarrgswjxmb"),
        ]
    }

    #[test]
    fn enough_vowels() {
        let expected = [true, true, true, true, false];
        for (line, exp_result) in get_example1().iter().zip(expected) {
            let result = has_enough_vowels(line);
            assert_eq!(result, exp_result, "Incorrect line: '{line}'");
        }
    }

    #[test]
    fn double_letter() {
        let expected = [true, true, false, true, true];
        for (line, exp_result) in get_example1().iter().zip(expected) {
            let result = has_double_letter(line);
            assert_eq!(result, exp_result, "Incorrect line: '{line}'");
        }
    }

    #[test]
    fn naughty_string() {
        let expected = [false, false, false, true, false];
        for (line, exp_result) in get_example1().iter().zip(expected) {
            let result = contains_naughty_string(line);
            assert_eq!(result, exp_result, "Incorrect line: '{line}'");
        }
    }

    #[test]
    fn stage1() {
        let result = result_day05_stage1(&get_example1());
        assert_eq!(result, 2);
    }

    fn get_example2() -> Vec<String> {
        vec![
            String::from("xyxy"),
            String::from("aabcdefgaa"),
            String::from("xyx"),
            String::from("abcdefeghi"),
            String::from("aaa"),
            String::from("qjhvhtzxzqqjkmpb"),
            String::from("xxyxx"),
            String::from("uurcxstgmygtbstg"),
            String::from("ieodomkazucvgmuy"),
        ]
    }

    #[test]
    fn two_pairs() {
        let expected = [true, true, false, false, false, true, true, true, false];
        for (line, exp_result) in get_example2().iter().zip(expected) {
            let result = has_two_double_letters(line.as_bytes());
            assert_eq!(result, exp_result, "Incorrect line: '{line}'");
        }
    }

    #[test]
    fn double_letter_with_gap() {
        let expected = [true, false, true, true, true, true, true, false, true];
        for (line, exp_result) in get_example2().iter().zip(expected) {
            let result = has_double_letter_with_gap(line.as_bytes());
            assert_eq!(result, exp_result, "Incorrect line: '{line}'");
        }
    }

    #[test]
    fn stage2() {
        let result = result_day05_stage2(&get_example2());
        assert_eq!(result, 3);
    }
}
