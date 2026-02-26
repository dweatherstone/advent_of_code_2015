pub fn result_day17_stage1(lines: &[String]) -> usize {
    let containers = parse_day17(lines);
    let mut valid_counts = Vec::new();
    solve_recursive(&containers, 150, 0, 0, &mut valid_counts);
    valid_counts.len()
}

pub fn result_day17_stage2(lines: &[String]) -> usize {
    let containers = parse_day17(lines);
    let mut valid_counts = Vec::new();
    solve_recursive(&containers, 150, 0, 0, &mut valid_counts);

    let min_used = *valid_counts.iter().min().unwrap_or(&0);
    valid_counts.iter().filter(|&&c| c == min_used).count()
}

fn parse_day17(lines: &[String]) -> Vec<i32> {
    lines
        .iter()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn solve_recursive(
    containers: &[i32],
    target: i32,
    index: usize,
    containers_used: usize,
    results: &mut Vec<usize>,
) {
    // Base case: We hit the target exactly!
    if target == 0 {
        results.push(containers_used);
        return;
    }
    // Base case: We went over or ran out of containers
    if target < 0 || index >= containers.len() {
        return;
    }

    // Choice 1: Use the current container
    solve_recursive(
        containers,
        target - containers[index], // Reduce the target
        index + 1,                  // Move to next container
        containers_used + 1,        // We used one more container
        results,
    );

    // Choice 2: Skip the current container
    solve_recursive(
        containers,
        target,          // Target stays the same
        index + 1,       // Move to next container
        containers_used, // Count stays the same
        results,
    );
}

#[cfg(test)]
mod day17 {
    use super::*;

    #[test]
    fn stage1() {
        let containers = vec![20, 15, 10, 5, 5];
        let mut valid_counts = Vec::new();
        solve_recursive(&containers, 25, 0, 0, &mut valid_counts);
        assert_eq!(valid_counts.len(), 4);
    }

    #[test]
    fn stage2() {
        let containers = vec![20, 15, 10, 5, 5];
        let mut valid_counts = Vec::new();
        solve_recursive(&containers, 25, 0, 0, &mut valid_counts);
        let min_used = *valid_counts.iter().min().unwrap_or(&0);
        let result = valid_counts.iter().filter(|&&c| c == min_used).count();
        assert_eq!(result, 3);
    }
}
