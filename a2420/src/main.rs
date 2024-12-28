use clap::Parser;
use std::collections::{HashMap, VecDeque};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn bfs(blocks: &mut HashMap<(i32, i32), i32>, start: (i32, i32), end: (i32, i32), n: i32) {
    let bc = |p: i32, q: i32| (p >= 0 && p < n) && (q >= 0 && q < n);
    let dir = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut q = VecDeque::new();
    q.push_back((0, start));
    while let Some((c, (i, j))) = q.pop_front() {
        if end == (i, j) {
            blocks.entry(end).or_insert(c);
            break;
        }
        if bc(i, j) && !blocks.contains_key(&(i, j)) {
            blocks.entry((i, j)).or_insert(c);
            for (di, dj) in dir {
                q.push_back((c + 1, (i + di, j + dj)));
            }
        }
    }
}

fn cheatcount(blocks: &HashMap<(i32, i32), i32>, cutoff: i32, md: i32) -> i32 {
    let mut count = 0;
    for (&(i, j), &v) in blocks.iter() {
        if v != i32::MAX {
            for di in -md..=md {
                let ymd = md - di.abs();
                for dj in -ymd..=ymd {
                    let (ni, nj) = (i + di, j + dj);
                    if let Some(&v1) = blocks.get(&(ni, nj)).filter(|&&v1| v1 != i32::MAX) {
                        if v1 - v - di.abs() - dj.abs() >= cutoff {
                            count += 1
                        }
                    }
                }
            }
        }
    }
    count
}

fn part1(contents: &str) {
    let mut s = (0, 0);
    let mut e = (0, 0);
    let mut blocks = HashMap::new();
    let mut n = 0;
    for (i, v) in contents.lines().enumerate() {
        for (j, c) in v.chars().enumerate() {
            let cord = (i as i32, j as i32);
            match c {
                'S' => s = cord,
                'E' => e = cord,
                '#' => {
                    blocks.insert(cord, i32::MAX);
                }
                _ => (),
            }
        }
        n = i
    }
    let n = n as i32 + 1;
    bfs(&mut blocks, s, e, n);
    let res = cheatcount(&blocks, 100, 2);
    println!("{res}")
}

fn part2(contents: &str) {
    let mut s = (0, 0);
    let mut e = (0, 0);
    let mut blocks = HashMap::new();
    let mut n = 0;
    for (i, v) in contents.lines().enumerate() {
        for (j, c) in v.chars().enumerate() {
            let cord = (i as i32, j as i32);
            match c {
                'S' => s = cord,
                'E' => e = cord,
                '#' => {
                    blocks.insert(cord, i32::MAX);
                }
                _ => (),
            }
        }
        n = i
    }
    let n = n as i32 + 1;
    bfs(&mut blocks, s, e, n);
    let res = cheatcount(&blocks, 100, 20);
    println!("{res}")
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
