use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

type XY = [usize; 2];
type XYZ = [usize; 3];
type Brick = [XYZ; 2];

fn parse_xyz(s: &str) -> XYZ {
    s.split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn parse_brick(s: &str) -> Brick {
    let mut parts = s.split('~');
    [
        parse_xyz(parts.next().unwrap()),
        parse_xyz(parts.next().unwrap()),
    ]
}

fn get_brick_xy(b: &Brick) -> Vec<XY> {
    if b[0][0] < b[1][0] {
        (b[0][0]..=b[1][0]).map(|x| [x, b[0][1]]).collect()
    } else if b[0][1] < b[1][1] {
        (b[0][1]..=b[1][1]).map(|y| [b[0][0], y]).collect()
    } else {
        vec![[b[0][0], b[0][1]]]
    }
}

fn get_brick_graph(path: &str) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut bricks: Vec<Brick> = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|l| parse_brick(&l))
        .collect();
    bricks.sort_by_key(|b| b[0][2]);
    let mut above = vec![HashSet::new(); bricks.len()];
    let mut below = vec![HashSet::new(); bricks.len()];
    let (max_x, max_y) = bricks
        .iter()
        .map(|b| (b[1][0], b[1][1]))
        .fold((0, 0), |acc, (a, b)| (acc.0.max(a), acc.1.max(b)));
    let mut height = vec![vec![0usize; max_x + 1]; max_y + 1];
    let mut index = vec![vec![0usize; max_x + 1]; max_y + 1];
    for (i, b) in bricks.iter().enumerate() {
        let brick_xy = get_brick_xy(b);
        let max_height = brick_xy.iter().map(|&[x, y]| height[y][x]).max().unwrap();
        for &[x, y] in brick_xy.iter() {
            if max_height > 0 && height[y][x] == max_height {
                above[index[y][x]].insert(i);
                below[i].insert(index[y][x]);
            }
            height[y][x] = max_height + 1 + b[1][2] - b[0][2];
            index[y][x] = i;
        }
    }
    (above, below)
}

pub fn part1(path: &str) {
    let (above, below) = get_brick_graph(path);
    let count = above
        .iter()
        .filter(|&a| a.iter().all(|&j| below[j].len() > 1))
        .count();
    println!("{count}");
}

pub fn part2(path: &str) {
    let (above, below) = get_brick_graph(path);
    let sum: usize =
        (0..above.len())
            .map(|i| {
                let mut falls = HashSet::new();
                let mut queue = VecDeque::from(vec![i]);
                falls.insert(i);
                while let Some(i) = queue.pop_front() {
                    queue.extend(above[i].iter().filter(|&&j| {
                        below[j].iter().all(|k| falls.contains(k)) && falls.insert(j)
                    }));
                }
                falls.len() - 1
            })
            .sum();
    println!("{sum}");
}
