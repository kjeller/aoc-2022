mod util;
use std::env;

fn part_1(parsed_input: &Vec<i32>) -> i32 {
    return parsed_input.iter().sum();
}

fn part_2(parsed_input: &Vec<i32>) -> i32 {
    return parsed_input.iter().product();
}

fn main() {
    let parsed_input = util::read_lines("input.txt");
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part1" {
        println!("{}", part_1(&parsed_input));
    } else if part == "part2"{
        println!("{}", part_2(&parsed_input));
    }
}