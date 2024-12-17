use clap::Parser;
use std::cmp::Reverse as R;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fs::read_to_string, path::PathBuf};

type Grid = Vec<Vec<u8>>;
type BK = HashMap<((i32, i32), usize), usize>;

fn transform(c: &str) -> Grid {
    c.lines().map(|x| x.bytes().collect::<Vec<_>>()).collect()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn djik(g: &Grid, bk: &mut BK, start: (i32, i32), end: (i32, i32), initdr: Vec<usize>) -> usize {
    let i2u = |x: i32| x as usize;
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |p: i32, q: i32| (p >= 0 && p < m) && (q >= 0 && q < n);
    let dir = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut h = BinaryHeap::new();
    for d in initdr {
        h.push(R((0, start, d)));
    }
    while let Some(R((c, (i, j), dn))) = h.pop() {
        if end == (i, j) {
            bk.entry(((i, j), dn)).or_insert(c);
            return c;
        }
        if !bk.contains_key(&((i, j), dn)) && bc(i, j) && g[i2u(i)][i2u(j)] != b'#' {
            bk.entry(((i, j), dn)).or_insert(c);
            let (di, dj) = dir[dn];
            h.push(R((c + 1, (i + di, j + dj), dn)));
            h.push(R((c + 1000, (i, j), (dn + 1) % 4)));
            h.push(R((c + 1000, (i, j), (dn + 3) % 4)));
        }
    }
    21
}

fn part1(contents: &str) {
    let mut g = transform(contents);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, v) in g.iter_mut().enumerate() {
        for (j, c) in v.iter_mut().enumerate() {
            match *c {
                b'S' => {
                    *c = b'.';
                    start = (i as i32, j as i32)
                }
                b'E' => {
                    *c = b'.';
                    end = (i as i32, j as i32)
                }
                _ => (),
            }
        }
    }
    let res = djik(&g, &mut HashMap::new(), start, end, vec![2]);
    println!("{res}");
}

fn part2(contents: &str) {
    let mut g = transform(contents);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, v) in g.iter_mut().enumerate() {
        for (j, c) in v.iter_mut().enumerate() {
            match *c {
                b'S' => {
                    *c = b'.';
                    start = (i as i32, j as i32)
                }
                b'E' => {
                    *c = b'.';
                    end = (i as i32, j as i32)
                }
                _ => (),
            }
        }
    }
    let mut fwd = HashMap::new();
    let p1 = djik(&g, &mut fwd, start, end, vec![2]);
    let mut bwd = HashMap::new();
    let p2 = djik(&g, &mut bwd, end, start, vec![0, 1, 2, 3]);
    assert_eq!(p1, p2);
    let mut pathele = HashSet::new();
    for ((ind, dn), fv) in fwd {
        if fv + bwd.get(&(ind, (dn + 2) % 4)).map_or(0, |&x| x) == p1 {
            pathele.insert(ind);
        }
    }
    println!("{}", pathele.len())
}

fn main() {
    let cli = Cli::parse();
    let p = cli.input.unwrap_or(PathBuf::from(".\\inputs\\final.txt"));
    let contents = read_to_string(&p).expect("Failed to read the file");
    match cli.part {
        1 => part1(&contents),
        2 => part2(&contents),
        _ => {
            part1(&contents);
            part2(&contents)
        }
    }
}
