pub fn result_day08_stage1(lines: &[String]) -> usize {
    let mut memory_len = 0;
    let mut string_len = 0;
    for line in lines.iter() {
        let bytes = line.as_bytes();
        string_len += bytes.len();
        let mut i = 1; // skip the initial '"' character
        while i < bytes.len() - 1 {
            if bytes[i] == b'\\' {
                if bytes[i + 1] == b'x' {
                    memory_len += 1;
                    i += 4;
                } else {
                    memory_len += 1;
                    i += 2;
                }
            } else {
                memory_len += 1;
                i += 1;
            }
        }
    }
    string_len - memory_len
}

pub fn result_day08_stage2(lines: &[String]) -> usize {
    let mut encoded_total = 0;
    let mut string_total = 0;
    for line in lines.iter() {
        let bytes = line.as_bytes();
        string_total += bytes.len();
        let mut new_len = 2; // start and end ""
        for &byte in bytes {
            if byte == b'\\' || byte == b'\"' {
                new_len += 2;
            } else {
                new_len += 1;
            }
        }
        encoded_total += new_len;
    }

    encoded_total - string_total
}

#[cfg(test)]
mod day08 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from(r#""""#),         // ""
            String::from(r#""abc""#),      // "abc"
            String::from(r#""aaa\"aaa""#), // "aaa\"aaa"
            String::from(r#""\x27""#),     // "\x27"
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day08_stage1(&get_example());
        assert_eq!(result, 12);
    }

    #[test]
    fn stage2() {
        let result = result_day08_stage2(&get_example());
        assert_eq!(result, 19);
    }
}
