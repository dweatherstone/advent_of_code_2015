pub fn result_day25_stage1(lines: &[String]) -> i64 {
    let (row, col) = parse_day25(lines);
    get_value(row, col)
}

fn parse_day25(lines: &[String]) -> (i64, i64) {
    let mut row = 0;
    let mut row_done = false;
    if lines.len() != 1 {
        panic!("unexpected rows in input file");
    }
    for word in lines[0].split_whitespace() {
        if let Ok(value) = word.replace([',', '.'], "").parse::<i64>() {
            if !row_done {
                row = value;
                row_done = true;
            } else {
                return (row, value);
            }
        }
    }
    panic!("line does not contain two numbers")
}

fn get_value(row: i64, col: i64) -> i64 {
    let diagonal_number = row + col - 1;
    let triangle_number = ((diagonal_number - 1) * diagonal_number) / 2;
    let order_num = triangle_number + col;
    let mut value: i64 = 20151125;
    for _ in 0..order_num - 1 {
        value = (value * 252533) % 33554393;
    }
    value
}

#[cfg(test)]
mod day25 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            (1, 1, 20151125),
            (1, 2, 18749137),
            (1, 3, 17289845),
            (1, 4, 30943339),
            (1, 5, 10071777),
            (1, 6, 33511524),
            (2, 1, 31916031),
            (2, 2, 21629792),
            (2, 3, 16929656),
            (2, 4, 7726640),
            (2, 5, 15514188),
            (2, 6, 4041754),
            (3, 1, 16080970),
            (3, 2, 8057251),
            (3, 3, 1601130),
            (3, 4, 7981243),
            (3, 5, 11661866),
            (3, 6, 16474243),
            (4, 1, 24592653),
            (4, 2, 32451966),
            (4, 3, 21345942),
            (4, 4, 9380097),
            (4, 5, 10600672),
            (4, 6, 31527494),
            (5, 1, 77061),
            (5, 2, 17552253),
            (5, 3, 28094349),
            (5, 4, 6899651),
            (5, 5, 9250759),
            (5, 6, 31663883),
            (6, 1, 33071741),
            (6, 2, 6796745),
            (6, 3, 25397450),
            (6, 4, 24659492),
            (6, 5, 1534922),
            (6, 6, 27995004),
        ];
        for (row, col, expected) in tests {
            let result = get_value(row, col);
            assert_eq!(
                result, expected,
                "incorrect result for ({row}, {col}). Expected: {expected}. Got: {result}"
            );
        }
    }
}
