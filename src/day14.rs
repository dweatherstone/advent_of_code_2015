pub fn result_day14_stage1(lines: &[String], total_time: u64) -> u64 {
    let reindeers = parse_lines(lines);

    reindeers
        .iter()
        .map(|r| {
            let cycle = r.flying_time + r.resting_time;
            let full_cycles = total_time / cycle;
            let remainder = total_time % cycle;

            let flying_time_total = full_cycles * r.flying_time + remainder.min(r.flying_time);

            flying_time_total * r.flying_speed
        })
        .max()
        .unwrap()
}

pub fn result_day14_stage2(lines: &[String], total_time: u64) -> u64 {
    let reindeers = parse_lines(lines);
    let n = reindeers.len();

    let mut distances = vec![0u64; n];
    let mut points = vec![0u64; n];

    for t in 0..total_time {
        for (i, r) in reindeers.iter().enumerate() {
            let cycle = r.flying_time + r.resting_time;

            if t % cycle < r.flying_time {
                distances[i] += r.flying_speed;
            }
        }

        let max_dist = distances.iter().max().unwrap();
        for (i, &d) in distances.iter().enumerate() {
            if d == *max_dist {
                points[i] += 1;
            }
        }
    }

    points.into_iter().max().unwrap()
}

fn parse_lines(lines: &[String]) -> Vec<Reindeer> {
    let mut reindeers = Vec::new();
    for line in lines {
        let words: Vec<_> = line.split_whitespace().collect();
        let name = words[0].to_string();
        let flying_speed = words[3].parse().unwrap();
        let flying_time = words[6].parse().unwrap();
        let resting_time = words[13].parse().unwrap();
        reindeers.push(Reindeer {
            _name: name,
            flying_speed,
            flying_time,
            resting_time,
        });
    }
    reindeers
}

struct Reindeer {
    _name: String,
    flying_speed: u64,
    flying_time: u64,
    resting_time: u64,
}

#[cfg(test)]
mod day14 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from(
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            ),
            String::from(
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            ),
        ]
    }

    #[test]
    fn stage1() {
        let result = result_day14_stage1(&get_example(), 1000);
        assert_eq!(result, 1120);
    }

    #[test]
    fn stage2() {
        let result = result_day14_stage2(&get_example(), 1000);
        assert_eq!(result, 689)
    }
}
