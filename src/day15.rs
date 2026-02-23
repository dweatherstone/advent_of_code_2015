pub fn result_day15_stage1(lines: &[String]) -> i64 {
    let ingredients = parse_day15(lines);
    let n = ingredients.len();
    let mut amounts = vec![0u32; n];
    let mut max_score = 0i64;
    search(&ingredients, 0, 100, false, &mut amounts, &mut max_score);
    max_score
}

pub fn result_day15_stage2(lines: &[String]) -> i64 {
    let ingredients = parse_day15(lines);
    let n = ingredients.len();
    let mut amounts = vec![0u32; n];
    let mut max_score = 0i64;
    search(&ingredients, 0, 100, true, &mut amounts, &mut max_score);
    max_score
}

fn parse_day15(lines: &[String]) -> Vec<Ingredient> {
    let mut ingredients = Vec::new();
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let name = words[0].trim_end_matches(":").to_string();
        let capacity = words[2].trim_end_matches(",").parse().unwrap();
        let durability = words[4].trim_end_matches(",").parse().unwrap();
        let flavor = words[6].trim_end_matches(",").parse().unwrap();
        let texture = words[8].trim_end_matches(",").parse().unwrap();
        let calories = words[10].parse().unwrap();
        ingredients.push(Ingredient {
            _name: name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }

    ingredients
}

fn search(
    ingredients: &[Ingredient],
    index: usize,
    remaining_teaspoons: u32,
    check_calories: bool,
    amounts: &mut Vec<u32>,
    max_score: &mut i64,
) {
    // Last ingredient
    if index == ingredients.len() - 1 {
        amounts[index] = remaining_teaspoons;
        if check_calories
            && ingredients
                .iter()
                .zip(amounts.iter())
                .map(|(ing, &am)| ing.calories * am as i32)
                .sum::<i32>()
                != 500
        {
            return;
        }
        let score = compute_score(ingredients, amounts);
        if score > *max_score {
            *max_score = score;
        }
        return;
    }
    for x in 0..=remaining_teaspoons {
        amounts[index] = x;
        search(
            ingredients,
            index + 1,
            remaining_teaspoons - x,
            check_calories,
            amounts,
            max_score,
        );
    }
}

fn compute_score(ingredients: &[Ingredient], amounts: &[u32]) -> i64 {
    let mut total_capacity = 0;
    let mut total_durability = 0;
    let mut total_flavor = 0;
    let mut total_texture = 0;
    for (ingredient, &amount) in ingredients.iter().zip(amounts) {
        total_capacity += ingredient.capacity * amount as i32;
        total_durability += ingredient.durability * amount as i32;
        total_flavor += ingredient.flavor * amount as i32;
        total_texture += ingredient.texture * amount as i32;
    }
    total_capacity = total_capacity.max(0);
    total_durability = total_durability.max(0);
    total_flavor = total_flavor.max(0);
    total_texture = total_texture.max(0);
    total_capacity as i64 * total_durability as i64 * total_flavor as i64 * total_texture as i64
}

#[derive(Debug, PartialEq)]
struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[cfg(test)]
mod day15 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            ),
            String::from("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"),
        ]
    }

    #[test]
    fn parse() {
        let ingredients = parse_day15(&get_example());
        let expected = vec![
            Ingredient {
                _name: String::from("Butterscotch"),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                _name: String::from("Cinnamon"),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ];
        assert_eq!(ingredients, expected);
    }

    #[test]
    fn stage1() {
        let result = result_day15_stage1(&get_example());
        assert_eq!(result, 62842880);
    }

    #[test]
    fn stage2() {
        let result = result_day15_stage2(&get_example());
        assert_eq!(result, 57600000);
    }
}
