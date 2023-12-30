use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;
use std::fs::File;
use std::io::{self, BufRead};

type Coord = [f64; 3];

struct Hailstone {
    p: Coord,
    v: Coord,
}

fn parse_coord(s: &str) -> Coord {
    s.split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl Hailstone {
    fn parse(s: &str) -> Hailstone {
        let mut pv = s.split(" @ ");
        Hailstone {
            p: parse_coord(pv.next().unwrap()),
            v: parse_coord(pv.next().unwrap()),
        }
    }
}

fn read_hailstones(path: &str) -> Vec<Hailstone> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|line| Hailstone::parse(&line))
        .collect()
}

fn intersect_xy(h1: &Hailstone, h2: &Hailstone, min: f64, max: f64) -> bool {
    let a: Array2<f64> = array![[h1.v[0], -h2.v[0]], [h1.v[1], -h2.v[1]]];
    let b: Array1<f64> = array![h2.p[0] - h1.p[0], h2.p[1] - h1.p[1]];
    if let Ok(t) = a.solve_into(b) {
        let (x, y) = (h1.p[0] + h1.v[0] * t[0], h1.p[1] + h1.v[1] * t[0]);
        t[0] > 0.0 && t[1] > 0.0 && min <= x && x <= max && min <= y && y <= max
    } else {
        false
    }
}

fn solve_rock(h: &Vec<Hailstone>) {
    let mut r = Hailstone {
        p: [0.0; 3],
        v: [0.0; 3],
    };
    let max_vel = 1000i32;
    let mut w0w1s: Vec<_> = (-max_vel..=max_vel)
        .flat_map(|v0| (-max_vel..=max_vel).map(move |v1| (v0, v1)))
        .collect();
    w0w1s.sort_by_key(|&(w0, w1)| (w0 * w0 + w1 * w1, w0, w1));
    for &(w0, w1) in w0w1s.iter() {
        r.v[0] = w0 as f64;
        r.v[1] = w1 as f64;
        let a: Array2<f64> = array![
            [1.0, 0.0, r.v[0] - h[0].v[0], 0.0, 0.0],
            [0.0, 1.0, r.v[1] - h[0].v[1], 0.0, 0.0],
            [1.0, 0.0, 0.0, r.v[0] - h[1].v[0], 0.0],
            [0.0, 1.0, 0.0, r.v[1] - h[1].v[1], 0.0],
            [1.0, 0.0, 0.0, 0.0, r.v[0] - h[2].v[0]],
        ];
        let b: Array1<f64> = array![h[0].p[0], h[0].p[1], h[1].p[0], h[1].p[1], h[2].p[0]];
        if let Ok(s) = a.solve_into(b) {
            if (s[1].round() + (r.v[1] - h[2].v[1]) * s[4].round() - h[2].p[1]).abs() < 1e-6 {
                r.p[0] = s[0].round();
                r.p[1] = s[1].round();
                break;
            }
        }
    }
    let mut w2s: Vec<_> = (-max_vel..=max_vel).collect();
    w2s.sort_by_key(|&w2| w2.abs());
    for &w2 in w2s.iter() {
        r.v[2] = w2 as f64;
        let a: Array2<f64> = array![
            [1.0, 0.0, r.v[0] - h[0].v[0], 0.0, 0.0],
            [0.0, 1.0, r.v[2] - h[0].v[2], 0.0, 0.0],
            [1.0, 0.0, 0.0, r.v[0] - h[1].v[0], 0.0],
            [0.0, 1.0, 0.0, r.v[2] - h[1].v[2], 0.0],
            [1.0, 0.0, 0.0, 0.0, r.v[0] - h[2].v[0]],
        ];
        let b: Array1<f64> = array![h[0].p[0], h[0].p[2], h[1].p[0], h[1].p[2], h[2].p[0]];
        if let Ok(s) = a.solve_into(b) {
            if (s[1].round() + (r.v[2] - h[2].v[2]) * s[4].round() - h[2].p[2]).abs() < 1e-6 {
                r.p[2] = s[1].round();
                break;
            }
        }
    }
    println!("{}", r.p[0] + r.p[1] + r.p[2]);
}

pub fn part1(path: &str) {
    let h = read_hailstones(path);
    let n = h.len();
    let count: usize = (0..n - 1)
        .map(|i| {
            (i + 1..n)
                .filter(|&j| intersect_xy(&h[i], &h[j], 2e14, 4e14))
                .count()
        })
        .sum();
    println!("{count}");
}

pub fn part2(path: &str) {
    solve_rock(&read_hailstones(path));
}
