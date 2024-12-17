use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn solve(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> usize {
    let denom = a.0 * b.1 - b.0 * a.1;
    let at = (c.0 * b.1 - c.1 * b.0) / denom;
    let bt = (c.1 * a.0 - c.0 * a.1) / denom;
    if at.fract() > 0.0 || bt.fract() > 0.0 {
        0
    } else {
        3 * at.floor() as usize + bt.floor() as usize
    }
}

fn part1(contents: &str) {
    let v2t = |v: Vec<f64>| (v[0], v[1]);
    let mut res = 0;
    for a in contents.split("\r\n\r\n") {
        let mut ai = a.lines();
        let va: Vec<f64> = ai.next().unwrap()[12..]
            .split(", Y+")
            .map(|x| x.parse().unwrap())
            .collect();
        let vb: Vec<f64> = ai.next().unwrap()[12..]
            .split(", Y+")
            .map(|x| x.parse().unwrap())
            .collect();
        let vc: Vec<f64> = ai.next().unwrap()[9..]
            .split(", Y=")
            .map(|x| x.parse().unwrap())
            .collect();
        let (va, vb, vc) = (v2t(va), v2t(vb), v2t(vc));
        res += solve(va, vb, vc);
    }
    println!("{res}");
}

fn part2(contents: &str) {
    let v2t = |v: Vec<f64>| (v[0], v[1]);
    let mut res = 0;
    for a in contents.split("\r\n\r\n") {
        let mut ai = a.lines();
        let va: Vec<f64> = ai.next().unwrap()[12..]
            .split(", Y+")
            .map(|x| x.parse().unwrap())
            .collect();
        let vb: Vec<f64> = ai.next().unwrap()[12..]
            .split(", Y+")
            .map(|x| x.parse().unwrap())
            .collect();
        let vc: Vec<f64> = ai.next().unwrap()[9..]
            .split(", Y=")
            .map(|x| 10000000000000.0 + x.parse::<f64>().unwrap())
            .collect();
        let (va, vb, vc) = (v2t(va), v2t(vb), v2t(vc));
        res += solve(va, vb, vc);
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
