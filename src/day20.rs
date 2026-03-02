pub fn result_day20_stage1(target: u32) -> usize {
    // We need an estimate. Let's try 1 million.
    // If we don't find it, we'd need to increase this.
    let max_houses = (target as usize / 10).max(5);
    let mut houses = vec![0u32; max_houses];

    for elf in 1..max_houses {
        // This elf visits house 'elf', 'elf*2', 'elf*3', etc.
        // This is a simple step-by-loop
        for house in (elf..max_houses).step_by(elf) {
            houses[house] += (elf as u32) * 10;
        }
    }

    // Find the first house that meets the target
    houses
        .iter()
        .position(|&presents| presents >= target)
        .unwrap_or(0)
}

pub fn result_day20_stage2(target: u32) -> usize {
    let max_houses = (target as usize / 10).max(5);
    let mut houses = vec![0u32; max_houses];

    for elf in 1..max_houses {
        for house in (elf..max_houses).step_by(elf).take(50) {
            houses[house] += (elf as u32) * 11;
        }
    }
    houses
        .iter()
        .position(|&presents| presents >= target)
        .unwrap_or(0)
}

#[cfg(test)]
mod day20 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [
            (10, 1),
            (30, 2),
            (40, 3),
            (70, 4),
            (60, 4),
            (120, 6),
            (80, 6),
            (150, 8),
            (130, 8),
        ];
        for (input, expected_result) in tests {
            let result = result_day20_stage1(input);
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn stage2() {
        let tests = [(11, 1), (33, 2), (44, 3), (77, 4), (66, 4)];
        for (input, expected_result) in tests {
            let result = result_day20_stage2(input);
            assert_eq!(result, expected_result);
        }
    }
}
