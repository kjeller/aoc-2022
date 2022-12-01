use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_lines(path: &str) -> Vec<i32> {
    let file = File::open(path).expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line|
            if let Ok(l) = line {
                if let Ok(val) = l.parse::<i32>() {
                    val
                } else {
                    -1
                }
            } else {
                -1
            }
        )
        .collect()
}