pub fn result_day04_stage1(input: &str) -> u64 {
    for i in 0..u64::MAX {
        let test_str = format!("{}{}", input, i);
        let digest = format!("{:x}", md5::compute(test_str));
        if digest.starts_with("00000") {
            return i;
        }
    }
    0
}

pub fn result_day04_stage2(input: &str) -> u64 {
    for i in 0..u64::MAX {
        let test_str = format!("{}{}", input, i);
        let digest = format!("{:x}", md5::compute(test_str));
        if digest.starts_with("000000") {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn stage1() {
        assert_eq!(result_day04_stage1("abcdef"), 609043);
        assert_eq!(result_day04_stage1("pqrstuv"), 1048970);
    }
}
