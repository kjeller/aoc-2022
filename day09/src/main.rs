use std::{collections::HashSet, env};

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Unvalid,
}

impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => format!("U"),
            Direction::Left => format!("L"),
            Direction::Down => format!("D"),
            Direction::Right => format!("R"),
            _ => format!("-"),
        }
    }
}

struct Move {
    dir: Direction,
    steps: i32,
}

impl Move {
    fn to_string(&self) -> String {
        format!("dir: {}, steps: {}", self.dir.to_string(), self.steps)
    }

    fn from_string(line: &str) -> Move {
        match &line[0..2] {
            "U " => Move {
                dir: Direction::Up,
                steps: line[2..].parse().unwrap(),
            },
            "L " => Move {
                dir: Direction::Left,
                steps: line[2..].parse().unwrap(),
            },
            "D " => Move {
                dir: Direction::Down,
                steps: line[2..].parse().unwrap(),
            },
            "R " => Move {
                dir: Direction::Right,
                steps: line[2..].parse().unwrap(),
            },
            _ => Move {
                dir: Direction::Unvalid,
                steps: -1,
            },
        }
    }

    fn step_pieces(vec: &mut Vec<Piece>, i: usize) {
        let tail = &vec[i + 1];
        let head = &vec[i];

        match (tail.x.abs_diff(head.x) >= 2, tail.y.abs_diff(head.y) >= 2) {
            // new movement from part 2.
            // (tails can drag other tails diagonally)
            (true, true) => {
                if tail.y > head.y && tail.x > head.x {
                    vec[i + 1].x = vec[i].x + 1;
                    vec[i + 1].y = vec[i].y + 1;
                } else if tail.y > head.y && tail.x < head.x {
                    vec[i + 1].x = vec[i].x - 1;
                    vec[i + 1].y = vec[i].y + 1;
                } else if tail.y < head.y && tail.x < head.x {
                    vec[i + 1].x = vec[i].x - 1;
                    vec[i + 1].y = vec[i].y - 1;
                } else if tail.y < head.y && tail.x > head.x {
                    vec[i + 1].x = vec[i].x + 1;
                    vec[i + 1].y = vec[i].y - 1;
                }
            }
            (true, _) => {
                if tail.x > head.x {
                    vec[i + 1].x = vec[i].x + 1;
                    vec[i + 1].y = vec[i].y;
                } else {
                    vec[i + 1].x = vec[i].x - 1;
                    vec[i + 1].y = vec[i].y;
                }
            }
            (_, true) => {
                if tail.y > head.y {
                    vec[i + 1].y = vec[i].y + 1;
                    vec[i + 1].x = vec[i].x;
                } else {
                    vec[i + 1].y = vec[i].y - 1;
                    vec[i + 1].x = vec[i].x;
                }
            }
            (_, _) => (),
        }

        let x = vec[i + 1].x;
        let y = vec[i + 1].y;
        vec[i + 1].visited.insert((x, y));
    }

    // Return how many times tail have moved
    fn step_piece(dir: &Direction, head: &mut Piece, tail: &mut Piece) {
        match dir {
            Direction::Up => {
                head.y -= 1;
                if tail.y.abs_diff(head.y) >= 2 {
                    tail.y = head.y + 1;
                    tail.x = head.x;
                }
            }
            Direction::Left => {
                head.x -= 1;
                if tail.x.abs_diff(head.x) >= 2 {
                    tail.x = head.x + 1;
                    tail.y = head.y;
                }
            }
            Direction::Down => {
                head.y += 1;
                if tail.y.abs_diff(head.y) >= 2 {
                    tail.y = head.y - 1;
                    tail.x = head.x;
                }
            }
            Direction::Right => {
                head.x += 1;
                if tail.x.abs_diff(head.x) >= 2 {
                    tail.x = head.x - 1;
                    tail.y = head.y;
                }
            }
            _ => (),
        }
        tail.visited.insert((tail.x, tail.y));
    }
}

struct Piece {
    x: i32,
    y: i32,
    visited: HashSet<(i32, i32)>, // unique places visited (x,y)
}

fn move_rope(str: &str, tail_count: usize) -> i32 {
    let start: (i32, i32) = (0, 0);
    let mut head: Piece = Piece {
        x: start.1,
        y: start.0,
        visited: HashSet::new(),
    };

    let mut tails: Vec<Piece> = Vec::new();

    for _ in 0..tail_count {
        tails.push(Piece {
            x: start.1,
            y: start.0,
            visited: HashSet::new(),
        });
    }

    str.lines().into_iter().for_each(|line| {
        let move_instr = Move::from_string(line);

        for _ in 0..move_instr.steps {
            // first move head and align first tail
            Move::step_piece(&move_instr.dir, &mut head, &mut tails[0]);

            // then align the rest of the tail(s)
            if tail_count > 1 {
                for n in 0..tail_count - 1 {
                    Move::step_pieces(&mut tails, n);
                }
            }
        }
    });

    tails[tail_count - 1].visited.iter().count() as i32
}

fn part_1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    move_rope(&input, 1)
}

fn part_2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    move_rope(&input, 9)
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
    use crate::*;

    #[test]
    fn test_example() {
        let tail_count: usize = 1;
        let str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
        .to_string();

        assert_eq!(move_rope(&str, tail_count), 13);
    }

    #[test]
    fn test_example_2() {
        let tail_count: usize = 9;
        let str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    "
        .to_string();

        assert_eq!(move_rope(&str, tail_count), 1);
    }

    #[test]
    fn test_example_3() {
        let tail_count: usize = 9;
        let str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
    "
        .to_string();

        assert_eq!(move_rope(&str, tail_count), 36);
    }
}
