use clap::Parser;
use std::collections::{HashSet, VecDeque};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn bfs(corr: &[(i32, i32)], n: i32) -> Option<i32> {
    let bc = |p: i32, q: i32| (p >= 0 && p <= n) && (q >= 0 && q <= n);
    let dir = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut h = VecDeque::new();
    let mut blocks: HashSet<_> = HashSet::from_iter(corr.iter().copied());
    h.push_back((0, (0, 0)));
    while let Some((c, (i, j))) = h.pop_front() {
        if i == n && j == n {
            return Some(c);
        }
        if bc(i, j) && blocks.insert((i, j)) {
            for (di, dj) in dir {
                h.push_back((c + 1, (i + di, j + dj)));
            }
        }
    }
    None
}

fn part1(contents: &str) {
    let cord: Vec<_> = contents
        .lines()
        .map(|s| {
            let v: Vec<_> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            (v[1], v[0])
        })
        .collect();
    let res = bfs(&cord[0..1024], 70).unwrap();
    println!("{res}");
}

fn part2(contents: &str) {
    let cord: Vec<_> = contents
        .lines()
        .map(|s| {
            let v: Vec<_> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            (v[1], v[0])
        })
        .collect();
    let mut l = 1024;
    let mut r = cord.len() - 1;
    while l < r {
        let mid = (l + r) / 2;
        if bfs(&cord[0..=mid], 70).is_some() {
            l = mid + 1;
        } else {
            r = mid
        }
    }
    println!("{},{}", cord[l].1, cord[l].0);
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
