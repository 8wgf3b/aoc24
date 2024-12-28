use clap::Parser;
use std::collections::HashSet;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn posi(w: &str, base: &HashSet<String>) -> u64 {
    let n = w.len();
    let mut dp = vec![0; n];
    for i in 0..n {
        dp[i] += base.contains(&w[0..=i]) as u64;
        for j in 0..i {
            dp[i] += dp[j] * base.contains(&w[j + 1..=i]) as u64
        }
    }
    dp[n - 1]
}

fn part1(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let base = s1.split(", ").map(|x| x.to_owned()).collect();
    let res = s2.lines().filter(|w| posi(w, &base) > 0).count();
    println!("{res}");
}

fn part2(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let base = s1.split(", ").map(|x| x.to_owned()).collect();
    let res: u64 = s2.lines().map(|w| posi(w, &base)).sum();
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
