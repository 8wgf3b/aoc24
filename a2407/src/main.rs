use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn sim1(tar: u64, l: &[u64], i: usize, curr: u64) -> bool {
    if tar < curr || (tar > curr && i == l.len()) {
        return false;
    }
    if tar == curr && i == l.len() {
        return true;
    }
    sim1(tar, l, i + 1, curr + l[i]) || sim1(tar, l, i + 1, curr * l[i])
}

fn sim2(tar: u64, l: &[u64], i: usize, curr: u64) -> bool {
    if tar < curr || (tar > curr && i == l.len()) {
        return false;
    }
    if tar == curr && i == l.len() {
        return true;
    }
    let concat = |x: u64, y: u64| (x.to_string() + &y.to_string()).parse::<u64>().unwrap();
    sim2(tar, l, i + 1, curr + l[i])
        || sim2(tar, l, i + 1, curr * l[i])
        || sim2(tar, l, i + 1, concat(curr, l[i]))
}

fn part1(contents: &str) {
    let mut res = 0;
    for line in contents.lines() {
        let (n1, n2) = line.split_once(": ").unwrap();
        let n1 = n1.parse::<u64>().unwrap();
        let l: Vec<_> = n2.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
        res += n1 * (sim1(n1, &l, 0, 0)) as u64;
    }
    println!("{res}");
}

fn part2(contents: &str) {
    let mut res = 0;
    for line in contents.lines() {
        let (n1, n2) = line.split_once(": ").unwrap();
        let n1 = n1.parse::<u64>().unwrap();
        let l: Vec<_> = n2.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
        res += n1 * (sim2(n1, &l, 0, 0)) as u64;
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
