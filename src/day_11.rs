use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn solve(path: &str, empty_space: i64) {
    let map: Vec<Vec<u8>> = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.bytes().collect())
        .collect();
    let (m, n) = (map.len(), map[0].len());
    let empty_rows: HashSet<_> = (0..m)
        .filter(|&i| map[i].iter().all(|&x| x == b'.'))
        .collect();
    let empty_cols: HashSet<_> = (0..n)
        .filter(|&j| (0..m).all(|i| map[i][j] == b'.'))
        .collect();
    let mut gal = vec![];
    let mut row = 0i64;
    for i in 0..m {
        let mut col = 0i64;
        if empty_rows.contains(&i) {
            row += empty_space;
            continue;
        }

        for j in 0..n {
            if empty_cols.contains(&j) {
                col += empty_space;
                continue;
            }

            if map[i][j] == b'#' {
                gal.push((row, col));
            }
            col += 1;
        }
        row += 1;
    }
    let mut sum = 0;
    for i in 0..gal.len() - 1 {
        for j in i + 1..gal.len() {
            sum += (gal[i].0 - gal[j].0).abs() + (gal[i].1 - gal[j].1).abs()
        }
    }
    println!("{sum}");
}

pub fn part1(path: &str) {
    solve(path, 2);
}

pub fn part2(path: &str) {
    solve(path, 1_000_000);
}
