use std::{collections::HashSet, fs::read_to_string};

pub fn solve_day_7_part_1() {
    let text = read_to_string("./data/input/day_7.txt").expect("Could not parse text file");
    let result = part_1(text);
    println!("Day 7 Part 1: {}", result);
}

fn part_1(text: String) -> i64 {
    let mut lines = text.trim().lines();

    let manifold_line = lines.next().unwrap();
    let mut manifold_idx = 0;
    let mut line_length = 0;

    for (idx, c) in manifold_line.chars().enumerate() {
        line_length += 1;
        if c == 'S' {
            manifold_idx = idx;
        }
    }

    let mut beams: HashSet<usize> = HashSet::new();

    beams.insert(manifold_idx);

    let mut split_count = 0;

    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if beams.contains(&idx) && c == '^' {
                beams.remove(&idx);
                if idx + 1 < line_length {
                    beams.insert(idx + 1);
                }

                if idx > 0 {
                    beams.insert(idx - 1);
                }

                split_count += 1;
            }
        }
    }

    split_count
}

pub fn solve_day_7_part_2() {
    let text = read_to_string("./data/input/day_7.txt").expect("Could not parse text file");
    let result = part_2(text);
    println!("Day 7 Part 2: {}", result);
}

fn part_2(text: String) -> i64 {
    let mut lines = text.trim().lines();

    let manifold_line = lines.next().unwrap();
    let mut manifold_idx = 0;
    let mut line_length = 0;

    for (idx, c) in manifold_line.chars().enumerate() {
        line_length += 1;
        if c == 'S' {
            manifold_idx = idx;
        }
    }

    let mut beams = vec![0; line_length];
    beams[manifold_idx] = 1;

    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            if beams[idx] > 0 && c == '^' {
                if idx + 1 < line_length {
                    beams[idx + 1] += beams[idx];
                }

                if idx > 0 {
                    beams[idx - 1] += beams[idx];
                }

                beams[idx] = 0;
            }
        }
    }

    let total_sum: usize = beams.iter().sum();
    total_sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT.to_string()), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT.to_string()), 40);
    }
}
