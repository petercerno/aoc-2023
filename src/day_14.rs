use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Platform {
    map: Vec<Vec<u8>>,
    m: usize,
    n: usize,
}

impl Platform {
    fn read(path: &str) -> Platform {
        let map: Vec<Vec<u8>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().collect())
            .collect();
        let (m, n) = (map.len(), map[0].len());
        Platform { map, m, n }
    }

    fn tilt<U, V>(&mut self, u: usize, v: usize, ut: U, vt: V)
    where
        U: Fn(usize, usize) -> usize,
        V: Fn(usize, usize) -> usize,
    {
        let mut place = vec![0; v];
        for x in 0..u {
            for y in 0..v {
                let (i0, j0) = (ut(x, y), vt(x, y));
                let (i1, j1) = (ut(place[y], y), vt(place[y], y));
                match self.map[i0][j0] {
                    b'O' => {
                        self.map[i0][j0] = b'.';
                        self.map[i1][j1] = b'O';
                        place[y] += 1;
                    }
                    b'#' => place[y] = x + 1,
                    _ => (),
                }
            }
        }
    }

    fn cycle(&mut self) {
        let (m, n) = (self.m, self.n);
        self.tilt(m, n, |i, _| i, |_, j| j); // north
        self.tilt(n, m, |_, i| i, |j, _| j); // west
        self.tilt(m, n, |i, _| m - 1 - i, |_, j| j); // south
        self.tilt(n, m, |_, i| i, |j, _| n - 1 - j); // east
    }

    fn key(&self) -> String {
        String::from_utf8(self.map.concat()).unwrap()
    }

    fn load(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, r)| (self.m - i) * r.iter().filter(|&&b| b == b'O').count())
            .sum()
    }
}

pub fn part1(path: &str) {
    let mut platform = Platform::read(path);
    platform.tilt(platform.m, platform.n, |i, _| i, |_, j| j);
    println!("{}", platform.load());
}

pub fn part2(path: &str) {
    let mut platform = Platform::read(path);
    let mut platform_step: HashMap<String, usize> = HashMap::new();
    let mut step = 0;
    let mut num_steps = 1_000_000_000;
    let mut cycle_len = 0;
    while step < num_steps {
        if cycle_len == 0 {
            let key = platform.key();
            if platform_step.contains_key(&key) {
                cycle_len = step - platform_step[&key];
                num_steps = step + (num_steps - step) % cycle_len;
            } else {
                platform_step.insert(key, step);
            }
        }
        platform.cycle();
        step += 1;
    }
    println!("{}", platform.load());
}
