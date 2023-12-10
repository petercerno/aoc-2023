use std::fs::File;
use std::io::{self, BufRead};

fn parse_numbers(s: &str) -> Vec<i64> {
    s.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn predict(mut x: Vec<i64>) -> i64 {
    let n = x.len();
    let mut sum = 0;
    for k in 1..n {
        let mut all_zeros = true;
        for i in 0..n - k {
            x[i] = x[i + 1] - x[i];
            all_zeros &= x[i] == 0;
        }
        sum += x[n - k];
        if all_zeros {
            break;
        }
    }
    sum
}

fn predict_rev(mut x: Vec<i64>) -> i64 {
    let n = x.len();
    let mut sum = x[0];
    for k in 1..n {
        let mut all_zeros = true;
        for i in 0..n - k {
            x[i] = x[i + 1] - x[i];
            all_zeros &= x[i] == 0;
        }
        if k % 2 == 0 {
            sum += x[0];
        } else {
            sum -= x[0];
        }
        if all_zeros {
            break;
        }
    }
    sum
}

fn solve<F: Fn(Vec<i64>) -> i64>(path: &str, f: F) {
    let sum: i64 = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| f(parse_numbers(&line.unwrap())))
        .sum();
    println!("{sum}");
}

pub fn part1(path: &str) {
    solve(path, predict);
}

pub fn part2(path: &str) {
    solve(path, predict_rev);
}
