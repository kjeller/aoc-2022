mod util;
use std::{env, collections::LinkedList};

fn part_1(vec: &Vec<i32>) -> i32 {

    let mut greatest_res: i32 = 0;
    let mut res: i32 = 0;

    for v in vec {
        
        if v.is_positive() {
            res += v;
        } else {
            // new elf calorie
            if res > greatest_res {
                println!("{}", greatest_res);
                greatest_res = res;
                res = 0;
            } else {
                res = 0;
            }
        }
    }
    return greatest_res;
}

fn part_2(vec: &Vec<i32>) -> i32 {
    let mut greatest_res: i32 = 0;
    let mut res: i32 = 0;

    let mut top_stack: LinkedList<i32> = LinkedList::new();

    for v in vec {
        
        if v.is_positive() {
            res += v;
        } else {
            // new elf calorie
            top_stack.push_back(res);
            if res > greatest_res {
                
                greatest_res = res;
                res = 0;
            } else {
                res = 0;
            }
        }
    }

    // dumb way of sorting the stack
    let mut vec: Vec<_> = top_stack.into_iter().collect();
    vec.sort();
    let mut top_stack: LinkedList<_> = vec.into_iter().collect();

    return  top_stack.pop_back().unwrap() + top_stack.pop_back().unwrap() + top_stack.pop_back().unwrap()
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