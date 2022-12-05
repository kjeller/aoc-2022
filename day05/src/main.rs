use std::env;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Crate {
    tag: char,
}

impl Crate {
    pub fn parse_crate_line(line: &str) -> Vec<Crate> {
        line.chars()
            .skip(1) // skip first '['
            .step_by(4) // step to every tag
            .map(|c| Crate { tag: c })
            .collect::<Vec<Crate>>()
    }

    fn to_string(&self) -> String {
        format!("Crate[tag: {}]", self.tag)
    }

    fn eq(&self, other: &Crate) -> bool {
        self.tag == other.tag
    }
}

#[derive(PartialEq, Debug)]
struct Instruction {
    move_n: i32,
    from_stack: usize,
    to_stack: usize,
}

impl Instruction {
    // move x from y to z
    // returns vec![x, y, z]
    pub fn parse_instr(instr: &str) -> Instruction {
        let instr_vec = instr
            .split(" ")
            .into_iter()
            .filter(|str| *str != "move")
            .filter(|str| *str != "from")
            .filter(|str| *str != "to")
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Instruction {
            move_n: instr_vec[0],
            from_stack: instr_vec[1] as usize,
            to_stack: instr_vec[2] as usize,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "Instruction[move_n: {}, from_stack: {}, to_stack: {}]",
            self.move_n, self.from_stack, self.to_stack
        )
    }

    fn eq(&self, other: &Instruction) -> bool {
        self.move_n == other.move_n
            && self.from_stack == other.from_stack
            && self.to_stack == other.to_stack
    }
}

fn part_1() -> String {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let mut crate_stacks: Vec<Vec<Crate>> = crates
        .lines()
        .rev()
        .next()
        .unwrap()
        .split(char::is_numeric)
        .map(|_| Vec::new())
        .collect();

    crates.lines().rev().into_iter().for_each(|crts| {
        for (i, c) in Crate::parse_crate_line(crts).iter().enumerate() {
            if c.tag.is_alphabetic() {
                crate_stacks[i].push(*c);
            }
        }
    });

    instructions.lines().into_iter().for_each(|line| {
        let instr = Instruction::parse_instr(line);

        for _ in 0..instr.move_n {
            let temp = crate_stacks[instr.from_stack - 1].pop().unwrap();
            crate_stacks[instr.to_stack - 1].push(temp);
        }
    });

    String::from_iter(
        crate_stacks
            .iter_mut()
            .map(|s| {
                if let Some(last) = s.last() {
                    last.tag
                } else {
                    '\0'
                }
            })
            .collect::<Vec<char>>(),
    )
}

fn part_2() -> String {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (crates, instructions) = input.split_once("\n\n").unwrap();

    let mut crate_stacks: Vec<Vec<Crate>> = crates
        .lines()
        .rev()
        .next()
        .unwrap()
        .split(char::is_numeric)
        .map(|_| Vec::new())
        .collect();

    crates.lines().rev().into_iter().for_each(|crts| {
        for (i, c) in Crate::parse_crate_line(crts).iter().enumerate() {
            if c.tag.is_alphabetic() {
                crate_stacks[i].push(*c);
            }
        }
    });

    instructions.lines().into_iter().for_each(|line| {
        let instr = Instruction::parse_instr(line);
        let mut buf: Vec<Crate> = Vec::new();

        for _ in 0..instr.move_n {
            buf.push(crate_stacks[instr.from_stack - 1].pop().unwrap());
        }

        for _ in 0..instr.move_n {
            crate_stacks[instr.to_stack - 1].push(buf.pop().unwrap());
        }
    });

    String::from_iter(
        crate_stacks
            .iter_mut()
            .map(|s| {
                if let Some(last) = s.last() {
                    last.tag
                } else {
                    '\0'
                }
            })
            .collect::<Vec<char>>(),
    )
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
        let instr = "move 1 from 2 to 3";
        assert_eq!(
            Instruction::parse_instr(instr),
            Instruction {
                move_n: 1,
                from_stack: 2,
                to_stack: 3
            }
        );
    }

    #[test]
    fn parse_crates_test() {
        let line = "[A] [B] [C]";
        assert_eq!(
            Crate::parse_crate_line(line),
            vec![Crate { tag: 'A' }, Crate { tag: 'B' }, Crate { tag: 'C' }]
        )
    }
}
