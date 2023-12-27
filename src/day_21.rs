use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i64, i64);

struct Map {
    map: Vec<Vec<u8>>,
    m: usize,
    n: usize,
}

fn rem(x: i64, m: usize) -> usize {
    let m = m as i64;
    let r = x % m;
    if r < 0 {
        (r + m) as usize
    } else {
        r as usize
    }
}

impl Map {
    fn read(path: &str) -> Map {
        let map: Vec<Vec<u8>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().collect())
            .collect();
        let (m, n) = (map.len(), map[0].len());
        Map { map, m, n }
    }

    fn start(&self) -> Coord {
        for i in 0..self.m {
            for j in 0..self.n {
                if self.map[i][j] == b'S' {
                    return Coord(i as i64, j as i64);
                }
            }
        }
        panic!("Could not find start");
    }

    fn edges(&self, c: Coord, cyclic: bool) -> Vec<Coord> {
        let (i, j) = (rem(c.0, self.m), rem(c.1, self.n));
        let mut out = Vec::with_capacity(4);
        if (cyclic || i > 0) && self.map[(i + self.m - 1) % self.m][j] != b'#' {
            out.push(Coord(c.0 - 1, c.1));
        }
        if (cyclic || i < self.m - 1) && self.map[(i + 1) % self.m][j] != b'#' {
            out.push(Coord(c.0 + 1, c.1));
        }
        if (cyclic || j > 0) && self.map[i][(j + self.n - 1) % self.n] != b'#' {
            out.push(Coord(c.0, c.1 - 1));
        }
        if (cyclic || j < self.n - 1) && self.map[i][(j + 1) % self.n] != b'#' {
            out.push(Coord(c.0, c.1 + 1));
        }
        out
    }

    fn bfs(&self, max_dist: usize, steps: usize, cyclic: bool) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut cnt = vec![0i64; steps + 1];
        let c0 = self.start();
        queue.push_back((c0, 0usize));
        visited.insert(c0);
        cnt[0] += 1;
        while let Some((c0, d0)) = queue.pop_front() {
            if d0 == max_dist {
                continue;
            }
            for c1 in self.edges(c0, cyclic) {
                if !visited.contains(&c1) {
                    queue.push_back((c1, d0 + 1));
                    visited.insert(c1);
                    cnt[d0 + 1] += 1;
                }
            }
        }
        let m = self.m;
        for d in max_dist + 1..=steps {
            let prev_deriv1 = cnt[d - m] - cnt[d - m - 1];
            let prev_deriv2 = cnt[d - m] - cnt[d - m - 1] - (cnt[d - 2 * m] - cnt[d - 2 * m - 1]);
            let this_deriv1 = prev_deriv1 + prev_deriv2;
            cnt[d] = cnt[d - 1] + this_deriv1;
        }
        let mut acc = [0i64; 2];
        for (i, c) in cnt.iter().enumerate() {
            acc[i % 2] += c;
        }
        println!("{:?}", acc[steps % 2]);
    }
}

pub fn part1(path: &str) {
    Map::read(path).bfs(64, 64, false);
}

pub fn part2(path: &str) {
    let map = Map::read(path);
    map.bfs(map.m * 4, 26501365, true);
}
