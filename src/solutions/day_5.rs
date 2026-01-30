use std::{cmp::max, fs::read_to_string};

pub fn solve_day_5_part_1() {
    let text = read_to_string("./data/input/day_5.txt").expect("Could not parse text file");
    let result = part_1(text);
    println!("Day 5 Part 1: {}", result);
}

fn part_1(text: String) -> i32 {
    let mut in_ranges = true;
    let mut ranges = Vec::new();
    let mut fresh_count = 0;

    for line in text.trim().lines() {
        if in_ranges && line.trim().is_empty() {
            in_ranges = false;
        } else if in_ranges {
            let mut line_split = line.trim().split("-");

            let start: i64 = line_split
                .next()
                .unwrap()
                .parse()
                .expect("Could not parse start");
            let end: i64 = line_split
                .next()
                .unwrap()
                .parse()
                .expect("Could not parse end");

            ranges.push((start, end));
        } else {
            let num: i64 = line.trim().parse().expect("Could not parse num");

            for (start, end) in &ranges {
                if start <= &num && &num <= end {
                    fresh_count += 1;
                    break;
                }
            }
        }
    }

    fresh_count
}

pub fn solve_day_5_part_2() {
    let text = read_to_string("./data/input/day_5.txt").expect("Could not parse text file");
    let result = part_2(text);
    println!("Day 5 Part 2: {}", result);
}

fn part_2(text: String) -> i64 {
    let mut ranges = Vec::new();
    let mut fresh_count = 0;

    for line in text.trim().lines() {
        if line.trim().is_empty() {
            break;
        }

        let mut line_split = line.trim().split("-");

        let start: i64 = line_split
            .next()
            .unwrap()
            .parse()
            .expect("Could not parse start");
        let end: i64 = line_split
            .next()
            .unwrap()
            .parse()
            .expect("Could not parse end");

        ranges.push((start, end));
    }

    ranges.sort();

    let mut new_ranges = Vec::new();

    for (start, end) in ranges {
        if new_ranges.len() == 0 {
            new_ranges.push(vec![start, end]);
            continue;
        }

        let new_range_count = new_ranges.len();

        if start <= new_ranges[new_range_count - 1][1] {
            new_ranges[new_range_count - 1][1] = max(new_ranges[new_range_count - 1][1], end);
        } else {
            new_ranges.push(vec![start, end]);
        }
    }

    for range in new_ranges {
        let start = range[0];
        let end = range[1];
        fresh_count += end as i64 - start as i64 + 1;
    }

    fresh_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT.to_string()), 14);
    }
}
