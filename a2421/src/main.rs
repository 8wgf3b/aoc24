use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}
const DIRPAD: [&[u8]; 2] = [b" ^A", b"<v>"];
const NUMPAD: [&[u8]; 4] = [b"789", b"456", b"123", b" 0A"];

type Bk = HashMap<(u8, u8), HashSet<Vec<u8>>>;
type Cbk = HashMap<(u8, u8, u8, bool), i64>;

fn getpath(f: u8, t: u8, bk: &mut Bk, l: &[&[u8]]) -> Vec<Vec<u8>> {
    let res = if let Some(v) = bk.get(&(f, t)) {
        v.clone()
    } else {
        let (m, n) = (l.len() as i64, l[0].len() as i64);
        let bc = |p: i64, q: i64| (p >= 0 && p < m) && (q >= 0 && q < n);
        let dir = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        let i2u = |x: i64| x as usize;
        let mut q = VecDeque::new();
        q.push_back((vec![], HashSet::new(), f));
        let mut tflag = false;
        while !q.is_empty() {
            let count = q.len();
            for _ in 0..count {
                if let Some((path, mut h, cc)) = q.pop_front() {
                    let (i, j) = fi(cc, l).unwrap();
                    if !bc(i, j) || h.contains(&cc) || cc == b' ' {
                        continue;
                    }
                    if cc == t {
                        tflag = true;
                        bk.entry((f, cc)).or_default().insert(path.clone());
                    }
                    h.insert(cc);
                    for (ni, nj, dc) in
                        dir.into_iter().zip(b"<^>v").filter_map(|((di, dj), &dc)| {
                            bc(i + di, j + dj).then_some((i + di, j + dj, dc))
                        })
                    {
                        let mut np = path.clone();
                        np.push(dc);
                        q.push_back((np, h.clone(), l[i2u(ni)][i2u(nj)]));
                    }
                }
            }
            if tflag {
                break;
            }
        }
        //println!("{} {} {}", f as char, t as char, l.len());
        bk[&(f, t)].clone()
    };
    res.into_iter()
        .map(|mut v| {
            v.push(b'A');
            v
        })
        .collect()
}

fn fi(x: u8, l: &[&[u8]]) -> Option<(i64, i64)> {
    for (i, v) in l.iter().enumerate() {
        for (j, &c) in v.iter().enumerate() {
            if c == x {
                return Some((i as i64, j as i64));
            }
        }
    }
    None
}

fn _printo(v: &[Vec<u8>]) -> Vec<String> {
    v.iter()
        .map(|x| x.iter().map(|&y| y as char).collect())
        .collect()
}

fn moves(f: u8, t: u8, lvl: u8, isnp: bool, cbk: &mut Cbk, bk: &mut Bk) -> i64 {
    if let Some(&ans) = cbk.get(&(f, t, lvl, isnp)) {
        return ans;
    }
    if lvl == 0 {
        return *cbk.entry((f, t, lvl, isnp)).or_insert(
            getpath(f, t, bk, &DIRPAD)
                .into_iter()
                .map(|x| x.len() as i64)
                .min()
                .unwrap_or(0),
        );
    }
    let paths = if isnp {
        getpath(f, t, bk, &NUMPAD)
    } else {
        getpath(f, t, bk, &DIRPAD)
    };
    let mut curr = b'A';
    let mut minc = i64::MAX;
    for path in paths {
        let mut cost = 0;
        for b in path.into_iter() {
            cost += moves(curr, b, lvl - 1, false, cbk, bk);
            curr = b;
        }
        minc = minc.min(cost)
    }
    *cbk.entry((f, t, lvl, isnp)).or_insert(minc)
}

fn bigmoves(l: &[u8], lvl: u8, cbk: &mut Cbk, bk: &mut Bk) -> i64 {
    let mut curr = b'A';
    l.iter().fold(0, |a, &b| {
        let res = a + moves(curr, b, lvl, true, cbk, bk);
        curr = b;
        res
    })
}

fn _epoch(out: &[u8], bk: &mut Bk, map: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut curr = b'A';
    let mut coly = vec![];
    for &b in out.iter() {
        coly.push(getpath(curr, b, bk, map));
        //println!("{n} {} {} {:?}", curr as char, b as char, printo(&p));
        curr = b;
    }
    coly.into_iter()
        .map(|v| v.into_iter())
        .multi_cartesian_product()
        .map(|v| {
            v.into_iter().fold(vec![], |mut a, x| {
                a.extend_from_slice(&x);
                a
            })
        })
        .collect()
}

fn part1(contents: &str) {
    let mut bk = HashMap::new();
    let mut cbk = HashMap::new();
    let mut ss = 0;
    for l in contents.lines() {
        let n = l[0..l.len() - 1].parse::<i64>().unwrap();
        ss += n * bigmoves(l.as_bytes(), 2, &mut cbk, &mut bk);
        //tp.into_iter().for_each(|v| println!("{l} {:?}", printo(&v)));
        //println!("{:?}", _printo(&tp));
    }
    /*
    for (&k , v) in bk.iter() {
        let s: Vec<String> = v.iter().map(|x| x.iter().map(|&y| y as char).collect()).collect();
        println!("{} {} {s:?}", k.0 as char, k.1 as char);
    }
    */
    println!("{ss}");
}

fn part2(contents: &str) {
    let mut bk = HashMap::new();
    let mut cbk = HashMap::new();
    let mut ss = 0;
    for l in contents.lines() {
        let n = l[0..l.len() - 1].parse::<usize>().unwrap();
        ss += n * bigmoves(l.as_bytes(), 25, &mut cbk, &mut bk) as usize;
    }
    println!("{ss}");
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
