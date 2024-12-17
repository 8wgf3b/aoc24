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
    let mut res = 0;
    for line in contents.lines() {
        let s: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        res += safech(&s) as i32;
    }
    println!("{res}");
}

fn safech(v: &[i32]) -> bool {
    let ch = |x: i32| x.abs() >= 1 && x.abs() <= 3;
    let n: usize = v.len();
    let p = v[0] - v[1];
    ch(p)
        && (1..n - 1).all(|x| {
            let d = v[x] - v[x + 1];
            d.signum() == p.signum() && ch(d)
        })
}

fn part2(contents: &str) {
    let mut res = 0;
    for line in contents.lines() {
        let s: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        if safech(&s) {
            res += 1;
        } else {
            let n = s.len();
            for i in 0..n {
                let mut v = s[0..i].to_vec();
                v.extend_from_slice(&s[i + 1..n]);
                if safech(&v) {
                    res += 1;
                    break;
                }
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
