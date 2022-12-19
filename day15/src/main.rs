use regex::Regex;
use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point (i32, i32);

#[derive(Debug, Clone)]
struct Sensor {
    pos: Point,
    beacon: Point,
    signals: Vec<Point>,
    m_distance: i32,
}

impl Sensor {
    fn new() -> Sensor {
        Sensor { pos: Point (0, 0), beacon: Point(0, 0), signals: vec![], m_distance: 0 }
    }
}

fn parse_input(str: &str, y: i32) -> usize{
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    let mut range_combined: Vec<i32> = vec![];
    let mut beacon_row_set: HashSet<Point> = HashSet::new();

    for line in str.lines().into_iter() {
        let mut sensor = Sensor::new();
        let mut cap_iter = re.captures_iter(line);
        let mut cap = cap_iter.next().unwrap();
        let a = &cap[1].parse::<i32>().unwrap();
        let b = &cap[2].parse::<i32>().unwrap();
        sensor.pos = Point(a.to_owned(), b.to_owned());
        
        cap = cap_iter.next().unwrap();
        let c = &cap[1].parse::<i32>().unwrap();
        let d = &cap[2].parse::<i32>().unwrap();
        sensor.beacon = Point(c.to_owned(), d.to_owned());

        let m: i32 = (a.abs_diff(c.to_owned()) + b.abs_diff(d.to_owned())) as i32;
        sensor.m_distance = m as i32;

        let y_range  = b-m..=b+m;

        if y_range.contains(&y){
            let y_offset = y.abs_diff(*b) as i32;
            let x_offset = a - m + y_offset;
            let dist = 2 * (m - y_offset);
            let range = x_offset..=x_offset + dist;

            if d.eq(&y) && range.contains(&c) {
                beacon_row_set.insert(sensor.beacon);
            }
            
            range_combined.extend(range);
        }
    }
    range_combined.sort_unstable();
    range_combined.dedup();
    range_combined.iter().count()- beacon_row_set.iter().count()
}

fn part_1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    parse_input(&input, 2000000)
}

fn part_2() -> usize {
    0
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
