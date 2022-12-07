use std::{collections::HashMap, env};
struct Dir {
    name: String,
    files: HashMap<String, File>,
    dirs: HashMap<String, Dir>,
}

impl Dir {
    fn get_files_size(&self) -> u32 {
        self.files.values().map(|file| file.size).sum()
    }

    fn get_total_size (&self) -> u32 {
        let mut size = self.get_files_size();
        get_dirs_in_dir(self)
            .iter()
            .for_each(|d| size += d.get_files_size());
        size
    }
}

struct File {
    name: String,
    size: u32,
}

enum CommandLine {
    Ls,
    Cd(String),
    File(File),
    Dir(Dir),
}

struct FileSystem {
    dirs: HashMap<String, Dir>,
}

fn get_dirs_in_dir(dir: &Dir) -> Vec<&Dir> {
    let mut dirs: Vec<&Dir> = Vec::new();

    for d in dir.dirs.values() {
        dirs.push(d);
        dirs.append(&mut get_dirs_in_dir(d))
    }

    dirs
}

fn parse_filesystem(input: String) -> FileSystem {
    let mut fs = FileSystem {
        dirs: HashMap::new(),
    };
    let mut pwd: Vec<String> = Vec::new();

    let root: Dir = Dir {
        name: "/".to_string(),
        files: HashMap::new(),
        dirs: HashMap::new(),
    };
    fs.dirs.insert(root.name.to_string(), root);

    for line in input.lines() {
        let cmd = match &line[0..4] {
            "$ cd" => CommandLine::Cd(line[5..].to_string()),
            "$ ls" => CommandLine::Ls, // Nothing to do here!
            "dir " => CommandLine::Dir(Dir {
                name: line[4..].to_string(),
                files: HashMap::new(),
                dirs: HashMap::new(),
            }),
            _ => {
                let (size, name) = line.split_once(" ").unwrap();
                let file = File {
                    size: size.parse().unwrap(),
                    name: name.to_string(),
                };
                CommandLine::File(file)
            }
        };

        match cmd {
            CommandLine::Cd(dir) => {
                if dir.eq("..") {
                    pwd.pop().unwrap();
                } else {
                    pwd.push(dir);
                }
            }
            CommandLine::File(file) => {
                let mut curdir = fs.dirs.get_mut(pwd.first().unwrap()).unwrap();

                for directory in pwd.iter().skip(1) {
                    curdir = curdir.dirs.get_mut(directory).unwrap()
                }

                curdir.files.insert(file.name.to_string(), file);
            }
            CommandLine::Dir(dir) => {
                let mut curdir = fs.dirs.get_mut(pwd.first().unwrap()).unwrap();

                for directory in pwd.iter().skip(1) {
                    curdir = curdir.dirs.get_mut(directory).unwrap()
                }

                curdir.dirs.insert(dir.name.to_string(), dir);
            }
            CommandLine::Ls => (),
        }
    }
    fs
}

fn part_1() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let fs = parse_filesystem(input);

    get_dirs_in_dir(&fs.dirs["/"])
        .iter()
        .map(|dir| dir.get_total_size())
        .filter(|&v| v < 100000)
        .sum()
}

fn part_2() -> u32 {
    const TOTAL_DISK_SPACE: u32 = 70000000;
    const FREE_DISK_SPACE: u32 = 30000000;
    let input = std::fs::read_to_string("input.txt").unwrap();
    let fs = parse_filesystem(input);

    let free_disk_space = TOTAL_DISK_SPACE - &fs.dirs["/"].get_total_size();

    get_dirs_in_dir(&fs.dirs["/"])
        .iter()
        .map(|dir| dir.get_total_size())
        .filter(|&size| size + free_disk_space >= FREE_DISK_SPACE)
        .min()
        .unwrap()
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