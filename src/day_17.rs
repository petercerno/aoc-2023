use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord(usize, usize, u8, u8, u8, u8);

struct City {
    map: Vec<Vec<u32>>,
    m: usize,
    n: usize,
}

impl City {
    fn read(path: &str) -> City {
        let map: Vec<Vec<u32>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().map(|b| (b - b'0') as u32).collect())
            .collect();
        let (m, n) = (map.len(), map[0].len());
        City { map, m, n }
    }

    fn edges(&self, c: Coord, ultra: bool) -> Vec<Coord> {
        let mut out = Vec::with_capacity(4);
        if c.0 > 0 && c.2 > 0 {
            if ultra {
                let side = if c.2 - 1 <= 6 { 10 } else { 0 };
                out.push(Coord(c.0 - 1, c.1, c.2 - 1, 0, side, side));
            } else {
                out.push(Coord(c.0 - 1, c.1, c.2 - 1, 0, 3, 3));
            }
        }
        if c.0 < self.m - 1 && c.3 > 0 {
            if ultra {
                let side = if c.3 - 1 <= 6 { 10 } else { 0 };
                out.push(Coord(c.0 + 1, c.1, 0, c.3 - 1, side, side));
            } else {
                out.push(Coord(c.0 + 1, c.1, 0, c.3 - 1, 3, 3));
            }
        }
        if c.1 > 0 && c.4 > 0 {
            if ultra {
                let side = if c.4 - 1 <= 6 { 10 } else { 0 };
                out.push(Coord(c.0, c.1 - 1, side, side, c.4 - 1, 0));
            } else {
                out.push(Coord(c.0, c.1 - 1, 3, 3, c.4 - 1, 0));
            }
        }
        if c.1 < self.n - 1 && c.5 > 0 {
            if ultra {
                let side = if c.5 - 1 <= 6 { 10 } else { 0 };
                out.push(Coord(c.0, c.1 + 1, side, side, 0, c.5 - 1));
            } else {
                out.push(Coord(c.0, c.1 + 1, 3, 3, 0, c.5 - 1));
            }
        }
        out
    }

    fn search(&self, ultra: bool) -> u32 {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push((
            Reverse(0),
            Coord(0, 0, 0, ultra as u8 * 7 + 3, 0, ultra as u8 * 7 + 3),
        ));
        while let Some((Reverse(c0_dist), c0)) = heap.pop() {
            if !visited.insert(c0) {
                continue;
            }
            if c0.0 == self.m - 1 && c0.1 == self.n - 1 {
                return c0_dist;
            }
            for c1 in self.edges(c0, ultra) {
                heap.push((Reverse(c0_dist + self.map[c1.0][c1.1]), c1));
            }
        }
        0
    }
}

pub fn part1(path: &str) {
    println!("{}", City::read(path).search(false));
}

pub fn part2(path: &str) {
    println!("{}", City::read(path).search(true));
}
