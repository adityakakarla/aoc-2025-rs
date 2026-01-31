use std::fs::read_to_string;

pub fn solve_day_6_part_1() {
    let text = read_to_string("./data/input/day_6.txt").expect("Could not parse text file");
    let result = part_1(text);
    println!("Day 6 Part 1: {}", result);
}

fn part_1(text: String) -> i64 {
    let mut values = Vec::new();
    let mut operations = Vec::new();

    for line in text.trim().lines() {
        let line_values: Vec<&str> = line.split_whitespace().collect();
        if line_values[0] == "*" || line_values[0] == "+" {
            operations = line_values;
        } else {
            values.push(line_values);
        }
    }

    let mut total_sum = 0;

    for (i, operation) in operations.iter().enumerate() {
        let mut curr_sum;

        if operation == &"+" {
            curr_sum = 0;
        } else {
            curr_sum = 1;
        }

        for row in &values {
            let value: i64 = row[i].parse().unwrap();
            if operation == &"+" {
                curr_sum += value;
            } else if operation == &"*" {
                curr_sum *= value;
            }
        }

        total_sum += curr_sum;
    }

    total_sum
}

pub fn solve_day_6_part_2() {}

fn part_2(text: String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +
    ";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 4277556);
    }

    #[test]
    fn test_part_2() {}
}
