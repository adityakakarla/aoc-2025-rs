use std::fs;

pub fn solve_day_1() {
    let text = fs::read_to_string("./data/input/day_1.txt").expect("Could not parse text file");
    let lines = text.lines();

    let mut current_value = 50;
    let mut zero_count = 0;

    for line in lines {
        let direction = line.chars().nth(0).expect("Could not get direction char");
        let steps_str = &line[1..line.len()];
        let steps_num: i32 = steps_str.parse().expect("Could not get number of steps");

        if direction == 'L' {
            current_value -= steps_num;
        } else if direction == 'R' {
            current_value += steps_num;
        }

        current_value %= 100;

        if current_value == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}
