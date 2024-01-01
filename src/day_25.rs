use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

struct Graph {
    mat: Vec<Vec<i32>>,
    n: usize,
}

impl Graph {
    fn read(path: &str) -> Graph {
        let conns: Vec<Vec<String>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| {
                line.split(' ')
                    .enumerate()
                    .map(|(i, s)| {
                        if i == 0 {
                            String::from(&s[0..s.len() - 1])
                        } else {
                            String::from(s)
                        }
                    })
                    .collect()
            })
            .collect();
        let mut nodes: Vec<&String> =
            Vec::from_iter(HashSet::<&String>::from_iter(conns.iter().flatten()));
        nodes.sort();
        let n = nodes.len();
        let node_index: HashMap<&String, usize> =
            HashMap::from_iter(nodes.iter().enumerate().map(|(i, &s)| (s, i)));
        let mut mat = vec![vec![0; n]; n];
        for conn in conns.iter() {
            let i = node_index[&conn[0]];
            for k in 1..conn.len() {
                let j = node_index[&conn[k]];
                mat[i][j] += 1;
                mat[j][i] += 1;
            }
        }
        Graph { mat, n }
    }

    fn stoer_wagner(&self, expected_cut: i32) -> (i32, Vec<usize>) {
        let mut out = (i32::MAX, vec![]);
        let mut mat = self.mat.clone();
        let mut grp = vec![Vec::with_capacity(self.n); self.n];
        for i in 0..self.n {
            grp[i].push(i);
        }
        for phase in 1..self.n {
            let mut w = mat[0].clone(); // Keeps w(A, i) for all vertices i.
            let mut s = 0;
            let mut t = 0;
            for _ in 0..self.n - phase {
                // Add t to A. Make sure t is never selected again.
                w[t] = i32::MIN;
                s = t;
                // Find t (outside A) with maximum w(A, t).
                t = w
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, v)| v)
                    .map(|(i, _)| i)
                    .unwrap();
                // Update w(A, i) for all vertices i (as t is being added to A).
                for i in 0..self.n {
                    w[i] += mat[t][i];
                }
            }
            // In the last phase we added mat[t][t] to w[t].
            if w[t] - mat[t][t] < out.0 {
                out = (w[t] - mat[t][t], grp[t].clone());
            }
            // Merge s and t.
            let mut grp_t = grp[t].clone();
            grp[s].append(&mut grp_t);
            for i in 0..self.n {
                mat[s][i] += mat[t][i];
            }
            for i in 0..self.n {
                mat[i][s] = mat[s][i];
            }
            // Make sure t is never used again.
            mat[0][t] = i32::MIN;
            if out.0 == expected_cut {
                break;
            }
        }
        out
    }
}

pub fn part1(path: &str) {
    let graph = Graph::read(path);
    let (_, cut) = graph.stoer_wagner(3);
    println!("{}", cut.len() * (graph.n - cut.len()));
}
