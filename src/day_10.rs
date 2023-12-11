use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Coord) -> Coord {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

fn offsets(b: u8) -> [Coord; 2] {
    match b {
        b'|' => [Coord(1, 0), Coord(-1, 0)],
        b'-' => [Coord(0, 1), Coord(0, -1)],
        b'L' => [Coord(0, 1), Coord(-1, 0)],
        b'J' => [Coord(0, -1), Coord(-1, 0)],
        b'7' => [Coord(0, -1), Coord(1, 0)],
        b'F' => [Coord(0, 1), Coord(1, 0)],
        _ => panic!("Invalid input: {}", b as char),
    }
}

fn find_start(map: &Vec<Vec<u8>>) -> Coord {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i as usize][j as usize] == b'S' {
                return Coord(i as i32, j as i32);
            }
        }
    }
    panic!("Could not find start!");
}

struct PipeMaze {
    map: Vec<Vec<u8>>,
    m: usize,
    n: usize,
    s: Coord,
}

impl PipeMaze {
    fn read(path: &str) -> PipeMaze {
        let mut map: Vec<Vec<u8>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().collect())
            .collect();
        let m = map.len();
        let n = map[0].len();
        let s = find_start(&map);
        let (si, sj) = (s.0 as usize, s.1 as usize);
        let north = si > 0 && [b'|', b'7', b'F'].contains(&map[si - 1][sj]);
        let south = si < m - 1 && [b'|', b'L', b'J'].contains(&map[si + 1][sj]);
        let west = sj > 0 && [b'-', b'L', b'F'].contains(&map[si][sj - 1]);
        let east = sj < n - 1 && [b'-', b'J', b'7'].contains(&map[si][sj + 1]);
        map[s.0 as usize][s.1 as usize] = match (north, south, west, east) {
            (true, true, false, false) => b'|',
            (false, false, true, true) => b'-',
            (true, false, false, true) => b'L',
            (true, false, true, false) => b'J',
            (false, true, true, false) => b'7',
            (false, true, false, true) => b'F',
            _ => panic!("Invalid start!"),
        };
        PipeMaze { map, m, n, s }
    }

    fn neighbors(&self, c: Coord) -> [Coord; 2] {
        let [d0, d1] = offsets(self.map[c.0 as usize][c.1 as usize]);
        [c + d0, c + d1]
    }

    fn cycle(&self) -> Vec<Coord> {
        let mut out = vec![];
        out.push(self.s);
        let mut c0 = self.s;
        let mut c1 = self.neighbors(c0)[0];
        while c1 != self.s {
            out.push(c1);
            let [n0, n1] = self.neighbors(c1);
            if n0 == c0 {
                c0 = c1;
                c1 = n1;
            } else {
                c0 = c1;
                c1 = n0;
            }
        }
        out
    }

    fn num_inner_nodes(&self) -> usize {
        let mut sum = 0;
        let mut is_cycle = vec![vec![false; self.n]; self.m];
        for c in self.cycle() {
            is_cycle[c.0 as usize][c.1 as usize] = true;
        }
        for i in 0..self.m {
            let mut is_inside = false;
            let mut j = 0;
            while j < self.n {
                if !is_cycle[i][j] {
                    if is_inside {
                        sum += 1;
                    }
                } else if self.map[i][j] == b'|' {
                    is_inside = !is_inside;
                } else if self.map[i][j] == b'L' {
                    j += 1;
                    while j < self.n && self.map[i][j] == b'-' {
                        j += 1;
                    }
                    if self.map[i][j] == b'7' {
                        is_inside = !is_inside;
                    }
                } else if self.map[i][j] == b'F' {
                    j += 1;
                    while j < self.n && self.map[i][j] == b'-' {
                        j += 1;
                    }
                    if self.map[i][j] == b'J' {
                        is_inside = !is_inside;
                    }
                }
                j += 1;
            }
        }
        sum
    }
}

pub fn part1(path: &str) {
    let maze = PipeMaze::read(path);
    println!("{}", (maze.cycle().len() + 1) / 2);
}

pub fn part2(path: &str) {
    let maze = PipeMaze::read(path);
    println!("{}", maze.num_inner_nodes());
}
