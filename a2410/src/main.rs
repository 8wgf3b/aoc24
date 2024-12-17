use clap::Parser;
use std::collections::HashSet;
use std::{fs::read_to_string, path::PathBuf};

type Grid = Vec<Vec<u8>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn transform(c: &str) -> Grid {
    c.lines()
        .map(|x| x.bytes().map(|c| c - b'0').collect::<Vec<_>>())
        .collect()
}

fn trail2(g: &Grid, (i, j): (i32, i32), dp: &mut Vec<Vec<i32>>) -> i32 {
    let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let i2u = |x: i32| x as usize;
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |p: i32, q: i32| (p >= 0 && p < m) && (q >= 0 && q < n);
    //println!("{} {} {}", i, j, g[i2u(i)][i2u(j)]);
    if dp[i2u(i)][i2u(j)] != 0 {
        return dp[i2u(i)][i2u(j)];
    }
    if g[i2u(i)][i2u(j)] == 9 {
        dp[i2u(i)][i2u(j)] = 1;
        return 1;
    }
    let mut res = 0;
    for (di, dj) in dir {
        let (ni, nj) = (i + di, j + dj);
        if bc(ni, nj) && g[i2u(ni)][i2u(nj)] == 1 + g[i2u(i)][i2u(j)] {
            res += trail2(g, (ni, nj), dp);
        }
    }
    dp[i2u(i)][i2u(j)] = res;
    res
}

fn trail1(g: &Grid, i: i32, j: i32, dp: &mut HashSet<(i32, i32)>) -> usize {
    let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let i2u = |x: i32| x as usize;
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |p: i32, q: i32| (p >= 0 && p < m) && (q >= 0 && q < n);
    if g[i2u(i)][i2u(j)] == 9 {
        dp.insert((i, j));
        return 1;
    }
    for (di, dj) in dir {
        let (ni, nj) = (i + di, j + dj);
        if bc(ni, nj) && g[i2u(ni)][i2u(nj)] == 1 + g[i2u(i)][i2u(j)] {
            trail1(g, ni, nj, dp);
        }
    }
    dp.len()
}

fn part1(contents: &str) {
    let g = transform(contents);
    let (m, n) = (g.len(), g[0].len());
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 0 {
                res += trail1(&g, i as i32, j as i32, &mut HashSet::new());
            }
        }
    }
    println!("{res}");
}

fn part2(contents: &str) {
    let g = transform(contents);
    let (m, n) = (g.len(), g[0].len());
    let mut dp = vec![vec![0; m]; n];
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if g[i][j] == 0 {
                res += trail2(&g, (i as i32, j as i32), &mut dp);
            }
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
