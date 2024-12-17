use clap::Parser;
use std::collections::{HashSet, VecDeque};
//use std::io;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

type Grid = Vec<Vec<u8>>;

fn transform(c: &str) -> Grid {
    c.lines().map(|x| x.bytes().collect::<Vec<_>>()).collect()
}

fn sim(mut g: Grid, inst: char, curr: &mut (i32, i32)) -> Grid {
    //println!("{curr:?} {inst}");
    //printgrid(&g);
    let (di, dj) = match inst {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        _ => (1, 0),
    };
    //println!("{:?}", (di, dj));
    let i2u = |x: i32| x as usize;
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |p: i32, q: i32| (p >= 0 && p < m) && (q >= 0 && q < n);
    let mut fin = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(*curr);
    let mut go = true;
    while let Some((i, j)) = q.pop_front() {
        if bc(i, j) && g[i2u(i)][i2u(j)] != b'#' && fin.insert((i, j)) {
            let (ni, nj) = (i + di, j + dj);
            let ele = g[i2u(ni)][i2u(nj)];
            match ele {
                b'#' => {
                    go = false;
                    break;
                }
                b'O' => q.push_back((ni, nj)),
                b'[' => {
                    q.push_back((ni, nj));
                    q.push_back((ni, nj + 1))
                }
                b']' => {
                    q.push_back((ni, nj));
                    q.push_back((ni, nj - 1))
                }
                _ => (),
            }
        }
    }
    //println!("{go} {fin:?}");
    if go {
        *curr = (curr.0 + di, curr.1 + dj);
        while !fin.is_empty() {
            for (i, j) in fin.clone().into_iter() {
                let (ni, nj) = (i + di, j + dj);
                if !fin.contains(&(ni, nj)) {
                    fin.remove(&(i, j));
                    g[i2u(ni)][i2u(nj)] = g[i2u(i)][i2u(j)];
                    g[i2u(i)][i2u(j)] = b'.';
                }
            }
        }
    }
    //printgrid(&g);
    //let mut so = String::new();
    //io::stdin().read_line(&mut so).expect("Failed to read line");
    g
}

fn _printgrid(g: &Grid) {
    let s = g.iter().fold(String::new(), |a, x| {
        a + core::str::from_utf8(x).unwrap() + "\n"
    });
    println!("{s}");
}

fn score(g: &Grid) -> usize {
    let mut res = 0;
    for (i, v) in g.iter().enumerate() {
        for (j, &c) in v.iter().enumerate() {
            if c == b'[' || c == b'O' {
                res += i * 100 + j;
            }
        }
    }
    res
}

fn part1(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let mut g = transform(s1);
    let inst = s2.lines().fold(String::new(), |a, x| a + x);
    let mut curr = (0, 0);
    for (i, v) in g.iter_mut().enumerate() {
        for (j, c) in v.iter_mut().enumerate() {
            if *c == b'@' {
                curr = (i as i32, j as i32);
                *c = b'.';
                break;
            }
        }
    }
    g = inst.chars().fold(g, |a, x| sim(a, x, &mut curr));
    //printgrid(&g);
    println!("{}", score(&g));
}

fn part2(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let og = transform(s1);
    let inst = s2.lines().fold(String::new(), |a, x| a + x);
    let mut curr = (0, 0);
    let mut g = vec![];
    for v in og {
        let mut t = vec![];
        for c in v {
            match c {
                b'O' => {
                    t.push(b'[');
                    t.push(b']')
                }
                b'@' => {
                    t.push(b'@');
                    t.push(b'.')
                }
                x => (0..2).for_each(|_| t.push(x)),
            }
        }
        g.push(t)
    }
    for (i, v) in g.iter_mut().enumerate() {
        for (j, c) in v.iter_mut().enumerate() {
            if *c == b'@' {
                curr = (i as i32, j as i32);
                *c = b'.';
                break;
            }
        }
    }
    g = inst.chars().fold(g, |a, x| sim(a, x, &mut curr));
    //printgrid(&g);
    println!("{}", score(&g));
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
