use std::env;
use std::collections::HashSet;

fn part_1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_iterator = input.chars();
    let found_at_index: Vec<i32> = input_iterator
        .enumerate()
        .map(|(i, _)| {
            if i < input.len()-4 {
                let mut set: HashSet<u8> = HashSet::new();
                
                if set.insert(*input.as_bytes().get(i).unwrap()) &&
                    set.insert(*input.as_bytes().get(i+1).unwrap()) &&
                    set.insert(*input.as_bytes().get(i+2).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+3).unwrap()) {
                    i as i32
                } else {
                    -1
                }
            }
             else {
                -1
            }
        })
        .collect();

    for i in found_at_index {
        if i!=-1 {
            return i + 4;
        }
    }
    0
}

fn part_2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_iterator = input.chars();
    let found_at_index: Vec<i32> = input_iterator
        .enumerate()
        .map(|(i, _)| {
            if i < input.len()-14 {
                let mut set: HashSet<u8> = HashSet::new();
                
                // oogabooga
                if set.insert(*input.as_bytes().get(i).unwrap()) &&
                    set.insert(*input.as_bytes().get(i+1).unwrap()) &&
                    set.insert(*input.as_bytes().get(i+2).unwrap()) &&
                    set.insert(*input.as_bytes().get(i+3).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+4).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+5).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+6).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+7).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+8).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+9).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+10).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+11).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+12).unwrap()) && 
                    set.insert(*input.as_bytes().get(i+13).unwrap()) {
                    i as i32
                } else {
                    -1
                }
            }
             else {
                -1
            }
        })
        .collect();

    for i in found_at_index {
        if i!=-1 {
            return i + 14;
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instr_test() {
        // let datastream = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        // assert_eq!(detect_marker(datastream), 7);

        let datastream = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(detect_marker(datastream), 5);

        let datastream = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(detect_marker(datastream), 6);

        let datastream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(detect_marker(datastream), 10);

        let datastream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(detect_marker(datastream), 11);
    }

}
