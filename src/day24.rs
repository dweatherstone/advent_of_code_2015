use itertools::Itertools;

pub fn result_day24_stage1(lines: &[String]) -> i64 {
    run_day24(lines, 3)
}

pub fn result_day24_stage2(lines: &[String]) -> i64 {
    run_day24(lines, 4)
}

fn run_day24(lines: &[String], num_groups: i64) -> i64 {
    let presents: Vec<i64> = lines
        .iter()
        .map(|line| line.trim().parse::<i64>().expect("should be a number"))
        .collect();
    let target = presents.iter().sum::<i64>() / num_groups;
    for k in 1..presents.len() {
        let entanglement: Vec<i64> = presents
            .iter()
            .combinations(k)
            .filter(|combo| combo.iter().copied().sum::<i64>() == target)
            .map(|combo| combo.iter().copied().product::<i64>())
            .collect();
        if !entanglement.is_empty() {
            return entanglement.into_iter().min().unwrap();
        }
    }
    unreachable!("unable to find suitable groups")
}

#[cfg(test)]
mod day24 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("7"),
            String::from("8"),
            String::from("9"),
            String::from("10"),
            String::from("11"),
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day24_stage1(&get_example());
        assert_eq!(result, 99);
    }

    #[test]
    fn stage2() {
        let result = result_day24_stage2(&get_example());
        assert_eq!(result, 44);
    }
}
