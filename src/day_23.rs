use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

struct Map {
    map: Vec<Vec<u8>>,
    m: usize,
    n: usize,
}

struct Graph {
    adj: Vec<Vec<(usize, usize)>>,
    n: usize,
}

impl Map {
    fn read(path: &str) -> Map {
        let map: Vec<Vec<u8>> = io::BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().collect())
            .collect();
        let (m, n) = (map.len(), map[0].len());
        Map { map, m, n }
    }

    fn edges(&self, c: Coord, downhill: bool) -> Vec<Coord> {
        let mut out = Vec::with_capacity(4);
        match self.map[c.0][c.1] {
            b'^' if downhill => out.push(Coord(c.0 - 1, c.1)),
            b'v' if downhill => out.push(Coord(c.0 + 1, c.1)),
            b'<' if downhill => out.push(Coord(c.0, c.1 - 1)),
            b'>' if downhill => out.push(Coord(c.0, c.1 + 1)),
            _ => {
                if c.0 > 0 && self.map[c.0 - 1][c.1] != b'#' {
                    out.push(Coord(c.0 - 1, c.1));
                }
                if c.0 < self.m - 1 && self.map[c.0 + 1][c.1] != b'#' {
                    out.push(Coord(c.0 + 1, c.1));
                }
                if c.1 > 0 && self.map[c.0][c.1 - 1] != b'#' {
                    out.push(Coord(c.0, c.1 - 1));
                }
                if c.1 < self.n - 1 && self.map[c.0][c.1 + 1] != b'#' {
                    out.push(Coord(c.0, c.1 + 1));
                }
            }
        }
        out
    }

    fn graph(&self, downhill: bool) -> Graph {
        let mn = self.m * self.n;
        let mut index = vec![vec![mn; self.n]; self.m];
        let mut nodes = Vec::<Coord>::with_capacity(mn);
        nodes.push(Coord(0, 1));
        for i in 0..self.m {
            for j in 0..self.n {
                let c = Coord(i, j);
                if self.map[i][j] != b'#' && self.edges(c, downhill).len() > 2 {
                    index[i][j] = nodes.len();
                    nodes.push(c);
                }
            }
        }
        index[self.m - 1][self.n - 2] = nodes.len();
        nodes.push(Coord(self.m - 1, self.n - 2));
        let n = nodes.len();
        let mut adj = vec![Vec::with_capacity(4); n];
        for (i, c0) in nodes.iter().enumerate() {
            'outer: for c1 in self.edges(*c0, downhill).iter() {
                let mut c0 = *c0;
                let mut c1 = *c1;
                let mut w = 1;
                while index[c1.0][c1.1] == mn {
                    if let Some(&c2) = self
                        .edges(c1, downhill)
                        .iter()
                        .filter(|&&c2| c2 != c0)
                        .next()
                    {
                        c0 = c1;
                        c1 = c2;
                        w += 1;
                    } else {
                        continue 'outer;
                    }
                }
                let j = index[c1.0][c1.1];
                adj[i].push((j, w));
            }
        }
        Graph { adj, n }
    }
}

impl Graph {
    fn search(&self) -> usize {
        let mut max_len = 0;
        let mut visited = vec![false; self.n];
        let mut edge = vec![0usize; self.n];
        let mut path = Vec::with_capacity(self.n);
        path.push((0, 0));
        'outer: while let Some(&(i0, w0)) = path.last() {
            if i0 == self.n - 1 {
                max_len = max_len.max(w0);
            } else {
                visited[i0] = true;
                while edge[i0] < self.adj[i0].len() {
                    let (i1, w1) = self.adj[i0][edge[i0]];
                    edge[i0] += 1;
                    if !visited[i1] {
                        path.push((i1, w0 + w1));
                        continue 'outer;
                    }
                }
            }
            edge[i0] = 0;
            visited[i0] = false;
            path.pop();
        }
        max_len
    }
}

pub fn part1(path: &str) {
    println!("{}", Map::read(path).graph(true).search());
}

pub fn part2(path: &str) {
    println!("{}", Map::read(path).graph(false).search());
}
