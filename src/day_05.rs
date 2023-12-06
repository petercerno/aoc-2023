use std::fs::File;
use std::io::{self, BufRead};

struct Range {
    dest_start: u64,
    src_start: u64,
    src_len: u64,
}

impl Range {
    fn parse(s: &str) -> Range {
        let nums = parse_numbers(s);
        assert_eq!(nums.len(), 3);
        Range {
            dest_start: nums[0],
            src_start: nums[1],
            src_len: nums[2],
        }
    }
}

struct Map {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Clone)]
struct Interval {
    start: u64,
    end: u64, // Exclusive
}

impl Map {
    fn upper_bound(&self, x: u64, hint: usize) -> usize {
        let mut left = hint;
        let mut right = self.ranges.len();
        while left < right {
            let mid = (left + right) / 2;
            if self.ranges[mid].src_start > x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn apply(&self, x: u64) -> u64 {
        let i = self.upper_bound(x, 0);
        if i > 0 {
            let range = &self.ranges[i - 1];
            if x >= range.src_start && x < range.src_start + range.src_len {
                range.dest_start + x - range.src_start
            } else {
                x
            }
        } else {
            x
        }
    }

    fn apply_interval(&self, int: &Interval) -> Vec<Interval> {
        let mut out = vec![];
        let mut x = int.start;
        let mut i = 0usize;
        while x < int.end {
            i = self.upper_bound(x, i);
            if i > 0 {
                let range = &self.ranges[i - 1];
                if x >= range.src_start && x < range.src_start + range.src_len {
                    if int.end <= range.src_start + range.src_len {
                        out.push(Interval {
                            start: range.dest_start + x - range.src_start,
                            end: range.dest_start + int.end - range.src_start,
                        });
                        x = int.end;
                    } else {
                        out.push(Interval {
                            start: range.dest_start + x - range.src_start,
                            end: range.dest_start + range.src_len,
                        });
                        x = range.src_start + range.src_len;
                    }
                } else if i < self.ranges.len() {
                    let range = &self.ranges[i];
                    out.push(Interval {
                        start: x,
                        end: range.src_start.min(int.end),
                    });
                    x = range.src_start;
                } else {
                    out.push(Interval {
                        start: x,
                        end: int.end,
                    });
                    x = int.end;
                }
            } else {
                let range = &self.ranges[i];
                out.push(Interval {
                    start: x,
                    end: range.src_start.min(int.end),
                });
                x = range.src_start;
            }
        }
        out.sort_by_key(|int| int.start);
        out
    }

    fn apply_intervals(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut out = intervals
            .iter()
            .map(|int| self.apply_interval(int))
            .collect::<Vec<_>>()
            .concat();
        out.sort_by_key(|int| int.start);
        out
    }
}

struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Input {
    fn read(path: &str) -> Input {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();
        let seeds = lines.next().unwrap().unwrap();
        assert!(seeds.starts_with("seeds: "));
        let seeds = parse_numbers(&seeds[7..]);
        lines.next();
        let mut maps = vec![];
        loop {
            let mut map = Map {
                name: String::from(""),
                ranges: vec![],
            };
            loop {
                let line = lines.next();
                if line.is_none() {
                    break; // End of input
                }

                let line = line.unwrap().unwrap();
                if line.trim().is_empty() {
                    break; // Empty line marking the start of a new block
                }

                if map.name.is_empty() {
                    map.name = line.strip_suffix(" map:").unwrap().to_string();
                    continue;
                }

                map.ranges.push(Range::parse(&line));
            }
            if map.ranges.is_empty() {
                break;
            }

            map.ranges.sort_by_key(|range| range.src_start);
            maps.push(map);
        }
        Input { seeds, maps }
    }
}

fn parse_numbers(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part1(path: &str) {
    let input = Input::read(path);
    let min_location = input
        .seeds
        .iter()
        .map(|&x| input.maps.iter().fold(x, |acc, map| map.apply(acc)))
        .min()
        .unwrap();
    println!("{min_location}");
}

pub fn part2(path: &str) {
    let input = Input::read(path);
    let initial_intervals = (0usize..input.seeds.len() / 2)
        .map(|i| Interval {
            start: input.seeds[2 * i],
            end: input.seeds[2 * i] + input.seeds[2 * i + 1],
        })
        .collect::<Vec<_>>();
    let final_intervals = input
        .maps
        .iter()
        .fold(initial_intervals, |acc, map| map.apply_intervals(acc));
    println!("{}", final_intervals[0].start);
}
