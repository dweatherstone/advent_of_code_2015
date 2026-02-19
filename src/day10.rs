pub fn result_day10(input: &str, n: u32) -> usize {
    let mut current = input.as_bytes().to_vec();

    for _ in 0..n {
        let mut next = Vec::with_capacity(current.len() * 2);

        let mut current_byte = current[0];
        let mut count = 1;
        for &b in &current[1..] {
            if b == current_byte {
                count += 1;
            } else {
                next.extend_from_slice(count.to_string().as_bytes());
                next.push(current_byte);
                current_byte = b;
                count = 1;
            }
        }
        next.extend_from_slice(count.to_string().as_bytes());
        next.push(current_byte);

        current = next;
    }

    current.len()
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn stage1() {
        let mut current = "1";
        let expected = ["11", "21", "1211", "111221", "312211"];
        for &expected_str in expected.iter() {
            let str_len = result_day10(current, 1);
            assert_eq!(str_len, expected_str.len());
            current = expected_str;
        }
    }
}
