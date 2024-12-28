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

fn gensec(mut x: usize, count: usize) -> Vec<usize> {
    let prune = |x| x % 16777216;
    let mix = |x, y| x ^ y;
    (0..count)
        .map(|_| {
            x = prune(mix(x, 64 * x));
            x = prune(mix(x, x / 32));
            x = prune(mix(x, x * 2048));
            x
        })
        .collect()
}

fn part1(contents: &str) {
    let res: usize = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| gensec(x, 2000)[1999])
        .sum();
    println!("{res}")
}

fn eventgen(num: usize) -> HashMap<Vec<i64>, usize> {
    let v: Vec<usize> = gensec(num, 2000).into_iter().map(|x| x % 10).collect();
    let mut h = HashMap::new();
    let n = v.len();
    let deltas: Vec<i64> = (1..n).map(|i| v[i] as i64 - v[i - 1] as i64).collect();
    for (vv, &n) in deltas.windows(4).zip(v.iter().skip(4)) {
        h.entry(vv.to_owned()).or_insert(n);
    }
    h
}

fn part2(contents: &str) {
    let nums: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    //let score  = HashMap::new();
    let score = nums.into_iter().fold(HashMap::new(), |mut a, x| {
        for (k, v) in eventgen(x) {
            a.entry(k).and_modify(|p| *p += v).or_insert(v);
        }
        a
    });
    println!("{}", score.values().max().unwrap());
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
