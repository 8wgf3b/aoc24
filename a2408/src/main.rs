use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::PathBuf};

type Grid = Vec<Vec<u8>>;
type Col = (i32, i32);

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn transform(c: &str) -> Grid {
    c.lines().map(|x| x.bytes().collect::<Vec<_>>()).collect()
}

fn build(g: &Grid) -> HashMap<u8, Vec<Col>> {
    let mut h: HashMap<_, Vec<_>> = HashMap::new();
    for (i, v) in g.iter().enumerate() {
        for (j, &vv) in v.iter().enumerate() {
            if vv != b'.' {
                h.entry(vv).or_default().push((i as i32, j as i32));
            }
        }
    }
    h
}

fn anti(mut res: HashSet<Col>, l: Vec<Col>) -> HashSet<Col> {
    let n = l.len();
    for i in 1..n {
        for j in 0..i {
            let (x1, y1, x2, y2) = (l[i].0, l[i].1, l[j].0, l[j].1);
            let (dx, dy) = (x1 - x2, y1 - y2);
            res.insert((x1 + dx, y1 + dy));
            res.insert((x2 - dx, y2 - dy));
            //println!("{:?} {:?} {:?} {:?}", (x1 + dx, y1 + dy), (x1, y1), (x2, y2), (x2 - dx, y2 - dy));
        }
    }
    res
}

fn part1(contents: &str) {
    let g = transform(contents);
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let bc = |&(x, y): &(i32, i32)| (x >= 0 && x < m) && (y >= 0 && y < n);
    let h = build(&g);
    let res = h.into_values().fold(HashSet::new(), anti);
    //println!("{:?}", res);
    let res = res.into_iter().filter(bc).count();
    println!("{:?}", res);
}

fn anti2(mut res: HashSet<Col>, l: Vec<Col>, m: i32, n: i32) -> HashSet<Col> {
    let e = l.len();
    let bc = |(x, y): (i32, i32)| (x >= 0 && x < m) && (y >= 0 && y < n);
    for i in 1..e {
        for j in 0..i {
            let (x1, y1, x2, y2) = (l[i].0, l[i].1, l[j].0, l[j].1);
            let (dx, dy) = (x1 - x2, y1 - y2);
            let mut curr = (x1, y1);
            while bc(curr) {
                //println!("b{:?} {:?} {:?}", curr, (x1, y1), (x2, y2));
                res.insert(curr);
                curr = (curr.0 + dx, curr.1 + dy);
            }
            let mut curr = (x2, y2);
            while bc(curr) {
                //println!("{:?} {:?} {:?}l", (x1, y1), (x2, y2), curr);
                res.insert(curr);
                curr = (curr.0 - dx, curr.1 - dy);
            }
            //println!("{:?} {:?} {:?} {:?}", (x1 + dx, y1 + dy), (x1, y1), (x2, y2), (x2 - dx, y2 - dy));
        }
    }
    //println!();
    res
}

fn part2(contents: &str) {
    let g = transform(contents);
    let (m, n) = (g.len() as i32, g[0].len() as i32);
    let h = build(&g);
    let res = h
        .into_values()
        .fold(HashSet::new(), |a, x| anti2(a, x, m, n));
    //println!("{:?}", res);
    println!("{:?}", res.len());
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
