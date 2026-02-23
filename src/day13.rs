use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn result_day13_stage1(lines: &[String]) -> i64 {
    let (names, happiness) = parse_lines(lines);
    let n = names.len();
    get_score(&happiness, n)
}

pub fn result_day13_stage2(lines: &[String]) -> i64 {
    let (mut names, mut happiness) = parse_lines(lines);
    // Add "You"
    names.push("You".to_string());
    let n = names.len();
    happiness.iter_mut().for_each(|row| row.push(0));
    happiness.push(vec![0; n]);
    get_score(&happiness, n)
}

fn parse_lines(lines: &[String]) -> (Vec<String>, Vec<Vec<i64>>) {
    // E.g. "Alice would gain 54 happiness units by sitting next to Bob."
    let mut names_set: HashSet<String> = HashSet::new();
    for line in lines {
        let words: Vec<_> = line.split_whitespace().collect();
        let name1 = words[0].to_string();
        names_set.insert(name1);
        let name2 = words[10].trim_end_matches(".").to_string();
        names_set.insert(name2);
    }
    let mut names: Vec<String> = names_set.iter().cloned().collect();
    names.sort();
    let mut name_to_idx = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        name_to_idx.insert(name.clone(), i);
    }
    let mut happiness = vec![vec![i64::MIN; names.len()]; names.len()];
    for line in lines {
        let words: Vec<_> = line.split_whitespace().collect();
        let name1_idx = *name_to_idx.get(words[0]).unwrap();
        let name2_idx = *name_to_idx.get(words[10].trim_end_matches(".")).unwrap();
        let sign = if words[2] == "gain" { 1 } else { -1 };
        let amount = words[3].parse::<i64>().unwrap();
        happiness[name1_idx][name2_idx] = sign * amount;
    }
    (names, happiness)
}

fn get_score(happiness: &[Vec<i64>], n: usize) -> i64 {
    let indices: Vec<usize> = (0..n).collect();

    let first = indices[0];
    let others = &indices[1..];

    let mut max_score = 0;

    for perm in others.iter().permutations(others.len()) {
        let mut seating = vec![first];
        seating.extend(perm.into_iter().copied());
        // compute score
        let score: i64 = (0..n)
            .map(|i| {
                let left = seating[i];
                let right = seating[(i + 1) % seating.len()]; // wrap around
                happiness[left][right] + happiness[right][left]
            })
            .sum();
        if score > max_score {
            max_score = score;
        }
    }
    max_score
}

#[cfg(test)]
mod day13 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("Alice would gain 54 happiness units by sitting next to Bob."),
            String::from("Alice would lose 79 happiness units by sitting next to Carol."),
            String::from("Alice would lose 2 happiness units by sitting next to David."),
            String::from("Bob would gain 83 happiness units by sitting next to Alice."),
            String::from("Bob would lose 7 happiness units by sitting next to Carol."),
            String::from("Bob would lose 63 happiness units by sitting next to David."),
            String::from("Carol would lose 62 happiness units by sitting next to Alice."),
            String::from("Carol would gain 60 happiness units by sitting next to Bob."),
            String::from("Carol would gain 55 happiness units by sitting next to David."),
            String::from("David would gain 46 happiness units by sitting next to Alice."),
            String::from("David would lose 7 happiness units by sitting next to Bob."),
            String::from("David would gain 41 happiness units by sitting next to Carol."),
        ]
    }

    #[test]
    fn parse() {
        let (names, happiness) = parse_lines(&get_example());
        let expected_names = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
            "David".to_string(),
        ];
        let expected_happiness = vec![
            vec![i64::MIN, 54, -79, -2],
            vec![83, i64::MIN, -7, -63],
            vec![-62, 60, i64::MIN, 55],
            vec![46, -7, 41, i64::MIN],
        ];
        assert_eq!(names, expected_names);
        assert_eq!(happiness, expected_happiness);
    }

    #[test]
    fn stage1() {
        let result = result_day13_stage1(&get_example());
        assert_eq!(result, 330);
    }
}
