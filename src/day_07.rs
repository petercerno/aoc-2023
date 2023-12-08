use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct HandBuilder {
    map: HashMap<u8, usize>,
    joker: bool,
}

type Hand = ([usize; 5], [usize; 5]);

impl HandBuilder {
    fn new(joker: bool) -> HandBuilder {
        let priority = if joker {
            "J23456789TQKA"
        } else {
            "23456789TJQKA"
        };
        HandBuilder {
            map: HashMap::from_iter(priority.as_bytes().iter().enumerate().map(|(i, &b)| (b, i))),
            joker,
        }
    }

    fn build(&self, s: &str) -> Hand {
        let mut cards = [0; 5];
        s.bytes()
            .enumerate()
            .for_each(|(i, b)| cards[i] = self.map[&b]);
        let mut card_counts = [0usize; 13];
        cards.iter().for_each(|&c| card_counts[c] += 1);
        if self.joker {
            // Convert all Jokers to the most frequent non-Joker card.
            let max = card_counts
                .iter()
                .enumerate()
                .skip(1) // Skip Joker cards
                .map(|(i, &c)| (c, i))
                .max()
                .unwrap();
            card_counts[max.1] += card_counts[0];
            card_counts[0] = 0;
        }
        let mut hist = [0usize; 5];
        card_counts
            .iter()
            .filter(|&&c| c > 0)
            .for_each(|&c| hist[5 - c] += 1);
        (hist, cards)
    }
}

fn solve(path: &str, joker: bool) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let hand_builder = HandBuilder::new(joker);
    let mut input: Vec<(Hand, usize)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<_> = line.split(' ').collect();
            let input = (hand_builder.build(parts[0]), parts[1].parse().unwrap());
            input
        })
        .collect();
    input.sort();
    let sum: usize = input.iter().enumerate().map(|(i, h)| (i + 1) * h.1).sum();
    println!("{sum}");
}

pub fn part1(path: &str) {
    solve(path, false);
}

pub fn part2(path: &str) {
    solve(path, true);
}
