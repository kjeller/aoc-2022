extern crate pathfinding;
use pathfinding::prelude::dijkstra;

use std::{env, hash::Hash};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(i32, i32);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Successor {
    pub pos: Pos,
    pub cost: i32,
}

impl PartialEq<(Pos, i32)> for Successor {
    fn eq(&self, other: &(Pos, i32)) -> bool {
        self.pos == other.0 && self.cost == other.1
    }
}

struct Map {
    width: u8,
    height: u8,
    data: Vec<Vec<Option<(u8, u8)>>>,
    start: Vec<Pos>,
    goal: Pos,
}

impl Map {
    fn new(map_row: Vec<&str>, start_char: Vec<char>) -> Map {
        let width = map_row[0].len() as u8;
        let height = map_row.len() as u8;
        let mut data = Vec::new();
        let mut start: Vec<Pos> = Vec::new();
        let mut goal = Pos(0, 0);

        for (i, map_line) in map_row.iter().enumerate() {
            let mut row: Vec<Option<(u8, u8)>> = Vec::new();
            for (j, c) in map_line.chars().enumerate() {
                if start_char.contains(&c) {
                    start.push(Pos(j as i32, i as i32));
                    row.push(Some((1, 'a' as u8)))
                } else {
                    match c {
                        'E' => {
                            goal = Pos(j as i32, i as i32);
                            row.push(Some((1, 'z' as u8)))
                        }
                        _ => row.push(Some((1, c as u8))),
                    }
                }
            }
            data.push(row);
        }
        Map {
            width,
            height,
            data,
            start,
            goal,
        }
    }

    fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                // Remove diagonal and moving to same position
                if (dx + dy).abs() != 1 {
                    continue;
                }
                let new_pos = Pos(position.0 + dx, position.1 + dy);
                if new_pos.0 < 0
                    || new_pos.0 >= self.width.into()
                    || new_pos.1 < 0
                    || new_pos.1 >= self.height.into()
                {
                    continue;
                }

                // Diff in elevation
                let mut delev = 0;

                let val = self.data[position.1 as usize][position.0 as usize];
                if let Some((_, elev)) = val {
                    delev = elev;
                }

                let val = self.data[new_pos.1 as usize][new_pos.0 as usize];
                if let Some((cost, elev)) = val {
                    // Remove when elevation diff is too great
                    if delev < elev && delev.abs_diff(elev) > 1 {
                        continue;
                    }

                    successors.push(Successor {
                        pos: new_pos,
                        cost: cost as i32,
                    });
                }
            }
        }

        successors
    }
}

fn part_1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(
        input.lines().into_iter().map(|line| line).collect(),
        vec!['S'],
    );
    let start = map.start[0];
    let goal = map.goal;

    let result = dijkstra(
        &start,
        |p| {
            map.get_successors(p)
                .iter()
                .map(|s| (s.pos, s.cost))
                .collect::<Vec<_>>()
        },
        |p| *p == goal,
    );
    let result = result.expect("No path found");
    result.1
}

fn part_2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(
        input.lines().into_iter().map(|line| line).collect(),
        vec!['S', 'a'],
    );
    let goal = map.goal;
    let mut res: Vec<i32> = Vec::new();

    for i in 0..map.start.len() {
        let start = map.start[i];
        let result = dijkstra(
            &start,
            |p| {
                map.get_successors(p)
                    .iter()
                    .map(|s| (s.pos, s.cost))
                    .collect::<Vec<_>>()
            },
            |p| *p == goal,
        );

        if let Some(result) = result {
            res.push(result.1);
        }
    }

    res.sort_unstable();
    res[0]
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
