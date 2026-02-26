use std::collections::HashMap;

pub fn result_day16_stage1(lines: &[String]) -> u16 {
    solve(lines, false)
}

pub fn result_day16_stage2(lines: &[String]) -> u16 {
    solve(lines, true)
}

fn get_target_profile() -> HashMap<&'static str, u8> {
    HashMap::from([
        ("children", 2),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ])
}

#[derive(Debug, PartialEq)]
struct Sue {
    number: u16,
    attributes: HashMap<String, u8>,
}

fn parse_day16(lines: &[String]) -> Vec<Sue> {
    lines
        .iter()
        .map(|line| {
            let (name, attrs) = line.split_once(": ").unwrap();
            let number = name["Sue ".len()..].parse().unwrap();

            let attributes = attrs
                .split(", ")
                .map(|attr| {
                    let (k, v) = attr.split_once(": ").unwrap();
                    (k.to_string(), v.parse().unwrap())
                })
                .collect();

            Sue { number, attributes }
        })
        .collect()
}

fn solve(lines: &[String], is_part_two: bool) -> u16 {
    let sues = parse_day16(lines);
    let target = get_target_profile();

    sues.into_iter()
        .find(|sue| {
            sue.attributes.iter().all(|(key, &val)| {
                let target_val = target[key.as_str()];
                if is_part_two {
                    match key.as_str() {
                        "cats" | "trees" => val > target_val,
                        "pomeranians" | "goldfish" => val < target_val,
                        _ => val == target_val,
                    }
                } else {
                    val == target_val
                }
            })
        })
        .map(|sue| sue.number)
        .unwrap_or(0)
}

#[cfg(test)]
mod day16 {
    use super::*;

    fn get_first_5_lines() -> Vec<String> {
        vec![
            String::from("Sue 1: goldfish: 6, trees: 9, akitas: 0"),
            String::from("Sue 2: goldfish: 7, trees: 1, akitas: 0"),
            String::from("Sue 3: cars: 10, akitas: 6, perfumes: 7"),
            String::from("Sue 4: perfumes: 2, vizslas: 0, cars: 6"),
            String::from("Sue 5: goldfish: 1, trees: 3, perfumes: 10"),
        ]
    }

    #[test]
    fn parse() {
        let sues = parse_day16(&get_first_5_lines());
        let expected = vec![
            Sue {
                number: 1,
                attributes: HashMap::from([
                    ("goldfish".to_string(), 6),
                    ("trees".to_string(), 9),
                    ("akitas".to_string(), 0),
                ]),
            },
            Sue {
                number: 2,
                attributes: HashMap::from([
                    ("goldfish".to_string(), 7),
                    ("trees".to_string(), 1),
                    ("akitas".to_string(), 0),
                ]),
            },
            Sue {
                number: 3,
                attributes: HashMap::from([
                    ("cars".to_string(), 10),
                    ("akitas".to_string(), 6),
                    ("perfumes".to_string(), 7),
                ]),
            },
            Sue {
                number: 4,
                attributes: HashMap::from([
                    ("perfumes".to_string(), 2),
                    ("vizslas".to_string(), 0),
                    ("cars".to_string(), 6),
                ]),
            },
            Sue {
                number: 5,
                attributes: HashMap::from([
                    ("goldfish".to_string(), 1),
                    ("trees".to_string(), 3),
                    ("perfumes".to_string(), 10),
                ]),
            },
        ];
        assert_eq!(sues, expected);
    }
}
