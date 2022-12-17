use std::env;
use std::{thread, time::Duration};

#[derive(PartialEq, Debug, Clone)]
enum Unit {
    SandSource,
    Sand,
    Rock,
    Air,
}

impl Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::SandSource => "+".to_string(),
            Unit::Sand => "o".to_string(),
            Unit::Rock => "#".to_string(),
            Unit::Air => " ".to_string(),
        }
    }
}

struct Simulation {
    scan_map: Vec<Vec<Unit>>,
    rock_pos: Vec<(usize, usize)>,
    x_range: (usize, usize),
    y_range: (usize, usize),
    sand_source: (usize, usize),
}

impl Simulation {
    fn new() -> Simulation {
        Simulation {
            scan_map: vec![],
            rock_pos: vec![],
            x_range: (0, 0),
            y_range: (0, 0),
            sand_source: (0, 0),
        }
    }

    fn from_string(input: &str, infinite: bool) -> Simulation {
        let mut sim = Simulation::new();

        // Offset translate values
        let mut x_vec: Vec<usize> = vec![];
        let mut y_vec: Vec<usize> = vec![];

        input.split("\n").into_iter().for_each(|line| {
            let rock_cords: Vec<Option<(usize, usize)>> = line
                .trim()
                .split(" -> ")
                .into_iter()
                .map(|cord| {
                    if let Some((x, y)) = cord.split_once(",") {
                        let x: usize = x.parse().unwrap();
                        let y: usize = y.parse().unwrap();

                        x_vec.push(x);
                        y_vec.push(y);

                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect();

            for coord in rock_cords.windows(2) {
                match (coord[0], coord[1]) {
                    (Some((x1, y1)), Some((x2, y2))) => {
                        let ydiff: usize;
                        let xdiff: usize;
                        let ymax: usize;
                        let xmax: usize;

                        if y1 > y2 {
                            ydiff = y1 - y2 + 1;
                            ymax = y1;
                        } else if y1 < y2 {
                            ydiff = y2 - y1 + 1;
                            ymax = y2;
                        } else {
                            ydiff = 0;
                            ymax = y1;
                        }

                        if x1 > x2 {
                            xdiff = x1 - x2 + 1;
                            xmax = x1;
                        } else if x1 < x2 {
                            xdiff = x2 - x1 + 1;
                            xmax = x2;
                        } else {
                            xdiff = 0;
                            xmax = x1;
                        }

                        for y in 0..ydiff {
                            sim.rock_pos.push((xmax, ymax - y));
                        }

                        for x in 0..xdiff {
                            sim.rock_pos.push((xmax - x, ymax));
                        }
                    }
                    _ => (),
                }
            }
        });

        x_vec.sort_unstable();
        y_vec.sort_unstable();

        sim.x_range = (
            x_vec.first().unwrap().to_owned(),
            x_vec.last().unwrap().to_owned(),
        );
        sim.y_range = (
            y_vec.first().unwrap().to_owned(),
            y_vec.last().unwrap().to_owned(),
        );

        let scan_height = sim.y_range.1;
        let scan_width = if infinite {
            sim.x_range.1 - sim.x_range.0 + 1 + 2 * scan_height
        } else {
            sim.x_range.1 - sim.x_range.0
        };


        // Very hacky offset calculation from width..
        let x_offset = if infinite {
            scan_width / 2 - 4
        } else {
            0
        };

        // Init scan map with air
        for y in 0..=scan_height {
            sim.scan_map.push(Vec::new());

            for _ in 0..=scan_width {
                sim.scan_map[y].push(Unit::Air);
            }
        }

        // .. and rock points
        for r in sim.rock_pos.iter() {
            sim.scan_map[r.1][sim.x_range.1 - r.0 + x_offset] = Unit::Rock;
        }

        if infinite {
            for y in 1..=2 {
                sim.scan_map.push(Vec::new());

                if y == 2 {
                    for _ in 0..=scan_width {
                        sim.scan_map[sim.y_range.1 + y].push(Unit::Rock);
                    }
                } else {
                    for _ in 0..=scan_width {
                        sim.scan_map[sim.y_range.1 + y].push(Unit::Air);
                    }
                }
            }
        }

        sim.sand_source = (sim.x_range.1 % 500 + x_offset, 0);
        sim.scan_map[sim.sand_source.1][sim.sand_source.0] = Unit::SandSource;


        sim
    }

    // Simulate sand falling.
    // Returns all sand positions when done
    fn simulate(&mut self, sleep: Duration, wait_for_rest: bool) -> Vec<(usize, usize)> {
        let mut sand_vec: Vec<(usize, usize)> = vec![];

        print!("{}[2J", 27 as char); // clear screen
        loop {
            // generate sand
            let sand_pos = (self.sand_source.0, self.sand_source.1);

            self.scan_map[sand_pos.1][sand_pos.0] = Unit::Sand;
            sand_vec.push(sand_pos);

            loop {
                let mut moved = false;
                thread::sleep(sleep);

                for sand in sand_vec.iter_mut() {
                    if sand.1 == self.scan_map.len() - 1
                        || sand.0 == self.scan_map[0].len() - 1
                        || sand.0 == 0
                    {
                        let sand_remove = sand_vec.pop().unwrap();
                        self.scan_map[sand_remove.1][sand_remove.0] = Unit::Air;
                        return sand_vec;
                    }

                    match self.scan_map[sand.1 + 1][sand.0] {
                        Unit::Air => {
                            self.scan_map[sand.1][sand.0] = Unit::Air;
                            self.scan_map[sand.1 + 1][sand.0] = Unit::Sand;

                            let new_sand_pos = (sand.0, sand.1 + 1);
                            *sand = new_sand_pos;
                            moved = true;
                        }
                        Unit::Rock | Unit::Sand => {
                            if sand.1 == self.scan_map.len() - 1
                                || sand.0 == self.scan_map[0].len() - 1
                                || sand.0 == 0
                            {
                                let sand_remove = sand_vec.pop().unwrap();
                                self.scan_map[sand_remove.1][sand_remove.0] = Unit::Air;
                                return sand_vec;
                            }
                            // one step down and one to the left
                            match self.scan_map[sand.1 + 1][sand.0 + 1] {
                                Unit::Air => {
                                    self.scan_map[sand.1][sand.0] = Unit::Air;
                                    self.scan_map[sand.1 + 1][sand.0 + 1] = Unit::Sand;

                                    let new_sand_pos = (sand.0 + 1, sand.1 + 1);
                                    *sand = new_sand_pos;
                                    moved = true;
                                }
                                _ => {
                                    // one step down and one to the right
                                    match self.scan_map[sand.1 + 1][sand.0 - 1] {
                                        Unit::Air => {
                                            if sand.1 == self.scan_map.len() - 1
                                                || sand.0 == self.scan_map[0].len() - 1
                                                || sand.0 == 0
                                            {
                                                let sand_remove = sand_vec.pop().unwrap();
                                                self.scan_map[sand_remove.1][sand_remove.0] =
                                                    Unit::Air;
                                                return sand_vec;
                                            }
                                            self.scan_map[sand.1][sand.0] = Unit::Air;
                                            self.scan_map[sand.1 + 1][sand.0 - 1] = Unit::Sand;

                                            let new_sand_pos = (sand.0 - 1, sand.1 + 1);
                                            *sand = new_sand_pos;
                                            moved = true;
                                        }
                                        _ => (),
                                    };
                                }
                            };
                        }
                        _ => (),
                    };
                }

                // uncommment below to print sand
                // print!("{}[1;1H", 27 as char);
                // let mut lock = io::stdout().lock();
                // self.scan_map.iter().enumerate().for_each(|(_, e)| {
                //     e.iter().rev().enumerate().for_each(|(_, e)| {
                //         write!(lock, "{}", e.to_string()).unwrap();
                //     });
                //     writeln!(lock).unwrap();
                // });

                if wait_for_rest {
                    // wait for current sand grain to rest before starting with next
                    if !moved {
                        sand_vec.pop(); // grain is now dormant, remove from list
                        break;
                    }
                } else {
                    if !moved {
                        sand_vec.pop(); // grain is now dormant, remove from list
                    }
                    // Let next sand start falling directly, won't work for part 1 but is much faster for part 2
                    break;
                }
            }

            if self.scan_map[sand_pos.1][sand_pos.0] == Unit::Sand {
                break;
            }
        }
        sand_vec
    }
}

fn part_1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sim = Simulation::from_string(&input, false);

    sim.simulate(Duration::from_millis(0), true);
    sim.scan_map
        .iter()
        .flatten()
        .filter(|e| e.to_owned().to_owned() == Unit::Sand)
        .count()
}

fn part_2() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sim = Simulation::from_string(&input, true);

    sim.simulate(Duration::from_millis(0), false);
    sim.scan_map
        .iter()
        .flatten()
        .filter(|e| e.to_owned().to_owned() == Unit::Sand)
        .count()
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
