use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;


fn part_1() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|l| {
            // Split line into two, look for duplicate over two compartments
            if let Ok(line) = l {

                let mut rucksack = HashSet::new();
                let compartment = line.split_at(line.len()/2);

                // Insert everything in compartment 0
                for b in compartment.0.bytes() {
                    rucksack.insert(b);
                }

                let mut dup: u8 = 0;
                
                for b in compartment.1.bytes() {
                    if rucksack.contains(&b) {
                        if b >= 96 && b <= 122 {
                            dup = b - 96;
                        } else {
                            dup = b - 38;
                        }
                        
                    }
                }
                i32::from(dup)
            } else {
                0
            }
        })
        .collect::<Vec<i32>>().iter().sum()
}

fn part_2() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    lines
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            
            let mut rucksack_a = HashSet::new();
            let mut rucksack_b = HashSet::new();

            for b in chunk.get(0).unwrap().bytes() {
                rucksack_a.insert(b);
            }

            for b in chunk.get(1).unwrap().bytes() {
                rucksack_b.insert(b);
            }

            let mut dup: u8 = 0;
            
            for b in chunk.get(2).unwrap().bytes() {
                if rucksack_a.contains(&b) && rucksack_b.contains(&b) {
                    if b >= 96 && b <= 122 {
                        dup = b - 96;
                    } else {
                        dup = b - 38;
                    }
                    
                }
            }
            i32::from(dup)

        })
        .collect::<Vec<i32>>().iter().sum()
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part1" {
        println!("{}", part_1());
    } else if part == "part2"{
        println!("{}", part_2());
    }
}