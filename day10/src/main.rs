use std::env;

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Addx(x) => format!("addx {}", x),
            Instruction::Noop => format!("noop"),
        }
    }
}
struct SimpleCPU {
    x: i32,
    cycle: i32,
    sig_str: i32,
}

impl SimpleCPU {
    fn new() -> SimpleCPU {
        SimpleCPU {
            x: 1,
            cycle: 1,
            sig_str: 0,
        }
    }

    fn clock(&mut self) {
        self.cycle += 1;
    }

    fn eval_instr(&mut self, instr: Instruction) {
        match instr {
            Instruction::Addx(x) => self.x += x,
            Instruction::Noop => (),
        }
    }

    fn incr_sig_str(&mut self) {
        if self.cycle == 20 {
            self.sig_str += self.cycle * self.x;
        } else if (self.cycle + 20) % 40 == 0 {
            self.sig_str += self.cycle * self.x;
        }
    }
}

struct CRT {
    width: i32,
    height: i32,
    curr_row: Vec<char>,
    sprite_pos: i32,
}

impl CRT {
    fn new() -> CRT {
        CRT {
            width: 40,
            height: 6,
            curr_row: Vec::new(),
            sprite_pos: 1,
        }
    }

    fn update_screen(&mut self, cpu_cycle: i32) {
        // Handle edge
        if cpu_cycle % self.width == 0 {
            self.curr_row.push('.');
            return;
        }

        if (cpu_cycle % self.width) <= self.sprite_pos + 2
            && (cpu_cycle % self.width) >= self.sprite_pos
        {
            self.curr_row.push('#');
        } else {
            self.curr_row.push('.');
        }
    }
}

fn part_1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut cpu: SimpleCPU = SimpleCPU::new();

    input.lines().into_iter().for_each(|line| {
        let n = match &line[0..4] {
            "addx" => Instruction::Addx(line[5..].parse().unwrap()),
            _ => Instruction::Noop,
        };
        match n {
            Instruction::Addx(_) => {
                cpu.incr_sig_str();
                cpu.clock();
                cpu.incr_sig_str();
                cpu.clock();
            }
            Instruction::Noop => {
                cpu.incr_sig_str();
                cpu.clock();
            }
        }
        cpu.eval_instr(n);
    });
    cpu.sig_str
}

fn part_2() -> String {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut cpu: SimpleCPU = SimpleCPU::new();
    let mut crt: CRT = CRT::new();

    input.lines().into_iter().for_each(|line| {
        let n = match &line[0..4] {
            "addx" => Instruction::Addx(line[5..].parse().unwrap()),
            _ => Instruction::Noop,
        };

        match n {
            Instruction::Addx(_) => {
                crt.update_screen(cpu.cycle);
                cpu.clock();

                crt.update_screen(cpu.cycle);
                cpu.clock();
            }
            Instruction::Noop => {
                crt.update_screen(cpu.cycle);
                cpu.clock();
            }
        }
        cpu.eval_instr(n);
        crt.sprite_pos = cpu.x; // set sprite position
    });

    crt.curr_row.chunks(crt.width as usize).for_each(|c| {
        let s: String = c.iter().collect();
        println!("{}", s);
    });
    "".to_string()
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
