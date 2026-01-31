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

pub fn solve_day_6_part_2() {
    let text = read_to_string("./data/input/day_6.txt").expect("Could not parse text file");
    let result = part_2(text);
    println!("Day 6 Part 2: {}", result);
}

fn part_2(text: String) -> i64 {
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in text.trim().lines() {
        data.push(line.chars().collect());
    }

    let mut values = Vec::new();
    let mut operation = '\0';
    let mut total_sum = 0;

    for idx in 0..data[0].len() {
        let mut digits = Vec::new();

        for (line_idx, line) in data.iter().enumerate() {
            if line_idx == data.len() - 1 {
                if operation == '\0' {
                    operation = line[idx]
                }
                continue;
            }

            if line[idx] != ' ' {
                let digit = line[idx].to_digit(10).unwrap();
                digits.push(digit);
            }
        }

        if digits.len() == 0 || idx == data[0].len() - 1 {
            if digits.len() > 0 {
                let value = digits.iter().fold(0, |acc, &x| acc * 10 + x);
                values.push(value);
            }

            if operation == '+' {
                let mut curr_sum = 0;
                for value in values {
                    curr_sum += value as i64;
                }
                total_sum += curr_sum
            } else {
                let mut curr_product = 1;
                for value in values {
                    curr_product *= value as i64;
                }
                total_sum += curr_product;
            }
            values = Vec::new();
            operation = '\0';
        } else {
            let value = digits.iter().fold(0, |acc, &x| acc * 10 + x);
            values.push(value);
        }
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 4277556);
    }

    // NOTE:
    // Auto-formatting made this way more complicated.
    // Make sure text is pasted without auto-formatting, otherwise
    // trailing spaces will be removed which is a big deal.

    #[test]
    fn test_part_2() {
        let text =
            read_to_string("./data/input/day_6_test.txt").expect("Could not parse text file");
        assert_eq!(part_2(text), 3263827);
    }
}
