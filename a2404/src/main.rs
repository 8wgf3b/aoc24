use clap::Parser;
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

fn chexmas(grid: &Grid, i: i32, j: i32) -> i32 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let pat = [b'X', b'M', b'A', b'S'];
    let dir = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut res = 0;
    for (di, dj) in dir {
        res += ((0..4)
            .filter(|&p| {
                let (ni, nj) = (i + p * di, j + p * dj);
                (ni >= 0 && ni < m)
                    && (nj >= 0 && nj < n)
                    && (grid[ni as usize][nj as usize] == pat[p as usize])
            })
            .count()
            == 4) as i32;
    }
    res
}

fn chexmas2(grid: &Grid, i: i32, j: i32) -> i32 {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let dir1 = [(1, 1), (0, 0), (-1, -1)];
    let dir2 = [(-1, 1), (0, 0), (1, -1)];
    let bc = |ni: i32, nj: i32| (ni >= 0 && ni < m) && (nj >= 0 && nj < n);
    let b2u = |p: u8| (p - b'A') as usize;
    let cc =
        |count: [u8; 26]| count[b2u(b'A')] == 1 && count[b2u(b'M')] == 1 && count[b2u(b'S')] == 1;
    let cm = |dir: [(i32, i32); 3], i: i32, j: i32| {
        let mut count = [0; 26];
        for (di, dj) in dir {
            let (ni, nj) = (i + di, j + dj);
            if bc(ni, nj) {
                count[b2u(grid[ni as usize][nj as usize])] += 1;
            }
        }
        cc(count)
    };
    if grid[i as usize][j as usize] == b'A' {
        (cm(dir1, i, j) && cm(dir2, i, j)) as i32
    } else {
        0
    }
}

fn part1(contents: &str) {
    let grid = transform(contents);
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res += chexmas(&grid, i as i32, j as i32)
        }
    }
    println!("{res}");
}

fn part2(contents: &str) {
    let grid = transform(contents);
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res += chexmas2(&grid, i as i32, j as i32)
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
