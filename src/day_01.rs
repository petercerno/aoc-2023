use std::fs::File;
use std::io::{self, BufRead};

pub fn part1(path: &str) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let sum: u32 = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            let first_digit = bytes
                .iter()
                .find_map(|&b| (b as char).to_digit(10))
                .expect("No digit found");
            let last_digit = bytes
                .iter()
                .rev()
                .find_map(|&b| (b as char).to_digit(10))
                .expect("No digit found");
            first_digit * 10u32 + last_digit
        })
        .sum();
    println!("{sum}");
}

pub fn part2(path: &str) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let find_digit = |s: &str| -> Option<u32> {
        s.chars().next().and_then(|c| c.to_digit(10)).or_else(|| {
            digits.iter().enumerate().find_map(|(i, &digit)| {
                if s.starts_with(digit) {
                    Some(i as u32 + 1)
                } else {
                    None
                }
            })
        })
    };
    let sum: u32 = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let first_digit = (0..line.len())
                .find_map(|i| find_digit(&line[i..]))
                .expect("No digit found");
            let last_digit = (0..line.len())
                .rev()
                .find_map(|i| find_digit(&line[i..]))
                .expect("No digit found");
            first_digit * 10u32 + last_digit
        })
        .sum();
    println!("{sum}");
}
