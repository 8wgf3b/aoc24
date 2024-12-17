use clap::Parser;
use regex::Regex;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn mulp(s: &str) -> i32 {
    (s[4..s.len() - 1])
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .product()
}

fn part1(contents: &str) {
    let pat = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let res: i32 = pat.find_iter(contents).map(|x| mulp(x.as_str())).sum();
    println!("{res}");
}

fn part2(contents: &str) {
    let pat = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut run = true;
    let res = pat.find_iter(contents).fold(0, |a, x| {
        a + match x.as_str() {
            "do()" => {
                run = true;
                0
            }
            "don't()" => {
                run = false;
                0
            }
            s => mulp(s) * run as i32,
        }
    });
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
