use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

struct Card {
    num_winning: usize,
}

impl Card {
    fn parse(s: &str) -> Card {
        assert!(s.starts_with("Card"));
        let colon = s.find(':').unwrap();
        let parts = s[colon + 1..].split(" | ").collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        let winning = Card::parse_numbers(parts[0]);
        let numbers = Card::parse_numbers(parts[1]);
        Card {
            num_winning: winning.intersection(&numbers).count(),
        }
    }

    fn parse_numbers(s: &str) -> HashSet<u32> {
        s.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn points(&self) -> u32 {
        if self.num_winning > 0 {
            1 << self.num_winning - 1
        } else {
            0
        }
    }
}

pub fn part1(path: &str) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let sum: u32 = reader
        .lines()
        .map(|s| Card::parse(&s.unwrap()).points())
        .sum();
    println!("{sum}");
}

pub fn part2(path: &str) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let cards = reader
        .lines()
        .map(|s| Card::parse(&s.unwrap()))
        .collect::<Vec<_>>();
    // Note: A more efficient implementation could be done using Segment Trees.
    let mut count = vec![1u32; cards.len()];
    cards.iter().enumerate().for_each(|(i, card)| {
        let upper_bound = (i + card.num_winning + 1).min(cards.len());
        for j in i + 1..upper_bound {
            count[j] += count[i];
        }
    });
    let sum: u32 = count.iter().sum();
    println!("{sum}");
}
