use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Condition {
    cat: usize,
    cmp: Ordering,
    val: usize,
}

impl Condition {
    fn parse(c: &str) -> Condition {
        let b = c.as_bytes();
        let cat = match b[0] {
            b'x' => 0,
            b'm' => 1,
            b'a' => 2,
            b's' => 3,
            _ => panic!("Invalid category: '{}'", b[0] as char),
        };
        let cmp = match b[1] {
            b'<' => Ordering::Less,
            b'>' => Ordering::Greater,
            _ => panic!("Invalid ordering: '{}'", b[1] as char),
        };
        let val = c[2..].parse().unwrap();
        Condition { cat, cmp, val }
    }
}

enum Outcome {
    Redirect(String),
    Accept,
    Reject,
}

impl Outcome {
    fn parse(o: &str) -> Outcome {
        match o {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            other => Outcome::Redirect(String::from(other)),
        }
    }
}

struct Rule {
    con: Option<Condition>,
    out: Outcome,
}

impl Rule {
    fn parse(r: &str) -> Rule {
        if let Some(i) = r.find(':') {
            Rule {
                con: Some(Condition::parse(&r[..i])),
                out: Outcome::parse(&r[i + 1..]),
            }
        } else {
            Rule {
                con: None,
                out: Outcome::parse(r),
            }
        }
    }

    fn matches(&self, p: &Part) -> bool {
        if let Some(con) = self.con.as_ref() {
            match con.cmp {
                Ordering::Less => p.0[con.cat] < con.val,
                Ordering::Greater => p.0[con.cat] > con.val,
                _ => panic!("Invalid condition"),
            }
        } else {
            true
        }
    }
}

struct Rules(Vec<Rule>);

impl Rules {
    fn parse(r: &str) -> Rules {
        Rules(
            r[1..r.len() - 1]
                .split(',')
                .map(|r| Rule::parse(r))
                .collect(),
        )
    }

    fn run(&self, p: &Part) -> &Outcome {
        for rule in self.0.iter() {
            if rule.matches(p) {
                return &rule.out;
            }
        }
        panic!("No matching rule found");
    }
}

struct Workflows(HashMap<String, Rules>);

impl Workflows {
    fn new() -> Workflows {
        Workflows(HashMap::new())
    }

    fn add(&mut self, w: &str) {
        let i = w.find('{').unwrap();
        self.0.insert(String::from(&w[..i]), Rules::parse(&w[i..]));
    }

    fn run(&self, p: &Part) -> &Outcome {
        let mut cur = "in";
        loop {
            let out = self.0[cur].run(p);
            if let Outcome::Redirect(next) = out {
                cur = next;
            } else {
                return out;
            }
        }
    }

    fn solve(&self, cur: &str, ins: Intervals) -> u64 {
        let mut ins = ins;
        let mut sum = 0;
        for rule in &self.0[cur].0 {
            if ins.volume() == 0 {
                break;
            }
            match &rule.out {
                Outcome::Redirect(next) => {
                    if let Some(con) = rule.con.as_ref() {
                        sum += self.solve(&next, ins.restrict(con, false));
                        ins = ins.restrict(con, true);
                    } else {
                        sum += self.solve(&next, ins);
                    }
                }
                Outcome::Accept => {
                    if let Some(con) = rule.con.as_ref() {
                        sum += ins.restrict(con, false).volume();
                        ins = ins.restrict(con, true);
                    } else {
                        sum += ins.volume();
                    }
                }
                Outcome::Reject => {
                    if let Some(con) = rule.con.as_ref() {
                        ins = ins.restrict(con, true);
                    }
                }
            }
        }
        sum
    }
}

struct Part([usize; 4]);

impl Part {
    fn parse(p: &str) -> Part {
        let cats: Vec<_> = p[1..p.len() - 1].split(',').collect();
        assert_eq!(cats.len(), 4);
        Part([
            cats[0][2..].parse().unwrap(),
            cats[1][2..].parse().unwrap(),
            cats[2][2..].parse().unwrap(),
            cats[3][2..].parse().unwrap(),
        ])
    }
}

#[derive(Clone, Copy)]
struct Interval(usize, usize);

impl Interval {
    fn new() -> Interval {
        Interval(1, 4001)
    }

    fn len(&self) -> usize {
        self.1 - self.0
    }

    fn restrict(&self, cmp: Ordering, val: usize, inverted: bool) -> Interval {
        if inverted {
            match cmp {
                Ordering::Less => Interval(self.0.max(val).min(self.1), self.1),
                Ordering::Greater => Interval(self.0, self.1.min(val + 1).max(self.0)),
                _ => panic!("Invalid condition"),
            }
        } else {
            match cmp {
                Ordering::Less => Interval(self.0, self.1.min(val).max(self.0)),
                Ordering::Greater => Interval(self.0.max(val + 1).min(self.1), self.1),
                _ => panic!("Invalid condition"),
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Intervals([Interval; 4]);

impl Intervals {
    fn new() -> Intervals {
        Intervals([
            Interval::new(),
            Interval::new(),
            Interval::new(),
            Interval::new(),
        ])
    }

    fn volume(&self) -> u64 {
        self.0.iter().map(|int| int.len() as u64).product()
    }

    fn restrict(&self, con: &Condition, inverted: bool) -> Intervals {
        let mut out = self.clone();
        out.0[con.cat] = out.0[con.cat].restrict(con.cmp, con.val, inverted);
        out
    }
}

pub fn part1(path: &str) {
    let mut lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok);
    let mut ws = Workflows::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        ws.add(&line);
    }
    let ps: Vec<_> = lines.map(|line| Part::parse(&line)).collect();
    let sum = ps
        .iter()
        .filter(|&p| matches!(ws.run(p), Outcome::Accept))
        .map(|p| p.0.iter().sum::<usize>())
        .sum::<usize>();
    println!("{sum}");
}

pub fn part2(path: &str) {
    let mut lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok);
    let mut ws = Workflows::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        ws.add(&line);
    }
    let sum = ws.solve("in", Intervals::new());
    println!("{sum}");
}
