use std::{fs::read_to_string, i32};

pub fn solve_day_4_part_1() {
    let text = read_to_string("./data/input/day_4.txt").expect("Could not parse text file");
    let result = part_1(text);
    println!("Day 4 Part 1: {}", result);
}

fn part_1(text: String) -> i32 {
    let deltas = vec![
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let mut input_vec = Vec::new();
    let mut accessible_roll_count = 0;

    for line in text.lines() {
        let line_vec: Vec<char> = line.trim().chars().collect();
        input_vec.push(line_vec);
    }

    let m = input_vec.len();
    let n = input_vec[0].len();

    let is_valid_coord = |i: i32, j: i32| (0 <= i && i < (m as i32)) && (0 <= j && j < (n as i32));

    for (i, line) in input_vec.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c != &'@' {
                continue;
            }

            let mut curr_roll_count = 0;

            for (di, dj) in &deltas {
                let new_i = (i as i32) + di;
                let new_j = (j as i32) + dj;

                if is_valid_coord(new_i, new_j) && input_vec[new_i as usize][new_j as usize] == '@'
                {
                    curr_roll_count += 1;
                };
            }

            if curr_roll_count < 4 {
                accessible_roll_count += 1;
            }
        }
    }

    accessible_roll_count
}

pub fn solve_day_4_part_2() {
    let text = read_to_string("./data/input/day_4.txt").expect("Could not parse text file");
    let result = part_2(text);
    println!("Day 4 Part 2: {}", result);
}

fn part_2(text: String) -> i32 {
    let deltas = vec![
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let mut input_vec = Vec::new();
    let mut accessible_roll_count = 0;

    for line in text.lines() {
        let line_vec: Vec<char> = line.trim().chars().collect();
        input_vec.push(line_vec);
    }

    let m = input_vec.len();
    let n = input_vec[0].len();

    let is_valid_coord = |i: i32, j: i32| (0 <= i && i < (m as i32)) && (0 <= j && j < (n as i32));

    let mut total_roll_count = 0;
    let mut prev_roll_count = i32::MAX;

    for line in &input_vec {
        for c in line {
            if c == &'@' {
                total_roll_count += 1;
            }
        }
    }

    while prev_roll_count > total_roll_count {
        prev_roll_count = total_roll_count;
        let mut positions_to_remove = Vec::new();

        for (i, line) in input_vec.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if c != &'@' {
                    continue;
                }

                let mut neighbor_roll_count = 0;

                for (di, dj) in &deltas {
                    let new_i = (i as i32) + di;
                    let new_j = (j as i32) + dj;

                    if is_valid_coord(new_i, new_j)
                        && input_vec[new_i as usize][new_j as usize] == '@'
                    {
                        neighbor_roll_count += 1;
                    };
                }

                if neighbor_roll_count < 4 {
                    positions_to_remove.push((i, j));
                }
            }
        }

        for (i, j) in positions_to_remove {
            input_vec[i][j] = '.';
            accessible_roll_count += 1;
            total_roll_count -= 1;
        }
    }

    accessible_roll_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT.to_string()), 43);
    }
}
