use std::fs::File;
use std::io::{self, BufRead};

fn read_input(path: &str) -> (String, String) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let (line1, line2) = (
        lines.next().unwrap().unwrap(),
        lines.next().unwrap().unwrap(),
    );
    assert!(line1.starts_with("Time:"));
    assert!(line2.starts_with("Distance:"));
    (line1, line2)
}

fn parse_numbers(s: &str) -> Vec<f64> {
    s.split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_concat_number(s: &str) -> f64 {
    s.split_ascii_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse()
        .unwrap()
}

// (t - x) * x > d
// x^2 - t * x + d < 0
// (x - t/2)^2 - t^2 / 4 + d < 0
// |x - t/2| < sqrt(t^2 / 4 - d)
fn win_count(t: f64, d: f64) -> u64 {
    let k = f64::sqrt(t * t / 4.0 - d);
    let l = ((t / 2.0 - k).floor() + 1.0).max(0.0) as u64;
    let r = ((t / 2.0 + k).ceil() - 1.0) as u64;
    r - l + 1
}

pub fn part1(path: &str) {
    let (line1, line2) = read_input(path);
    let times = parse_numbers(&line1[5..]);
    let distances = parse_numbers(&line2[9..]);
    let num_ways: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| win_count(t, d))
        .product();
    println!("{num_ways}");
}

pub fn part2(path: &str) {
    let (line1, line2) = read_input(path);
    let time = parse_concat_number(&line1[5..]);
    let distance = parse_concat_number(&line2[9..]);
    let num_ways = win_count(time, distance);
    println!("{num_ways}");
}
