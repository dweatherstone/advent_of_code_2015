pub fn result_day11(previous: &str) -> String {
    let mut bytes: Vec<u8> = previous.as_bytes().to_vec();
    loop {
        increment(&mut bytes);
        if is_valid(&bytes) {
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn has_straight(s: &[u8]) -> bool {
    s.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn has_no_forbidden(s: &[u8]) -> bool {
    !s.iter().any(|&b| matches!(b, b'i' | b'o' | b'l'))
}

fn has_two_pairs(s: &[u8]) -> bool {
    let mut count = 0;
    let mut i = 0;

    while i < s.len() - 1 {
        if s[i] == s[i + 1] {
            count += 1;
            i += 2; // skip the pair (non-overlapping)
        } else {
            i += 1;
        }
    }

    count >= 2
}

fn is_valid(bytes: &[u8]) -> bool {
    has_straight(bytes) && has_no_forbidden(bytes) && has_two_pairs(bytes)
}

fn increment(s: &mut [u8]) {
    for i in (0..s.len()).rev() {
        if s[i] == b'z' {
            s[i] = b'a';
        } else {
            s[i] += 1;
            break;
        }
    }
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn has_straight_test() {
        let tests = [("hijklmmn", true), ("abbceffg", false), ("abbcegjk", false)];
        for (input, expected) in tests {
            let b = input.as_bytes();
            let result = has_straight(b);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn has_no_forbidden_test() {
        let tests = [("hijklmmn", false), ("abbceffg", true), ("abbcegjk", true)];
        for (input, expected) in tests {
            let b = input.as_bytes();
            let result = has_no_forbidden(b);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn has_two_pairs_test() {
        let tests = [("hijklmmn", false), ("abbceffg", true), ("abbcegjk", false)];
        for (input, expected) in tests {
            let b = input.as_bytes();
            let result = has_two_pairs(b);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn stage1() {
        let tests = [
            ("abcdefgh", "abcdffaa".to_string()),
            ("ghijklmn", "ghjaabcc".to_string()),
        ];
        for (input, expected) in tests {
            let result = result_day11(input);
            assert_eq!(result, expected);
        }
    }
}
