use std::fs::File;
use std::io::{self, BufRead};

struct Row {
    con: Vec<u8>,
    grp: Vec<usize>,
}

impl Row {
    fn parse(s: &str, unfold: usize) -> Row {
        let parts: Vec<_> = s.split(' ').collect();
        assert_eq!(parts.len(), 2);
        let mut con_str = String::from("");
        let mut grp_str = String::from("");
        for i in 0..unfold {
            con_str.push_str(parts[0]);
            grp_str.push_str(parts[1]);
            if i < unfold - 1 {
                con_str.push('?');
                grp_str.push(',');
            }
        }
        con_str.push('.');
        let con: Vec<_> = con_str.bytes().collect();
        let grp = grp_str.split(',').map(|g| g.parse().unwrap()).collect();
        Row { con, grp }
    }

    fn solve(&self) -> i64 {
        let mut cache = vec![vec![-1i64; self.grp.len()]; self.con.len()];
        self.solve_part(&mut cache, 0, 0)
    }

    fn solve_part(&self, cache: &mut Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
        if j == self.grp.len() {
            if self.con[i..].iter().all(|x| [b'.', b'?'].contains(x)) {
                return 1;
            } else {
                return 0;
            };
        }
        let sum = cache[i][j];
        if sum >= 0 {
            return sum;
        }
        let suff_len: usize = self.grp[j..].iter().map(|&g| g + 1).sum();
        let g = self.grp[j];
        let mut sum = 0;
        for k in i..=self.con.len() - suff_len {
            if self.con[k..k + g].iter().all(|x| [b'#', b'?'].contains(x))
                && self.con[k + g] != b'#'
            {
                sum += self.solve_part(cache, k + g + 1, j + 1);
            }
            if self.con[k] == b'#' {
                break;
            }
        }
        cache[i][j] = sum;
        sum
    }
}

fn solve(path: &str, unfold: usize) {
    let sum: i64 = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(Result::ok)
        .map(|line| Row::parse(&line, unfold).solve())
        .sum();
    println!("{sum}");
}

pub fn part1(path: &str) {
    solve(path, 1);
}

pub fn part2(path: &str) {
    solve(path, 5);
}
