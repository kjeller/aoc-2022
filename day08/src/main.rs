use std::cmp;
use std::env;

struct Tree {
    val: u32,
    score: u32,
}

impl Tree {
    fn to_string(&self) -> String {
        format!("{} {}", self.val, self.score)
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl PartialOrd for Tree {
    fn gt(&self, other: &Self) -> bool {
        self.val > other.val
    }
    fn lt(&self, other: &Self) -> bool {
        self.val < other.val
    }

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

// TreeVisibleness is a real word
enum TreeVisibleness {
    Unidentified(Tree),
    Visible(Tree),
    NotVisible(Tree),
}

impl TreeVisibleness {
    fn get(&self) -> &Tree {
        match self {
            TreeVisibleness::Unidentified(t) => t,
            TreeVisibleness::Visible(t) => t,
            TreeVisibleness::NotVisible(t) => t,
        }
    }

    fn to_visible(&self) -> TreeVisibleness {
        match self {
            TreeVisibleness::Unidentified(t) => TreeVisibleness::Visible(Tree {
                val: t.val,
                score: t.score,
            }),
            TreeVisibleness::Visible(t) => TreeVisibleness::Visible(Tree {
                val: t.val,
                score: t.score,
            }),
            TreeVisibleness::NotVisible(t) => TreeVisibleness::Visible(Tree {
                val: t.val,
                score: t.score,
            }),
        }
    }

    fn to_not_visible(&self) -> TreeVisibleness {
        match self {
            TreeVisibleness::Unidentified(t) => TreeVisibleness::NotVisible(Tree {
                val: t.val,
                score: t.score,
            }),
            TreeVisibleness::Visible(t) => TreeVisibleness::NotVisible(Tree {
                val: t.val,
                score: t.score,
            }),
            TreeVisibleness::NotVisible(t) => TreeVisibleness::NotVisible(Tree {
                val: t.val,
                score: t.score,
            }),
        }
    }

    fn set_score(&self, score: u32) -> TreeVisibleness {
        match self {
            TreeVisibleness::Unidentified(t) => {
                TreeVisibleness::NotVisible(Tree { val: t.val, score })
            }
            TreeVisibleness::Visible(t) => TreeVisibleness::NotVisible(Tree { val: t.val, score }),
            TreeVisibleness::NotVisible(t) => {
                TreeVisibleness::NotVisible(Tree { val: t.val, score })
            }
        }
    }

    fn is_visible(&self) -> bool {
        match self {
            TreeVisibleness::Unidentified(_) => false,
            TreeVisibleness::Visible(_) => true,
            TreeVisibleness::NotVisible(_) => false,
        }
    }

    fn to_string(&self) -> String {
        match self {
            TreeVisibleness::Unidentified(t) => format!("U: {}", t.to_string()),
            TreeVisibleness::Visible(t) => format!("V: {}", t.to_string()),
            TreeVisibleness::NotVisible(t) => format!("N: {}", t.to_string()),
        }
    }
}

fn part_1() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let m_size = input.lines().count();
    let mut m: Vec<Vec<TreeVisibleness>> = input
        .lines()
        .map(|line| {
            let mut vec: Vec<TreeVisibleness> = Vec::new();
            line.chars().filter(|c| c.is_digit(10)).for_each(|c| {
                let tree = TreeVisibleness::Unidentified(Tree {
                    val: c.to_digit(10).unwrap(),
                    score: 0, // only used in part 2
                });
                vec.push(tree)
            });
            vec
        })
        .collect();

    for x in 0..m_size {
        for y in 0..m_size {
            // All the outer trees are visible
            if x == 0 || y == 0 || x == m_size - 1 || y == m_size - 1 {
                m[x][y] = m[x][y].to_visible();
                continue;
            }

            m[x][y] = m[x][y].to_visible();

            for h in x + 1..m_size {
                if m[x][y].get() <= m[h][y].get() {
                    m[x][y] = m[x][y].to_not_visible();
                    break;
                }
            }

            if m[x][y].is_visible() {
                continue;
            }

            m[x][y] = m[x][y].to_visible();

            for h in (0..x).rev() {
                if m[x][y].get() <= m[h][y].get() {
                    m[x][y] = m[x][y].to_not_visible();
                    break;
                }
            }

            if m[x][y].is_visible() {
                continue;
            }
            m[x][y] = m[x][y].to_visible();

            for v in y + 1..m_size {
                if m[x][y].get() <= m[x][v].get() {
                    m[x][y] = m[x][y].to_not_visible();
                    break;
                }
            }

            if m[x][y].is_visible() {
                continue;
            }

            m[x][y] = m[x][y].to_visible();

            for v in (0..y).rev() {
                if m[x][y].get() <= m[x][v].get() {
                    m[x][y] = m[x][y].to_not_visible();
                    break;
                }
            }
        }
    }
    let visible_tree_count = m
        .iter()
        .flat_map(|v| v)
        .map(|e| e)
        .filter(|e| e.is_visible())
        .count();
    visible_tree_count as u32
}

fn part_2() -> u32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let m_size = input.lines().count();
    let mut m: Vec<Vec<TreeVisibleness>> = input
        .lines()
        .map(|line| {
            let mut vec: Vec<TreeVisibleness> = Vec::new();
            line.chars().filter(|c| c.is_digit(10)).for_each(|c| {
                let tree = TreeVisibleness::Unidentified(Tree {
                    val: c.to_digit(10).unwrap(),
                    score: 0,
                });
                vec.push(tree)
            });
            vec
        })
        .collect();

    for x in 0..m_size {
        for y in 0..m_size {
            let mut left = 0;
            let mut right = 0;
            let mut up = 0;
            let mut down = 0;

            for h in x + 1..m_size {
                down += 1;
                if m[x][y].get() <= m[h][y].get() {
                    break;
                }
            }

            for h in (0..x).rev() {
                up += 1;
                if m[x][y].get() <= m[h][y].get() {
                    break;
                }
            }

            for v in y + 1..m_size {
                right += 1;
                if m[x][y].get() <= m[x][v].get() {
                    break;
                }
            }

            for v in (0..y).rev() {
                left += 1;
                if m[x][y].get() <= m[x][v].get() {
                    break;
                }
            }
            m[x][y] = m[x][y].set_score(left * right * up * down);
        }
    }
    let visible_tree_count = m
        .iter()
        .flat_map(|v| v)
        .map(|e| e.get().score)
        .max()
        .unwrap();

    visible_tree_count as u32
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
