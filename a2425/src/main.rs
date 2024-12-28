use clap::Parser;
use itertools::Itertools;
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
    let mut locks = HashMap::new();
    let mut keys = HashMap::new();
    for pat in contents.split("\r\n\r\n") {
        let mut it = pat.lines();
        let t = match it.next() {
            Some(".....") => &mut keys,
            _ => &mut locks,
        };
        let mut combo = vec![0; 5];
        for l in it.take(5) {
            for (i, c) in l.chars().enumerate() {
                combo[i] += (c == '#') as i32;
            }
        }
        t.entry(combo).and_modify(|p| *p += 1).or_insert(1);
    }
    //println!("{locks:?}");
    //println!("{keys:?}");
    let mut ss: i32 = 0;
    for (k, v) in locks {
        ss += k
            .into_iter()
            .map(|x| 0..=(5 - x))
            .multi_cartesian_product()
            .filter_map(|vv| keys.get(&vv).map(|&x| v * x))
            .sum::<i32>();
    }
    println!("{ss}");
}

fn part2(_contents: &str) {
    println!("part2");
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
