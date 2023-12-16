use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn hash(s: &str) -> usize {
    s.bytes()
        .fold(0, |hash, b| (17 * (hash + b as usize)) % 256)
}

pub fn part1(path: &str) {
    let reader = io::BufReader::new(File::open(path).unwrap());
    let line = reader.lines().next().unwrap().unwrap();
    let sum: usize = line.split(',').map(|s| hash(s)).sum();
    println!("{sum}");
}

pub fn part2(path: &str) {
    let reader = io::BufReader::new(File::open(path).unwrap());
    let line = reader.lines().next().unwrap().unwrap();
    let mut box_contents: Vec<Vec<Option<usize>>> = vec![vec![]; 256];
    let mut label_to_idx: HashMap<String, usize> = HashMap::new();
    for cmd in line.split(',') {
        if cmd.ends_with('-') {
            let label = &cmd[..cmd.len() - 1];
            let label_hash = hash(&label);
            if label_to_idx.contains_key(label) {
                let i = label_to_idx[label];
                box_contents[label_hash][i] = None;
                label_to_idx.remove(label);
            }
        } else {
            let mut cmd_parts = cmd.split('=');
            let (label, focal_length) = (
                cmd_parts.next().unwrap(),
                cmd_parts.next().unwrap().parse().unwrap(),
            );
            let label_hash = hash(&label);
            if label_to_idx.contains_key(label) {
                let i = label_to_idx[label];
                box_contents[label_hash][i] = Some(focal_length);
            } else {
                label_to_idx.insert(String::from(label), box_contents[label_hash].len());
                box_contents[label_hash].push(Some(focal_length));
            }
        }
    }
    let sum: usize = box_contents
        .iter()
        .map(|focal_lengths| {
            focal_lengths
                .iter()
                .filter_map(|&x| x)
                .enumerate()
                .map(|(slot_index, focal_len)| (slot_index + 1) * focal_len)
                .sum::<usize>()
        })
        .enumerate()
        .map(|(box_index, sum)| (box_index + 1) * sum)
        .sum();
    println!("{sum}");
}
