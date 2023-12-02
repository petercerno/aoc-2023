use std::fs::File;
use std::io::{self, BufRead};

type RGB = [u32; 3];

struct Game {
    game_id: u32,
    subsets: Vec<RGB>,
}

impl Game {
    fn parse(line: &str) -> Game {
        assert!(line.starts_with("Game "));
        let colon = line.find(':').unwrap();
        Game {
            game_id: line[5..colon].parse().unwrap(),
            subsets: line[colon + 2..].split("; ").map(Game::parse_rgb).collect(),
        }
    }

    fn parse_rgb(s: &str) -> RGB {
        let mut out: RGB = [0, 0, 0];
        for color in s.split(", ") {
            let space = color.find(' ').unwrap();
            let idx: usize = match &color[space + 1..] {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!("Invalid color!"),
            };
            out[idx] = color[0..space].parse().unwrap();
        }
        out
    }

    fn max(&self) -> RGB {
        let mut out: RGB = [0, 0, 0];
        for rgb in self.subsets.iter() {
            out = [out[0].max(rgb[0]), out[1].max(rgb[1]), out[2].max(rgb[2])];
        }
        out
    }

    fn valid(&self) -> bool {
        let max = self.max();
        max[0] <= 12 && max[1] <= 13 && max[2] <= 14
    }

    fn power(&self) -> u32 {
        let max = self.max();
        max[0] * max[1] * max[2]
    }
}

pub fn part1(path: &str) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let sum: u32 = reader
        .lines()
        .map(|line| Game::parse(&line.unwrap()))
        .filter(|game| game.valid())
        .map(|game| game.game_id)
        .sum();
    println!("{sum}");
}

pub fn part2(path: &str) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let sum: u32 = reader
        .lines()
        .map(|line| Game::parse(&line.unwrap()))
        .map(|game| game.power())
        .sum();
    println!("{sum}");
}
