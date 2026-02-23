use serde_json::Value;

pub fn result_day12_stage1(lines: &[String]) -> i64 {
    let mut total = 0;

    for line in lines {
        let bytes = line.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i].is_ascii_digit() || bytes[i] == b'-' {
                let mut sign = 1;

                if bytes[i] == b'-' {
                    sign = -1;
                    i += 1;
                }

                let mut value = 0i64;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    value = value * 10 + (bytes[i] - b'0') as i64;
                    i += 1;
                }
                total += sign * value;
            } else {
                i += 1;
            }
        }
    }

    total
}

pub fn result_day12_stage2(lines: &[String]) -> i64 {
    let mut total = 0;
    for line in lines {
        let value: Value = serde_json::from_str(line.as_str()).unwrap();
        total += sum_json(&value);
    }
    total
}

fn sum_json(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_json).sum(),
        Value::Object(obj) => {
            if obj
                .values()
                .any(|v| matches!(v, Value::String(s) if s == "red"))
            {
                0
            } else {
                obj.values().map(sum_json).sum()
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod day12 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            ("[1,2,3]", 6),
            ("{\"a\":2,\"b\":4}", 6),
            ("[[[3]]]", 3),
            ("{\"a\":{\"b\":4},\"c\":-1}", 3),
            ("[]", 0),
            ("{}", 0),
        ];
        for (input, expected) in tests {
            let lines = vec![input.to_string()];
            let result = result_day12_stage1(&lines);
            assert_eq!(result, expected, "input: {input}");
        }
    }

    #[test]
    fn stage2() {
        let tests = [
            ("[1,2,3]", 6),
            ("[1,{\"c\":\"blue\",\"b\":2},3]", 6),
            ("[1,{\"c\":\"red\",\"b\":2},3]", 4),
            ("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0),
            ("[1,\"red\",5]", 6),
        ];
        for (input, expected) in tests {
            let lines = vec![input.to_string()];
            let result = result_day12_stage2(&lines);
            assert_eq!(result, expected, "input: {input}");
        }
    }
}
