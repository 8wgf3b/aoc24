use clap::Parser;
use std::collections::HashMap;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn part1(contents: &str) {
    let (mut l, mut r) = (vec![], vec![]);
    for line in contents.lines() {
        let nums: Vec<_> = line
            .split("   ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        l.push(nums[0]);
        r.push(nums[1]);
    }
    l.sort_unstable();
    r.sort_unstable();
    let res = l
        .into_iter()
        .zip(r)
        .map(|(p, q)| p.abs_diff(q))
        .sum::<u32>();
    println!("{res}");
}

fn part2(contents: &str) {
    let (mut l, mut r) = (HashMap::new(), HashMap::new());
    for line in contents.lines() {
        let nums: Vec<_> = line
            .split("   ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        l.entry(nums[0]).and_modify(|v| *v += 1).or_insert(1);
        r.entry(nums[1]).and_modify(|v| *v += 1).or_insert(1);
    }
    let res = l.keys().map(|&k| k * *r.get(&k).unwrap_or(&0)).sum::<u32>();
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
