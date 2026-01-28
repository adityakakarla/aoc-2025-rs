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

pub fn solve_day_2_part_2() {
    // let text = fs::read_to_string("./data/input/day_2.txt").expect("Could not parse text file");
    let text = String::from(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124",
    );
    let ranges = text.trim().split(",");
    let mut invalid_id_sum = 0;

    for range in ranges {
        let mut split_range = range.trim().split("-");
        let start: i64 = split_range.next().unwrap().parse().unwrap();
        let end: i64 = split_range.next().unwrap().parse().unwrap();

        for num in start..=end {
            let num_str = format!("{}", num);

            for i in 1..=(num_str.len() / 2) {
                if num_str.len() % i != 0 {
                    continue;
                }

                let pattern = &num_str[0..i];
                let mut pattern_exists = true;

                for j in 1..(num_str.len() / i) {
                    if &num_str[(i * j)..(i * (j + 1))] != pattern {
                        pattern_exists = false;
                        break;
                    }
                }

                if pattern_exists {
                    invalid_id_sum += num;
                    println!("{}", num);
                    break;
                }
            }
        }
    }

    println!("{}", invalid_id_sum);
}
