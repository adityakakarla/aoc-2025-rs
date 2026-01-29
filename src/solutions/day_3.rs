use std::{cmp::max, fs::read_to_string};

pub fn solve_day_3_part_1() {
    let text = read_to_string("./data/input/day_3.txt").expect("Could not parse text file");
    let result = part_1(text);
    println!("Day 3 Part 1: {}", result);
}

fn part_1(text: String) -> u32 {
    let mut total_joltage = 0;

    let lines = text.lines();

    for line in lines {
        let line = line.trim();
        let mut first_max = 0;
        let mut second_max = 0;

        for (i, char) in line.chars().enumerate() {
            let char_digit = char.to_digit(10).unwrap();
            if i == 0 {
                first_max = char_digit;
            } else if i == (line.len() - 1) {
                second_max = max(second_max, char_digit);
            } else {
                if char_digit > first_max {
                    first_max = char_digit;
                    second_max = 0;
                } else {
                    second_max = max(second_max, char_digit);
                }
            }
        }

        total_joltage += first_max * 10 + second_max;
    }

    total_joltage
}

pub fn solve_day_3_part_2() {
    let text = read_to_string("./data/input/day_3.txt").expect("Could not parse text file");
    let result = part_2(text);
    println!("Day 3 Part 2: {}", result);
}

fn part_2(text: String) -> u64 {
    let mut total_joltage = 0;

    let lines = text.lines();

    for line in lines {
        let line = line.trim();
        let mut current_joltage = 0;
        let mut curr_idx = 0;
        let mut curr_value;

        let line_vec: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        for i in (0..12).rev() {
            curr_value = 0;

            for j in curr_idx..(line_vec.len() - i) {
                if line_vec[j] > curr_value {
                    curr_value = line_vec[j];
                    curr_idx = j + 1;
                }
            }

            current_joltage *= 10;
            current_joltage += curr_value as u64;
        }

        total_joltage += current_joltage;
    }

    total_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
    811111111111119
    234234234234278
    818181911112111";

    #[test]
    fn part_1_test() {
        let result = part_1(TEST_INPUT.to_string());
        assert_eq!(result, 357);
    }

    #[test]
    fn part_2_test() {
        let result = part_2(TEST_INPUT.to_string());
        assert_eq!(result, 3121910778619);
    }
}
