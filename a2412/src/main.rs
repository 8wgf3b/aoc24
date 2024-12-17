use clap::Parser;
use std::collections::{HashSet, VecDeque};
use std::{fs::read_to_string, path::PathBuf};

type Grid = Vec<Vec<u8>>;
type Coord = (i32, i32);

fn transform(c: &str) -> Grid {
    //print!("{c}");
    c.lines().map(|x| x.bytes().collect::<Vec<_>>()).collect()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn bfs(g: &Grid, vis: &mut HashSet<Coord>, i: i32, j: i32) -> usize {
    let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let i2u = |x: i32| x as usize;
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |p: i32, q: i32| (p >= 0 && p < m) && (q >= 0 && q < n);
    let mut q = VecDeque::new();
    let ele = g[i2u(i)][i2u(j)];
    q.push_back((i, j));
    let mut ap = (0, 0);
    while let Some((r, c)) = q.pop_front() {
        if vis.insert((r, c)) {
            let it = dir.into_iter().filter_map(|(p, q)| {
                let (ni, nj) = (r + p, c + q);
                (bc(ni, nj) && g[i2u(ni)][i2u(nj)] == ele).then_some((ni, nj))
            });
            ap.0 += 1;
            ap.1 += 4 - it.clone().count();
            q.extend(it);
        }
    }
    ap.0 * ap.1
}

fn cornernei(g: &Grid, i: i32, j: i32) -> (usize, Vec<(i32, i32)>) {
    let dir: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let i2u = |x: i32| x as usize;
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |(p, q): (i32, i32)| (p >= 0 && p < m) && (q >= 0 && q < n);
    let ele = g[i2u(i)][i2u(j)];
    let ec = |(p, q): (i32, i32)| g[i2u(p)][i2u(q)] == ele;
    let mut co = 0;
    let mut nei = vec![];
    for dd in 0..4 {
        let (d1, d2) = (dir[dd], dir[(dd + 1) % 4]);
        let n1 = (i + d1.0, j + d1.1);
        let n2 = (i + d2.0, j + d2.1);
        let n3 = (i + d1.0 + d2.0, j + d1.1 + d2.1);
        if bc(n1) && ec(n1) {
            nei.push(n1)
        }
        co += !((bc(n1) && ec(n1)) || (bc(n2) && ec(n2))) as usize
            + ((bc(n1) && ec(n1)) && (bc(n2) && ec(n2)) && !(bc(n3) && ec(n3))) as usize;
    }
    (co, nei)
}

fn bfs2(g: &Grid, vis: &mut HashSet<Coord>, i: i32, j: i32) -> usize {
    let mut q = VecDeque::new();
    q.push_back((i, j));
    let mut ap = (0, 0);
    while let Some((r, c)) = q.pop_front() {
        if vis.insert((r, c)) {
            let (co, nei) = cornernei(g, r, c);
            ap.0 += 1;
            ap.1 += co;
            q.extend(nei.into_iter());
        }
    }
    ap.0 * ap.1
}

fn part1(contents: &str) {
    let g = transform(contents);
    //println!("{g:?}");
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let mut res = 0;
    let mut vis = HashSet::new();
    for i in 0..m {
        for j in 0..n {
            res += bfs(&g, &mut vis, i, j);
        }
    }
    println!("{res}");
}

fn part2(contents: &str) {
    let g = transform(contents);
    //println!("{g:?}");
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let mut res = 0;
    let mut vis = HashSet::new();
    for i in 0..m {
        for j in 0..n {
            res += bfs2(&g, &mut vis, i, j);
        }
    }
    println!("{res}");
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
