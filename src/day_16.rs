use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Coord(usize, usize, u8);

fn update(c: Coord, visits: &mut Vec<Vec<u8>>) -> bool {
    let visit = &mut visits[c.0][c.1];
    let old_visit = *visit;
    *visit |= match c.2 {
        b'^' => 1,
        b'v' => 2,
        b'<' => 4,
        b'>' => 8,
        _ => panic!("Invalid direction: '{}'", c.2 as char),
    };
    *visit != old_visit
}

struct Contraption {
    map: Vec<Vec<u8>>,
    m: usize,
    n: usize,
}

impl Contraption {
    fn read(path: &str) -> Contraption {
        let map: Vec<Vec<u8>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().collect())
            .collect();
        let (m, n) = (map.len(), map[0].len());
        Contraption { map, m, n }
    }

    fn step(&self, c: Coord) -> Vec<Coord> {
        let mut out = Vec::with_capacity(2);
        match (self.map[c.0][c.1], c.2) {
            (b'.', b'^') | (b'|', b'^') | (b'\\', b'<') | (b'/', b'>') if c.0 > 0 => {
                out.push(Coord(c.0 - 1, c.1, b'^'))
            }
            (b'.', b'v') | (b'|', b'v') | (b'\\', b'>') | (b'/', b'<') if c.0 < self.m - 1 => {
                out.push(Coord(c.0 + 1, c.1, b'v'))
            }
            (b'.', b'<') | (b'-', b'<') | (b'\\', b'^') | (b'/', b'v') if c.1 > 0 => {
                out.push(Coord(c.0, c.1 - 1, b'<'))
            }
            (b'.', b'>') | (b'-', b'>') | (b'\\', b'v') | (b'/', b'^') if c.1 < self.n - 1 => {
                out.push(Coord(c.0, c.1 + 1, b'>'))
            }
            (b'|', b'<') | (b'|', b'>') => {
                if c.0 > 0 {
                    out.push(Coord(c.0 - 1, c.1, b'^'))
                }
                if c.0 < self.m - 1 {
                    out.push(Coord(c.0 + 1, c.1, b'v'))
                }
            }
            (b'-', b'^') | (b'-', b'v') => {
                if c.1 > 0 {
                    out.push(Coord(c.0, c.1 - 1, b'<'))
                }
                if c.1 < self.n - 1 {
                    out.push(Coord(c.0, c.1 + 1, b'>'))
                }
            }
            _ => (),
        }
        out
    }

    fn multi_step(&self, beams: Vec<Coord>, visits: &mut Vec<Vec<u8>>) -> Vec<Coord> {
        let mut out = Vec::with_capacity(2 * beams.len());
        for c in beams {
            if update(c, visits) {
                out.append(&mut self.step(c));
            }
        }
        out
    }

    fn explore(&self, c: Coord) -> usize {
        let mut visits = vec![vec![0; self.n]; self.m];
        let mut beams = vec![c];
        while !beams.is_empty() {
            beams = self.multi_step(beams, &mut visits);
        }
        visits
            .iter()
            .map(|row| row.iter().filter(|&&x| x != 0).count())
            .sum()
    }

    fn explore_all(&self) -> usize {
        let top = (0..self.n).map(|j| Coord(0, j, b'v'));
        let bottom = (0..self.n).map(|j| Coord(self.m - 1, j, b'^'));
        let left = (0..self.m).map(|i| Coord(i, 0, b'>'));
        let right = (0..self.m).map(|i| Coord(i, self.n - 1, b'<'));
        top.chain(bottom)
            .chain(left)
            .chain(right)
            .map(|c| self.explore(c))
            .max()
            .unwrap()
    }
}

pub fn part1(path: &str) {
    let contraption = Contraption::read(path);
    let sum = contraption.explore(Coord(0, 0, b'>'));
    println!("{sum}");
}

pub fn part2(path: &str) {
    let contraption = Contraption::read(path);
    let sum = contraption.explore_all();
    println!("{sum}");
}
