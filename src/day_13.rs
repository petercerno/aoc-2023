use std::fs::File;
use std::io::{self, BufRead};

fn reflect(x: &[u64], skip: usize) -> usize {
    let n = x.len();
    for i in 1..n {
        if i == skip {
            continue;
        }
        let m = (n - i).min(i);
        if (0..m).all(|j| x[i - 1 - j] == x[i + j]) {
            return i;
        }
    }
    0
}

struct Pattern {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl Pattern {
    fn new() -> Pattern {
        Pattern {
            rows: vec![],
            cols: vec![],
        }
    }

    fn append(&mut self, line: &[u8]) {
        let n = line.len();
        if self.cols.is_empty() {
            self.cols = vec![0; n];
        }
        let mut row = 0;
        for i in 0..n {
            row <<= 1;
            self.cols[i] <<= 1;
            if line[i] == b'#' {
                row += 1;
                self.cols[i] += 1;
            }
        }
        self.rows.push(row);
    }

    fn flip(&mut self, i: usize, j: usize) {
        let (m, n) = (self.rows.len(), self.cols.len());
        self.rows[i] ^= 1 << (n - 1 - j);
        self.cols[j] ^= 1 << (m - 1 - i);
    }

    fn score(&mut self, original: bool) -> usize {
        let row = reflect(&self.rows, 0);
        let col = reflect(&self.cols, 0);
        if original {
            return 100 * row + col;
        }
        for i in 0..self.rows.len() {
            for j in 0..self.cols.len() {
                self.flip(i, j);
                let score = 100 * reflect(&self.rows, row) + reflect(&self.cols, col);
                self.flip(i, j);
                if score > 0 {
                    return score;
                }
            }
        }
        panic!("Alternate score not found!");
    }
}

fn solve(path: &str, original: bool) {
    let mut sum = 0;
    let mut pat = Pattern::new();
    for line in io::BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            sum += pat.score(original);
            pat = Pattern::new();
        } else {
            pat.append(line.as_bytes());
        }
    }
    sum += pat.score(original);
    println!("{sum}");
}

pub fn part1(path: &str) {
    solve(path, true);
}

pub fn part2(path: &str) {
    solve(path, false);
}
