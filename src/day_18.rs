use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Lagoon {
    edges: Vec<(i64, i64, i64, i64)>,
}

impl Lagoon {
    fn read(path: &str, parse_hex: bool) -> Lagoon {
        let mut edges = vec![];
        let mut i = 0i64;
        let mut j = 0i64;
        for line in io::BufReader::new(File::open(path).unwrap()).lines() {
            let line = line.unwrap();
            let parts: Vec<_> = line.split(' ').collect();
            let dir: u8;
            let len: i64;
            if !parse_hex {
                dir = parts[0].as_bytes()[0];
                len = parts[1].parse().unwrap();
            } else {
                dir = match parts[2].as_bytes()[7] {
                    b'0' => b'R',
                    b'1' => b'D',
                    b'2' => b'L',
                    b'3' => b'U',
                    _ => panic!("Invalid hex code: '{}'", &parts[2][2..8]),
                };
                len = i64::from_str_radix(&parts[2][2..7], 16).unwrap();
            }
            edges.push(match dir {
                b'U' => {
                    i -= len;
                    (i, j, i + len, j)
                }
                b'D' => {
                    i += len;
                    (i - len, j, i, j)
                }
                b'L' => {
                    j -= len;
                    (i, j, i, j + len)
                }
                b'R' => {
                    j += len;
                    (i, j - len, i, j)
                }
                _ => panic!("Invalid direction: '{}'", dir as char),
            });
        }
        Lagoon { edges }
    }

    fn lava_at_row(&self, i: i64) -> i64 {
        let mut e: Vec<_> = self
            .edges
            .iter()
            .filter(|e| e.0 <= i && e.2 >= i)
            .map(|&e| e)
            .collect();
        e.sort_by_key(|e| (e.1, e.3));
        let mut sum = 0;
        let mut inside = false;
        let mut k = 0;
        while k < e.len() {
            assert_eq!(e[k].1, e[k].3); // Vertical line
            let (u, d) = (e[k].0, e[k].2);
            if k < e.len() - 1 && e[k + 1].1 < e[k + 1].3 {
                sum += e[k + 1].3 - e[k + 1].1;
                k += 2; // Skip horizontal line (should be at most one)
            }
            assert_eq!(e[k].1, e[k].3); // Vertical line
            let (u, d) = (e[k].0.min(u), e[k].2.max(d));
            if u < i && i < d {
                inside = !inside;
            }
            sum += 1;
            k += 1;
            if inside {
                sum += e[k].1 - e[k - 1].1 - 1; // Add inside empty space
            }
        }
        sum
    }

    fn lava(&self) -> i64 {
        let rows: HashSet<_> = self
            .edges
            .iter()
            .flat_map(|e| vec![e.0 - 1, e.0, e.0 + 1, e.2 - 1, e.2, e.2 + 1])
            .collect();
        let mut rows: Vec<_> = rows.into_iter().collect();
        rows.sort();
        rows.push(rows.last().unwrap() + 1);
        rows.windows(2)
            .map(|r| (r[1] - r[0]) * self.lava_at_row(r[0]))
            .sum()
    }
}

pub fn part1(path: &str) {
    println!("{}", Lagoon::read(path, false).lava());
}

pub fn part2(path: &str) {
    println!("{}", Lagoon::read(path, true).lava());
}
