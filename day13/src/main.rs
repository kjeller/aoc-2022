use serde::Deserialize;
use std::cmp::Ordering;
use std::env;

#[derive(PartialEq, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(isize),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &PacketData) -> Option<std::cmp::Ordering> {
        match self {
            PacketData::Integer(x) => match other {
                PacketData::Integer(y) => Some(x.cmp(y)),
                PacketData::List(_) => {
                    let xlist = PacketData::List(vec![PacketData::Integer(*x)]);
                    xlist.partial_cmp(other)
                }
            },
            PacketData::List(x) => match other {
                PacketData::Integer(y) => {
                    let ylist = PacketData::List(vec![PacketData::Integer(*y)]);
                    self.partial_cmp(&ylist)
                }
                PacketData::List(y) => {
                    if x.is_empty() && !y.is_empty() {
                        return Some(Ordering::Less);
                    }

                    if !x.is_empty() && y.is_empty() {
                        return Some(Ordering::Greater);
                    }

                    let mut x = x.iter();
                    let mut y = y.iter();

                    for (left, right) in x.by_ref().zip(y.by_ref()) {
                        if let Some(t) = left.partial_cmp(right) {
                            if t != Ordering::Equal {
                                return Some(t);
                            }
                        }
                    }

                    match (x.next(), y.next()) {
                        (None, Some(_)) => Some(Ordering::Less),
                        (Some(_), None) => Some(Ordering::Greater),
                        _ => Some(Ordering::Equal),
                    }
                }
            },
        }
    }
}

fn parse_packet_pair(pair: &str) -> Vec<PacketData> {
    let mut packets: Vec<PacketData> = Vec::new();
    pair.split("\n").into_iter().for_each(|line| {
        if !line.is_empty() {
            packets.push(serde_json::from_str(line).unwrap());
        }
    });
    packets
}

fn part_1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut index = vec![];

    for (i, pair) in input.split("\n\n").into_iter().enumerate() {
        let pair_vec = parse_packet_pair(pair);

        if pair_vec[0] < pair_vec[1] {
            index.push(i + 1);
        }
    }

    index.iter().sum()
}

fn part_2() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let div_packet1 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);
    let div_packet2 = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
    let mut packets: Vec<PacketData> = vec![div_packet1.clone(), div_packet2.clone()];

    for (_, pair) in input.split("\n\n").into_iter().enumerate() {
        let mut packet = parse_packet_pair(pair);
        packets.append(packet.as_mut());
    }

    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut div: Vec<usize> = vec![];

    for (i, e) in packets.iter().enumerate() {
        if e.eq(&div_packet1) || e.eq(&div_packet2) {
            div.push(i + 1);
        }
    }
    div.iter().product()
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
    fn test_create_array() {
        let str = "[1,[2,[3,[4,[5,6,7]]]],8,9]";

        let actual = parse_packet_pair(str);

        let expected = vec![PacketData::List(vec![
            PacketData::Integer(1),
            PacketData::List(vec![
                PacketData::Integer(2),
                PacketData::List(vec![
                    PacketData::Integer(3),
                    PacketData::List(vec![
                        PacketData::Integer(4),
                        PacketData::List(vec![
                            PacketData::Integer(5),
                            PacketData::Integer(6),
                            PacketData::Integer(7),
                        ]),
                    ]),
                ]),
            ]),
            PacketData::Integer(8),
            PacketData::Integer(9),
        ])];

        assert_eq!(actual, expected);
    }
}
