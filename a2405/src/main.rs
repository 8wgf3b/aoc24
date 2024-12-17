use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

type Adj = HashMap<u32, HashSet<u32>>;

fn is_child(adj: &Adj, a: u32, b: u32) -> bool {
    if let Some(v) = adj.get(&a) {
        v.contains(&b)
    } else {
        false
    }
}

fn part1(contents: &str) {
    let mut adj: Adj = HashMap::new();
    let ci: Vec<_> = contents.lines().collect();
    let mut b = 0;
    let n = ci.len();
    for (i, c) in ci.iter().enumerate() {
        if ci[i].is_empty() {
            b = i;
            break;
        }
        let nums: Vec<_> = c.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
        adj.entry(nums[0]).or_default().insert(nums[1]);
    }
    //println!("{b}");
    let lis: Vec<Vec<u32>> = (1 + b..n)
        .map(|x| {
            ci[x]
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let res: u32 = lis
        .into_iter()
        .filter_map(|v| {
            v.is_sorted_by(|&a, &b| is_child(&adj, a, b))
                .then_some(v[v.len() / 2])
        })
        .sum();
    println!("{res}");
}

fn part2(contents: &str) {
    let mut adj: Adj = HashMap::new();
    let ci: Vec<_> = contents.lines().collect();
    let n = ci.len();
    let mut b = 0;
    for (i, c) in ci.iter().enumerate() {
        if ci[i].is_empty() {
            b = i;
            break;
        }
        let nums: Vec<_> = c.split('|').map(|x| x.parse::<u32>().unwrap()).collect();
        adj.entry(nums[0]).or_default().insert(nums[1]);
    }
    //println!("{b}");
    let lis: Vec<Vec<u32>> = (1 + b..n)
        .map(|x| {
            ci[x]
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let cres: u32 = lis
        .iter()
        .filter_map(|v| {
            v.is_sorted_by(|&a, &b| is_child(&adj, a, b))
                .then_some(v[v.len() / 2])
        })
        .sum();

    let res: u32 = lis
        .into_iter()
        .map(|mut v| {
            v.sort_by(|&a, &b| is_child(&adj, a, b).cmp(&true));
            //println!("{v:?}");
            v[v.len() / 2]
        })
        .sum();
    println!("{}", res - cres);
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
