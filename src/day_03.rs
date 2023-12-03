use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Number {
    row: usize,
    col: usize,
    len: usize,
    num: u32,
}

impl Number {
    fn surroundings(&self) -> Vec<(i32, i32)> {
        let mut out = vec![];
        out.reserve(2 * self.len + 6);
        let row = self.row as i32;
        let col = self.col as i32;
        let len = self.len as i32;
        for j in col - 1..=col + len {
            out.push((row - 1, j));
            out.push((row + 1, j));
        }
        out.push((row, col - 1));
        out.push((row, col + len));
        out
    }
}

struct Engine {
    schema: Vec<String>,
    numbers: Vec<Number>,
    rows: usize,
    cols: usize,
}

type StarsHashMap = HashMap<(i32, i32), Vec<u32>>;

impl Engine {
    fn new(path: &str) -> Engine {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut engine = Engine {
            schema: vec![],
            numbers: vec![],
            rows: 0,
            cols: 0,
        };
        for line in reader.lines() {
            let line = line.unwrap();
            let bytes = line.as_bytes();
            if engine.cols == 0 {
                engine.cols = line.len();
            } else {
                assert!(engine.cols == line.len());
            }
            let mut i: usize = 0;
            while i < engine.cols {
                if bytes[i].is_ascii_digit() {
                    let mut number = Number {
                        row: engine.rows,
                        col: i,
                        len: 0,
                        num: 0,
                    };
                    while i < engine.cols && bytes[i].is_ascii_digit() {
                        number.len += 1;
                        number.num = 10 * number.num + (bytes[i] - b'0') as u32;
                        i += 1;
                    }
                    engine.numbers.push(number);
                } else {
                    i += 1;
                }
            }
            engine.rows += 1;
            engine.schema.push(line);
        }
        engine
    }

    fn get(&self, i: i32, j: i32) -> Option<u8> {
        if (0..self.rows as i32).contains(&i) && (0..self.cols as i32).contains(&j) {
            Some(self.schema[i as usize].as_bytes()[j as usize])
        } else {
            None
        }
    }

    fn is_symbol(&self, i: i32, j: i32) -> bool {
        if let Some(b) = self.get(i, j) {
            !b.is_ascii_digit() && b != b'.'
        } else {
            false
        }
    }

    fn is_adjacent_to_symbol(&self, number: &Number) -> bool {
        number
            .surroundings()
            .iter()
            .any(|&(i, j)| self.is_symbol(i, j))
    }

    fn is_star(&self, i: i32, j: i32) -> bool {
        self.get(i, j) == Some(b'*')
    }

    fn add_number_to_stars(&self, number: &Number, stars_map: &mut StarsHashMap) {
        for (i, j) in number.surroundings() {
            if self.is_star(i, j) {
                stars_map.entry((i, j)).or_default().push(number.num);
            }
        }
    }
}

pub fn part1(path: &str) {
    let engine = Engine::new(path);
    let sum: u32 = engine
        .numbers
        .iter()
        .filter(|number| engine.is_adjacent_to_symbol(number))
        .map(|number| number.num)
        .sum();
    println!("{sum}");
}

pub fn part2(path: &str) {
    let engine = Engine::new(path);
    let mut stars_map: StarsHashMap = HashMap::new();
    engine
        .numbers
        .iter()
        .for_each(|number| engine.add_number_to_stars(number, &mut stars_map));
    let sum: u32 = stars_map
        .values()
        .filter_map(|v| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum();
    println!("{sum}");
}
