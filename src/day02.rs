pub struct Present {
    length: u32,
    width: u32,
    height: u32,
    smallest_side: (u32, u32),
}

impl Present {
    fn new(length: u32, width: u32, height: u32) -> Self {
        let mut dimensions = [length, width, height];
        dimensions.sort();
        let smallest_side = (dimensions[0], dimensions[1]);
        Present {
            length,
            width,
            height,
            smallest_side,
        }
    }

    fn get_surface_area(&self) -> u64 {
        (2 * self.length * self.width
            + 2 * self.width * self.height
            + 2 * self.height * self.length) as u64
    }

    fn get_smallest_side_area(&self) -> u64 {
        self.smallest_side.0 as u64 * self.smallest_side.1 as u64
    }

    fn get_smallest_side_perimiter(&self) -> u64 {
        2 * (self.smallest_side.0 as u64 + self.smallest_side.1 as u64)
    }

    fn get_volume(&self) -> u64 {
        self.length as u64 * self.width as u64 * self.height as u64
    }
}

pub fn parse_day02(lines: &[String]) -> Vec<Present> {
    let mut presents = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts = line.split("x").collect::<Vec<_>>();
        assert_eq!(parts.len(), 3, "line is malformed: {}", line);
        let length = parts[0].trim().parse().expect("should be an integer");
        let width = parts[1].trim().parse().expect("should be an integer");
        let height = parts[2].trim().parse().expect("should be an integer");
        presents.push(Present::new(length, width, height));
    }

    presents
}

pub fn result_day02_stage1(presents: &[Present]) -> u64 {
    presents
        .iter()
        .map(|present| present.get_surface_area() + present.get_smallest_side_area())
        .sum()
}

pub fn result_day02_stage2(presents: &[Present]) -> u64 {
    presents
        .iter()
        .map(|present| present.get_smallest_side_perimiter() + present.get_volume())
        .sum()
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn stage1() {
        let tests = [((2, 3, 4), 58), ((1, 1, 10), 43)];
        for ((l, w, h), expected) in tests {
            let present = Present::new(l, w, h);
            let result = result_day02_stage1(&[present]);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn stage2() {
        let tests = [((2, 3, 4), 34), ((1, 1, 10), 14)];
        for ((l, w, h), expected) in tests {
            let present = Present::new(l, w, h);
            let result = result_day02_stage2(&[present]);
            assert_eq!(result, expected);
        }
    }
}
