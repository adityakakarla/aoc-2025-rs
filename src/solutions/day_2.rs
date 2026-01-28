use std::fs;

pub fn solve_day_2_part_1() {
    let text = fs::read_to_string("./data/input/day_2.txt").expect("Could not parse text file");
    let ranges = text.trim().split(",");
    let mut invalid_id_sum = 0;

    for range in ranges {
        let mut split_range = range.split("-");
        let start: i64 = split_range.next().unwrap().parse().unwrap();
        let end: i64 = split_range.next().unwrap().parse().unwrap();

        for num in start..=end {
            let num_str = format!("{}", num);
            if num_str.len() % 2 != 0 {
                continue;
            }

            let middle_index = num_str.len() / 2;

            let first_half = &num_str[0..middle_index];
            let second_half = &num_str[middle_index..num_str.len()];

            if first_half == second_half {
                invalid_id_sum += num;
            }
        }
    }

    println!("{}", invalid_id_sum);
}
