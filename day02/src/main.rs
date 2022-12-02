use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq)]
enum RockPaperScissorOutcomeEncoded {
    Z,
    X,
    Y,
}

impl RockPaperScissorOutcomeEncoded {
    // RPC is obviously Rock Paper Scissor
    fn decode_rpc(&self) -> RockPaperScissorOutcome {
        match self {
            RockPaperScissorOutcomeEncoded::X => RockPaperScissorOutcome::Lose,
            RockPaperScissorOutcomeEncoded::Y => RockPaperScissorOutcome::Draw,
            RockPaperScissorOutcomeEncoded::Z => RockPaperScissorOutcome::Win,
        }
    }

    fn from_str(input: &str) -> Result<RockPaperScissorOutcomeEncoded, FailedToParse> {
        match input {
            "X"  => Ok(RockPaperScissorOutcomeEncoded::X),
            "Y"  => Ok(RockPaperScissorOutcomeEncoded::Y),
            "Z"  => Ok(RockPaperScissorOutcomeEncoded::Z),
            _      => Err(FailedToParse),
        }
    }
}

enum RockPaperScissorOutcome {
    Win,
    Lose,
    Draw,
}

impl RockPaperScissorOutcome {
    fn outcome_value(&self) -> i32{
        match self {
            RockPaperScissorOutcome::Win => 6,
            RockPaperScissorOutcome::Lose => 0,
            RockPaperScissorOutcome::Draw => 3,
        }
    }
}

impl RockPaperScissor {
    // From self's perspective
    fn outcome(&self, other: &RockPaperScissor) -> RockPaperScissorOutcome{
        match self {
            RockPaperScissor::Rock => {
                match other {
                    RockPaperScissor::Rock => RockPaperScissorOutcome::Draw,
                    RockPaperScissor::Paper => RockPaperScissorOutcome::Lose,
                    RockPaperScissor::Scissor => RockPaperScissorOutcome::Win,
                }
            },
            RockPaperScissor::Paper => {
                match other {
                    RockPaperScissor::Rock => RockPaperScissorOutcome::Win,
                    RockPaperScissor::Paper => RockPaperScissorOutcome::Draw,
                    RockPaperScissor::Scissor => RockPaperScissorOutcome::Lose,
                }
            }
            RockPaperScissor::Scissor => {
                match other {
                    RockPaperScissor::Rock => RockPaperScissorOutcome::Lose,
                    RockPaperScissor::Paper => RockPaperScissorOutcome::Win,
                    RockPaperScissor::Scissor => RockPaperScissorOutcome::Draw,
                }
            }
        }
    }

    fn rpc_value(&self) -> i32{
        match self {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
        }
    }

    // From self's perspective
    fn force_outcome(&self, other: &RockPaperScissorOutcome) -> RockPaperScissor{
        match self {
            RockPaperScissor::Rock => {
                match other {
                    RockPaperScissorOutcome::Win => RockPaperScissor::Paper,
                    RockPaperScissorOutcome::Lose => RockPaperScissor::Scissor,
                    RockPaperScissorOutcome::Draw => RockPaperScissor::Rock,
                }
            },
            RockPaperScissor::Paper => {
                match other {
                    RockPaperScissorOutcome::Win => RockPaperScissor::Scissor,
                    RockPaperScissorOutcome::Lose => RockPaperScissor::Rock,
                    RockPaperScissorOutcome::Draw => RockPaperScissor::Paper,
                }
            }
            RockPaperScissor::Scissor => {
                match other {
                    RockPaperScissorOutcome::Win => RockPaperScissor::Rock,
                    RockPaperScissorOutcome::Lose => RockPaperScissor::Paper,
                    RockPaperScissorOutcome::Draw => RockPaperScissor::Scissor,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum RockPaperScissorEncoded {
    Y,
    X,
    Z,
    A,
    B,
    C,
}

struct FailedToParse;

impl RockPaperScissorEncoded {
    fn from_str(input: &str) -> Result<RockPaperScissorEncoded, FailedToParse> {
        match input {
            "A"  => Ok(RockPaperScissorEncoded::A),
            "B"  => Ok(RockPaperScissorEncoded::B),
            "C"  => Ok(RockPaperScissorEncoded::C),
            "X" => Ok(RockPaperScissorEncoded::X),
            "Y" => Ok(RockPaperScissorEncoded::Y),
            "Z" => Ok(RockPaperScissorEncoded::Z),
            _      => Err(FailedToParse),
        }
    }

    // RPC is obviously Rock Paper Scissor
    fn decode_rpc(&self) -> RockPaperScissor {
        match self {
            RockPaperScissorEncoded::X => RockPaperScissor::Rock,
            RockPaperScissorEncoded::Y => RockPaperScissor::Paper,
            RockPaperScissorEncoded::Z => RockPaperScissor::Scissor,
            RockPaperScissorEncoded::A => RockPaperScissor::Rock,
            RockPaperScissorEncoded::B => RockPaperScissor::Paper,
            RockPaperScissorEncoded::C => RockPaperScissor::Scissor,
        }
    }
}

fn part_1() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|l| {

            if let Ok(line) = l {
                let mut split = line.split(" ");
                
                let elf = RockPaperScissorEncoded::from_str(split.next().unwrap());
                let human = RockPaperScissorEncoded::from_str(split.next().unwrap());

                let mut elf_input: RockPaperScissor = RockPaperScissor::Paper;
                let mut human_input: RockPaperScissor = RockPaperScissor::Paper;

                if let Ok(elf_) = elf {
                    elf_input = RockPaperScissorEncoded::decode_rpc(&elf_);
                }

                if let Ok(human_) = human {
                    human_input = RockPaperScissorEncoded::decode_rpc(&human_);
                }

                RockPaperScissor::outcome(&human_input, &elf_input).outcome_value() + human_input.rpc_value()

            } else {
                0
            }
        })
        .collect::<Vec<i32>>().iter().sum()
}

fn part_2() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|l| {

            if let Ok(line) = l {
                let mut split = line.split(" ");
                //println!("test{}", split.next().unwrap());
                
                let elf = RockPaperScissorEncoded::from_str(split.next().unwrap());
                let human = RockPaperScissorOutcomeEncoded::from_str(split.next().unwrap());

                let mut elf_input: RockPaperScissor = RockPaperScissor::Paper;
                let mut human_input: RockPaperScissor = RockPaperScissor::Paper;

                if let Ok(elf_) = elf {
                    elf_input = RockPaperScissorEncoded::decode_rpc(&elf_);
                }

                if let Ok(human_) = human {

                    let outcome = human_.decode_rpc();
                    human_input = RockPaperScissor::force_outcome(&elf_input, &outcome);
                }

                RockPaperScissor::outcome(&human_input, &elf_input).outcome_value() + human_input.rpc_value()

            } else {
                0
            }
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