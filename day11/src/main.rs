use std::{collections::LinkedList, env};

#[derive(Debug, PartialEq)]
enum Arg {
    Old,
    Number(u64),
}

impl Arg {
    fn from_string(str: &str) -> Arg {
        let val = str.parse::<u64>();

        match val {
            Ok(x) => Arg::Number(x),
            _ => Arg::Old,
        }
    }
}

// Operation: new = old * old
// new = old .. part is always the same
// so we only have to pick operator and last arg
#[derive(Debug, PartialEq)]
enum Operation {
    Add(Arg),
    Mul(Arg),
}

impl Operation {
    fn from_string(str: &str) -> Operation {
        let (op, val) = str.trim().split_once(" ").unwrap();

        match op {
            "*" => Operation::Mul(Arg::from_string(val)),
            _ => Operation::Add(Arg::from_string(val)),
        }
    }

    fn exec(&self, arg: u64) -> u64 {
        match self {
            Operation::Add(x) => match x {
                Arg::Number(x) => arg + x,
                Arg::Old => arg + arg,
            },
            Operation::Mul(x) => match x {
                Arg::Number(x) => arg * x,
                Arg::Old => arg * arg,
            },
        }
    }
}

struct Monkey {
    items: LinkedList<u64>,
    operation: Operation,
    divisible: u64,
    true_index: i32,
    false_index: i32,
    inspection_amount: u64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: LinkedList::new(),
            operation: Operation::Add(Arg::Old),
            divisible: 0,
            true_index: 0,
            false_index: 0,
            inspection_amount: 0,
        }
    }

    fn from_string(str: &str) -> Monkey {
        let mut monkey = Monkey::new();

        str.lines().into_iter().for_each(|line| {
            // Skip "Monkey x" line
            if line.len() > 15 {
                match &line.trim_start()[0..15] {
                    "Starting items:" => {
                        let mut items: LinkedList<u64> = LinkedList::new();
                        let _ = &line.trim()[15..]
                            .split(", ")
                            .into_iter()
                            .for_each(|splits| {
                                items.push_back(splits.trim().parse::<u64>().unwrap());
                            });

                        monkey.items = items;
                    }
                    "Operation: new " => {
                        monkey.operation = Operation::from_string(&line.trim()[20..])
                    }
                    "Test: divisible" => {
                        monkey.divisible = line.trim()[18..].trim().parse::<u64>().unwrap()
                    }
                    "If true: throw " => {
                        monkey.true_index = line.trim()[25..].trim().parse::<i32>().unwrap()
                    }
                    "If false: throw" => {
                        monkey.false_index = line.trim()[25..].trim().parse::<i32>().unwrap()
                    }
                    _ => (),
                }
            }
        });

        monkey
    }
}

fn parse_monkeys_from_str(str: &str) -> Vec<Monkey> {
    str.split("\n\n")
        .into_iter()
        .map(|str| Monkey::from_string(str))
        .collect()
}
fn start_monkey_inspection<T>(monkeys: &mut Vec<Monkey>, rounds: i32, worry_mod: T) -> u64
where
    T: Fn(u64) -> u64,
{
    let monkey_amount = monkeys.iter().count();
    for _ in 0..rounds {
        for n in 0..monkey_amount {
            let mut monkey = &mut monkeys[n];
            let item_count = monkey.items.iter().count();

            for _ in 0..item_count {
                monkey = &mut monkeys[n];
                monkey.inspection_amount += 1;
                let item = monkey.items.pop_front().unwrap();

                let mut worry_level = monkey.operation.exec(item);
                worry_level = worry_mod(worry_level);

                let monkey_index: usize;
                if worry_level % monkey.divisible == 0 {
                    monkey_index = monkey.true_index as usize;
                } else {
                    monkey_index = monkey.false_index as usize;
                }
                monkey = &mut monkeys[monkey_index];
                monkey.items.push_back(worry_level);
            }
        }
    }

    let mut res: Vec<u64> = monkeys.iter().map(|e| e.inspection_amount).collect();
    res.sort_unstable();
    res.reverse();
    res[0] * res[1]
}

fn part_1() -> u64 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut monkeys = parse_monkeys_from_str(&input);
    start_monkey_inspection(monkeys.as_mut(), 20, |modify: u64| modify / 3)
}

fn part_2() -> u64 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut monkeys = parse_monkeys_from_str(&input);
    let product = monkeys.iter().map(|m| m.divisible).product::<u64>();
    start_monkey_inspection(monkeys.as_mut(), 10000, |modify: u64| modify % product)
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

mod tests {
    use crate::*;

    #[test]
    fn test_create_monkey_from_string() {
        let str = "Monkey 0:
Starting items: 50, 70, 54, 83, 52, 78
Operation: new = old * 3
Test: divisible by 11
    If true: throw to monkey 2
    If false: throw to monkey 7

"
        .to_string();

        let monkey = Monkey::from_string(&str);

        assert_eq!(monkey.items, LinkedList::from([50, 70, 54, 83, 52, 78]));
        assert_eq!(monkey.operation, Operation::Mul(Arg::Number(3)));
        assert_eq!(monkey.divisible, 11);
        assert_eq!(monkey.true_index, 2);
        assert_eq!(monkey.false_index, 7);
    }
}
