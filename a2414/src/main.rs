use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn part1(contents: &str) {
    let mut p = vec![];
    let mut v = vec![];
    for l in contents.lines() {
        let raw: Vec<_> = l
            .split(" ")
            .map(|x| {
                //println!("{x}");
                x[2..]
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        p.push((raw[0][0], raw[0][1]));
        v.push((raw[1][0], raw[1][1]));
    }
    let (m, n) = (101, 103);
    let cv = |p: i32, v: i32, t: i32, b: i32| (b + p + (v * t) % b) % b;
    let mut qc = [0; 4];
    //println!("{p:?} {v:?}");
    for (pp, vv) in p.into_iter().zip(v) {
        //println!("{pp:?} {vv:?}");
        let rp = (cv(pp.0, vv.0, 100, m), cv(pp.1, vv.1, 100, n));
        match rp {
            (x, y) if x < m / 2 && y < n / 2 => qc[0] += 1,
            (x, y) if x < m / 2 && y > n / 2 => qc[1] += 1,
            (x, y) if x > m / 2 && y < n / 2 => qc[2] += 1,
            (x, y) if x > m / 2 && y > n / 2 => qc[3] += 1,
            _ => (),
        }
    }
    //println!("{qc:?}");
    println!("{}", qc.into_iter().product::<i32>());
}

fn part2(contents: &str) {
    let mut p = vec![];
    let mut v = vec![];
    for l in contents.lines() {
        let raw: Vec<_> = l
            .split(" ")
            .map(|x| {
                //println!("{x}");
                x[2..]
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        p.push((raw[0][0], raw[0][1]));
        v.push((raw[1][0], raw[1][1]));
    }
    let (m, n) = (101, 103);
    let cv = |p: i32, v: i32, t: i32, b: i32| (b + p + (v * t) % b) % b;
    //println!("{p:?} {v:?}");
    let lim = 10000;
    let mut res = 0;
    let mut mmax = 0;
    for i in 0..lim {
        let mut g = vec![vec![false; 101]; 103];
        for (pp, vv) in p.iter().zip(&v) {
            //println!("{pp:?} {vv:?}");
            let rp = (cv(pp.0, vv.0, i, m), cv(pp.1, vv.1, i, n));
            g[rp.1 as usize][rp.0 as usize] = true;
        }
        let mut cmax: i32 = 0;
        for c in 0..101 {
            let mut ccmax = 0;
            for v in g.iter().map(|x| x[c]) {
                match v {
                    true => ccmax += 1,
                    _ => {
                        cmax = cmax.max(ccmax);
                        ccmax = 0
                    }
                }
            }
        }
        if mmax < cmax {
            mmax = cmax;
            res = i;
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
