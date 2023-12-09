use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Input {
    instructions: String,
    map: HashMap<String, (String, String)>,
}

impl Input {
    fn read(path: &str) -> Input {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();
        let instructions = lines.next().unwrap().unwrap();
        let map = lines
            .skip(1) // Skip the newline
            .filter_map(Result::ok)
            .map(|line| {
                (
                    line[0..3].to_string(),
                    (line[7..10].to_string(), line[12..15].to_string()),
                )
            })
            .collect();
        Input { instructions, map }
    }
}

struct GhostCycle {
    end_pos: i64,
    loop_len: i64,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        // b == 0 * a + 1 * b
        (b, 0, 1)
    } else {
        // g == x * (b % a) + y * a
        // g == x * (b - (b / a) * a) + y * a
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - x * (b / a), x)
    }
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (g, x, _) = extended_gcd(a, m);
    assert_eq!(g, 1);
    (x % m + m) % m
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> i64 {
    let n: i64 = modulii.iter().product();
    let mut sum = 0;
    for (&r, &m) in residues.iter().zip(modulii) {
        let p = n / m;
        let m_inv = mod_inverse(p, m);
        sum += r * m_inv * p;
    }
    (sum % n + n) % n
}

pub fn part1(path: &str) {
    let input = Input::read(path);
    let n = input.instructions.len();
    let mut node = &String::from("AAA");
    let mut steps = 0;
    while node != "ZZZ" {
        let &(ref l, ref r) = input.map.get(node).unwrap();
        let b = input.instructions.as_bytes()[steps % n];
        node = if b == b'L' { l } else { r };
        steps += 1;
    }
    println!("{steps}");
}

pub fn part2(path: &str) {
    let input = Input::read(path);
    let n = input.instructions.len() as i64;
    let mut ghost_cycles = vec![];
    for mut node in input.map.keys().filter(|&node| node.as_bytes()[2] == b'A') {
        let mut end_pos = 0i64;
        let loop_len;
        let mut steps = 0i64;
        let mut visited: HashMap<(&String, i64), i64> = HashMap::new();
        loop {
            let state = (node, steps % n);
            if node.as_bytes()[2] == b'Z' {
                // There is exactly one end state inside the cycle.
                assert_eq!(end_pos, 0);
                end_pos = steps;
            }
            if visited.contains_key(&state) {
                loop_len = steps - visited[&state];
                break;
            }

            visited.insert(state, steps);
            let &(ref l, ref r) = input.map.get(node).unwrap();
            let b = input.instructions.as_bytes()[(steps % n) as usize];
            node = if b == b'L' { l } else { r };
            steps += 1;
        }
        ghost_cycles.push(GhostCycle { end_pos, loop_len })
    }
    // The ghost cycle lenghts share the same GCD.
    let ghost_gcd = gcd(ghost_cycles[0].loop_len, ghost_cycles[1].loop_len);
    // The end state positions yield the same reminder when divided by the shared GCD.
    let ghost_rem = ghost_cycles[0].end_pos % ghost_gcd;
    // Move by ghost_offset steps into the cycle so that the resulting position is divisible by the shared GCD.
    let ghost_offset = ghost_gcd - ghost_rem;
    // Solve using Chinese Reminder Theorem.
    // Dividing by the shared GCD will lead to pairwise coprime cycle lenghts.
    let residues: Vec<_> = ghost_cycles
        .iter()
        .map(|c| (c.end_pos - ghost_offset) / ghost_gcd)
        .collect();
    let modulii: Vec<_> = ghost_cycles
        .iter()
        .map(|c| c.loop_len / ghost_gcd)
        .collect();
    let steps = ghost_offset + ghost_gcd * chinese_remainder(&residues, &modulii);
    println!("{steps}");
}
