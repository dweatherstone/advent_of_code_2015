use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

pub fn result_day19_stage1(lines: &[String]) -> usize {
    let (transformations, initial) = parse_day19(lines);
    let final_strings = single_replacements(&transformations, &initial);
    final_strings.len()
}

pub fn result_day19_stage2(lines: &[String]) -> usize {
    let (mut transformations, medicine) = parse_day19(lines);
    // Sort by end_str.len() descending
    transformations.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    a_star_make_medicine(medicine, &transformations)
}

fn parse_day19(lines: &[String]) -> (Vec<(String, String)>, String) {
    let mut transforamtions = Vec::new();
    let medicine = lines.last().unwrap().to_owned();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((start, end)) = line.split_once(" => ") {
            transforamtions.push((String::from(start), String::from(end)));
        }
    }

    (transforamtions, medicine)
}

fn single_replacements(
    transformations: &[(String, String)],
    initial_medicine: &str,
) -> HashSet<String> {
    let mut final_strings: HashSet<String> = HashSet::new();
    for (start_str, end_str) in transformations {
        for (idx, _) in initial_medicine.match_indices(start_str.as_str()) {
            let new_medicine = format!(
                "{}{}{}",
                &initial_medicine[..idx],
                end_str,
                &initial_medicine[(idx + start_str.len())..]
            );
            final_strings.insert(new_medicine);
        }
    }
    final_strings
}

#[derive(Eq, PartialEq)]
struct State {
    f_score: usize,
    steps: usize,
    medicine: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star_make_medicine(initial: String, transformations: &[(String, String)]) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(State {
        f_score: initial.len(), // Initial heuristic
        steps: 0,
        medicine: initial,
    });

    while let Some(State {
        steps, medicine, ..
    }) = queue.pop()
    {
        if medicine == "e" {
            return steps;
        }

        for (start_str, end_str) in transformations {
            for (idx, _) in medicine.match_indices(end_str) {
                let next_str = format!(
                    "{}{}{}",
                    &medicine[..idx],
                    start_str,
                    &medicine[(idx + end_str.len())..]
                );
                if !visited.contains(&next_str) {
                    // h(n) = length of the string (crude but admissable)
                    let h = next_str.len();
                    queue.push(State {
                        f_score: (steps + 1) + h,
                        steps: steps + 1,
                        medicine: next_str.clone(),
                    });
                    visited.insert(next_str);
                }
            }
        }
    }
    0
}

/// This works backwards from the target molecule to the initial state to find the path.
/// The number of possibilities soon explodes if attempted in the opposite direction.
// fn make_medicine(
//     current: String,
//     target: &str,
//     steps: usize,
//     transformations: &[(String, String)],
// ) -> Option<usize> {
//     if &current == target {
//         return Some(steps);
//     }
//     if current.len() < target.len() {
//         return None;
//     }
//     for (start_str, end_str) in transformations {
//         for (idx, _) in current.match_indices(end_str) {
//             let next_medicine = format!(
//                 "{}{}{}",
//                 &current[..idx],
//                 start_str,
//                 &current[(idx + end_str.len())..]
//             );
//             if let Some(final_steps) =
//                 make_medicine(next_medicine, target, steps + 1, transformations)
//             {
//                 return Some(final_steps);
//             }
//         }
//     }
//     None
// }

#[cfg(test)]
mod day19 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("e => H"),
            String::from("e => O"),
            String::from("H => HO"),
            String::from("H => OH"),
            String::from("O => HH"),
            String::from(""),
            String::from("HOH"),
        ]
    }

    #[test]
    fn parse() {
        let (transformations, medicine) = parse_day19(&get_example());
        let expected_transformations = vec![
            (String::from("e"), String::from("H")),
            (String::from("e"), String::from("O")),
            (String::from("H"), String::from("HO")),
            (String::from("H"), String::from("OH")),
            (String::from("O"), String::from("HH")),
        ];
        let expected_medicine = String::from("HOH");
        assert_eq!(transformations, expected_transformations);
        assert_eq!(medicine, expected_medicine);
    }

    #[test]
    fn stage1() {
        let result = result_day19_stage1(&get_example());
        assert_eq!(result, 4);
    }

    #[test]
    fn stage1_hoh() {
        let (transformations, medicine) = parse_day19(&get_example());
        let after = single_replacements(&transformations, &medicine);
        println!("After: {:?}", after);
        assert_eq!(after.len(), 4);
    }

    #[test]
    fn stage1_santas_favourite() {
        let (transformations, _) = parse_day19(&get_example());
        let medicine = String::from("HOHOHO");
        let after = single_replacements(&transformations, &medicine);
        println!("After: {:?}", after);
        assert_eq!(after.len(), 7);
    }

    #[test]
    fn stage2() {
        let result = result_day19_stage2(&get_example());
        assert_eq!(result, 3);
    }

    #[test]
    fn stage2_santas_favourite() {
        let (transformations, _) = parse_day19(&get_example());
        let medicine = String::from("HOHOHO");
        let after = a_star_make_medicine(medicine, &transformations);
        assert_eq!(after, 6);
    }
}
