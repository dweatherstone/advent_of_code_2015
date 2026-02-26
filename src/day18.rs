pub fn result_day18_stage1(lines: &[String]) -> usize {
    let mut lights = parse_day18(lines);
    simulate(&mut lights, 100, false);
    lights.iter().flatten().filter(|&&val| val).count()
}

pub fn result_day18_stage2(lines: &[String]) -> usize {
    let mut lights = parse_day18(lines);
    let dim = lights.len() - 1;
    lights[0][0] = true;
    lights[0][dim] = true;
    lights[dim][0] = true;
    lights[dim][dim] = true;
    simulate(&mut lights, 100, true);
    lights.iter().flatten().filter(|&&val| val).count()
}

fn parse_day18(lines: &[String]) -> Vec<Vec<bool>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unknown bulb state"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn get_neighbor_count(grid: &[Vec<bool>], row: usize, col: usize) -> usize {
    let mut count = 0;

    let start_row = row.saturating_sub(1);
    let end_row = (row + 1).min(grid.len() - 1);
    let start_col = col.saturating_sub(1);
    let end_col = (col + 1).min(grid[0].len() - 1);

    for r in start_row..=end_row {
        for c in start_col..=end_col {
            if r == row && c == col {
                continue; // skip self
            }
            if grid[r][c] {
                count += 1;
            }
        }
    }
    count
}

fn simulate(lights: &mut Vec<Vec<bool>>, n: u32, corners_stuck_on: bool) {
    let rows = lights.len();
    let cols = lights[0].len();
    let mut next_gen = lights.clone();

    for _ in 0..n {
        for r in 0..rows {
            for c in 0..cols {
                // If corner is broken, it stays true
                if corners_stuck_on && is_corner(r, c, rows, cols) {
                    next_gen[r][c] = true;
                    continue;
                }

                let neighbors_on = get_neighbor_count(lights, r, c);
                let is_on = lights[r][c];

                next_gen[r][c] = match (is_on, neighbors_on) {
                    (true, 2 | 3) => true, // stays on
                    (true, _) => false,    // dies
                    (false, 3) => true,    // born
                    (false, _) => false,   // stays off
                };
            }
        }
        std::mem::swap(lights, &mut next_gen);
    }
}

fn is_corner(r: usize, c: usize, rows: usize, cols: usize) -> bool {
    (r == 0 || r == rows - 1) && (c == 0 || c == cols - 1)
}

#[cfg(test)]
mod day18 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from(".#.#.#"),
            String::from("...##."),
            String::from("#....#"),
            String::from("..#..."),
            String::from("#.#..#"),
            String::from("####.."),
        ]
    }

    #[test]
    fn stage1() {
        let mut bulbs = parse_day18(&get_example());
        simulate(&mut bulbs, 4, false);
        let result = bulbs.iter().flatten().filter(|&&val| val).count();
        assert_eq!(result, 4);
    }

    #[test]
    fn stage2() {
        let mut bulbs = parse_day18(&get_example());
        let dim = bulbs.len() - 1;
        bulbs[0][0] = true;
        bulbs[0][dim] = true;
        bulbs[dim][0] = true;
        bulbs[dim][dim] = true;
        simulate(&mut bulbs, 5, true);
        let result = bulbs.iter().flatten().filter(|&&val| val).count();
        assert_eq!(result, 17);
    }
}
