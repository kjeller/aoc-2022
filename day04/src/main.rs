use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;

fn part_1() -> i32 {
    
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            if let Ok(line) = l {
                let (a, b) = line.split_once(",").unwrap();
                let (a_start, a_end) = a.split_once("-").unwrap();
                let (b_start, b_end) = b.split_once("-").unwrap();

                (
                    Range {
                        start: a_start.parse::<i32>().unwrap(),
                        end: a_end.parse::<i32>().unwrap(),
                    },
                    Range {
                        start: b_start.parse::<i32>().unwrap(),
                        end: b_end.parse::<i32>().unwrap(),
                    },
                )
            } else {
                (
                    Range {
                        start: 0,
                        end: 0,
                    },
                    Range {
                        start: 0,
                        end: 0,
                    }
                )
                
            }
        })
        .map(|(a, b)| {
            if (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end) {
                1
            } else {
                0
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn part_2() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            if let Ok(line) = l {
                let (a, b) = line.split_once(",").unwrap();
                let (a_start, a_end) = a.split_once("-").unwrap();
                let (b_start, b_end) = b.split_once("-").unwrap();

                (
                    Range {
                        start: a_start.parse::<i32>().unwrap(),
                        end: a_end.parse::<i32>().unwrap(),
                    },
                    Range {
                        start: b_start.parse::<i32>().unwrap(),
                        end: b_end.parse::<i32>().unwrap(),
                    },
                )
            } else {
                (
                    Range {
                        start: 0,
                        end: 0,
                    },
                    Range {
                        start: 0,
                        end: 0,
                    }
                )
                
            }
        })
        .map(|(a, b)| {
            if (a.start <= b.start && a.end >= b.start) || (b.start <= a.start && b.end >= a.start) {
                1
            } else {
                0
            }
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part1" {
        println!("{}", part_1());
    } else if part == "part2" {
        println!("{}", part_2());
    }
}
