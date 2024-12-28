use clap::Parser;
use std::collections::{HashMap, VecDeque};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn part1(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let mut h = HashMap::new();
    for l in s1.lines() {
        let (p1, p2) = l.split_once(": ").unwrap();
        h.entry(p1.to_owned())
            .or_insert(p2.parse::<usize>().unwrap());
    }
    let op = |p, q, o| match o {
        "AND" => p & q,
        "OR" => p | q,
        "XOR" => p ^ q,
        _ => 0,
    };
    //println!("{h:?}");
    let mut inst: VecDeque<Vec<_>> = s2.lines().map(|l| l.split(' ').collect()).collect();
    while let Some(v) = inst.pop_front() {
        if let (Some(&b1), Some(&b2)) = (h.get(v[0]), h.get(v[2])) {
            let bres = op(b1, b2, v[1]);
            h.entry(v[4].to_owned())
                .and_modify(|p| *p = bres)
                .or_insert(bres);
        } else {
            inst.push_back(v)
        }
    }
    let res: usize = h
        .into_iter()
        .filter_map(|(k, v)| {
            k.starts_with("z").then(|| {
                let p = k[1..].parse::<u32>().unwrap();
                v * 2usize.pow(p)
            })
        })
        .sum();
    println!("{res}");
}

fn part2(_contents: &str) {
    // manual swaps. Used colab, networkx, graphviz, matplotlib
    // tst <-> z05
    // sps <-> z11
    // frt <-> z23
    // cgh <-> pmd
    println!("cgh,frt,pmd,sps,tst,z05,z11,z23");
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
