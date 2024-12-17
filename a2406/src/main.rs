use clap::Parser;
use std::collections::HashSet;
use std::iter::repeat_n as rn;
use std::{fs::read_to_string, path::PathBuf};

type Grid = Vec<Vec<u8>>;

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

fn _gp(g: &Grid) -> String {
    g.iter()
        .flat_map(|v| v.iter().map(|&x| x as char).chain(rn('\n', 1)))
        .collect()
}

fn sim1(g: &mut Grid, mut curr: (i32, i32)) -> usize {
    let mut dir = [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().cycle();
    let mut cd = dir.next().unwrap();
    let i2u = |x: i32| x as usize;
    while let Some(c) = g
        .get_mut(i2u(curr.0 + cd.0))
        .and_then(|v| v.get_mut(i2u(curr.1 + cd.1)))
    {
        match *c {
            b'#' => cd = dir.next().unwrap(),
            _ => {
                *c = b'X';
                curr = (curr.0 + cd.0, curr.1 + cd.1)
            }
        }
    }
    g[i2u(curr.0)][i2u(curr.1)] = b'X';
    //println!("{}", gp(g));

    g.iter()
        .flat_map(|v| v.iter())
        .filter(|&&x| x == b'X')
        .count()
}

fn getpath(mut g: Grid, mut curr: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut dir = [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().cycle();
    let mut cd = dir.next().unwrap();
    let i2u = |x: i32| x as usize;
    let mut res = HashSet::new();
    while let Some(c) = g
        .get_mut(i2u(curr.0 + cd.0))
        .and_then(|v| v.get_mut(i2u(curr.1 + cd.1)))
    {
        match *c {
            b'#' => cd = dir.next().unwrap(),
            _ => {
                *c = b'X';
                curr = (curr.0 + cd.0, curr.1 + cd.1);
                res.insert(curr);
            }
        }
    }
    g[i2u(curr.0)][i2u(curr.1)] = b'X';
    //println!("{}", gp(g));
    res
}

fn part1(contents: &str) {
    let mut grid = transform(contents);
    let mut curr = (0, 0);
    for (i, g) in grid.iter_mut().enumerate() {
        for (j, v) in g.iter_mut().enumerate() {
            if *v == b'^' {
                curr = (i as i32, j as i32);
                *v = b'.';
            }
        }
    }
    let res = sim1(&mut grid, curr);
    println!("{res}");
}

fn sim2(g: &Grid, mut curr: (i32, i32)) -> bool {
    let mut dir = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .into_iter()
        .cycle()
        .peekable();
    let mut cd = dir.next().unwrap();
    let i2u = |x: i32| x as usize;
    let mut bk = HashSet::new();
    while let Some(c) = g
        .get(i2u(curr.0 + cd.0))
        .and_then(|v| v.get(i2u(curr.1 + cd.1)))
    {
        if *c == b'#' {
            cd = dir.next().unwrap();
            continue;
        }
        if !bk.insert((curr, cd)) {
            return true;
        }
        curr = (curr.0 + cd.0, curr.1 + cd.1)
    }
    false
}

fn part2(contents: &str) {
    let mut grid = transform(contents);
    let mut curr = (0, 0);
    for (i, g) in grid.iter_mut().enumerate() {
        for (j, v) in g.iter_mut().enumerate() {
            if *v == b'^' {
                curr = (i as i32, j as i32);
                *v = b'.';
            }
        }
    }
    let mut posbl = getpath(grid.clone(), curr);
    posbl.remove(&curr);
    let res = posbl
        .into_iter()
        .filter(|&x| {
            grid[x.0 as usize][x.1 as usize] = b'#';
            let res = sim2(&grid, curr);
            grid[x.0 as usize][x.1 as usize] = b'.';
            res
        })
        .count();
    println!("{:?}", res);
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
