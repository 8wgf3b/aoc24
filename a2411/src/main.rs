use clap::Parser;
use std::collections::HashMap;
use std::{fs::read_to_string, path::PathBuf};

type Bk = HashMap<usize, usize>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn rule(x: usize) -> Vec<usize> {
    let ecc = |x: usize| x.to_string().len() % 2 == 0;
    let spl = |x: usize| {
        let mut w = x.to_string();
        let w1 = w.split_off(w.len() / 2);
        [w1, w].into_iter()
    };
    match x {
        0 => vec![1],
        n if ecc(n) => spl(n).map(|s| s.parse::<usize>().unwrap()).collect(),
        n => vec![n * 2024],
    }
}

fn blink(nums: Bk, cache: &mut HashMap<usize, Vec<usize>>) -> Bk {
    let mut t = HashMap::new();
    for (k, v) in nums {
        let nk = match cache.get(&k) {
            Some(vv) => vv,
            None => cache.entry(k).or_insert(rule(k)),
        };
        nk.iter().for_each(|&x| {
            t.entry(x).and_modify(|p| *p += v).or_insert(v);
        })
    }
    t
}

fn part1(contents: &str) {
    let mut nums: Bk = contents
        .trim()
        .split(' ')
        .map(|x| (x.parse().unwrap(), 1))
        .collect();
    let mut cache = HashMap::new();
    nums = (0..25).fold(nums, |a, _| blink(a, &mut cache));
    let res: usize = nums.into_values().sum();
    println!("{res}");
}

fn part2(contents: &str) {
    let mut nums: Bk = contents
        .trim()
        .split(' ')
        .map(|x| (x.parse().unwrap(), 1))
        .collect();
    let mut cache = HashMap::new();
    nums = (0..75).fold(nums, |a, _| blink(a, &mut cache));
    let res: usize = nums.into_values().sum();
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
