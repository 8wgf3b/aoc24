use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::PathBuf};

type Adj = HashMap<String, HashSet<String>>;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn part1(contents: &str) {
    let mut adj: Adj = HashMap::new();
    for l in contents.lines() {
        let (s1, s2) = l.split_once('-').unwrap();
        adj.entry(s1.to_owned()).or_default().insert(s2.to_owned());
        adj.entry(s2.to_owned()).or_default().insert(s1.to_owned());
    }
    let nodes: Vec<String> = adj.keys().cloned().collect();
    let n = nodes.len();
    let mut res = 0;
    for i in 2..n {
        for j in 1..i {
            for k in 0..j {
                let (a, b, c) = (&nodes[i], &nodes[j], &nodes[k]);
                if adj[a].contains(b)
                    && adj[b].contains(c)
                    && adj[c].contains(a)
                    && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                {
                    res += 1;
                }
            }
        }
    }
    println!("{res}");
}

fn bronkerb(
    adj: &Adj,
    r: &mut HashSet<String>,
    x: &mut HashSet<String>,
    p: &mut HashSet<String>,
    clique: &mut HashSet<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > clique.len() {
            *clique = r.clone()
        };
        return;
    }
    for v in &p.clone() {
        let mut rnew = r.clone();
        rnew.insert(v.clone());
        let mut xnew = x.intersection(&adj[v]).cloned().collect();
        let mut pnew = p.intersection(&adj[v]).cloned().collect();
        bronkerb(adj, &mut rnew, &mut xnew, &mut pnew, clique);
        p.remove(v);
        x.insert(v.clone());
    }
}

fn part2(contents: &str) {
    let mut adj: Adj = HashMap::new();
    for l in contents.lines() {
        let (s1, s2) = l.split_once('-').unwrap();
        adj.entry(s1.to_owned()).or_default().insert(s2.to_owned());
        adj.entry(s2.to_owned()).or_default().insert(s1.to_owned());
    }
    let mut clique = HashSet::new();
    let mut nodes: HashSet<_> = adj.keys().cloned().collect();
    bronkerb(
        &adj,
        &mut HashSet::new(),
        &mut HashSet::new(),
        &mut nodes,
        &mut clique,
    );
    let mut clique: Vec<String> = clique.into_iter().collect();
    clique.sort();
    let res = clique.join(",");
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
