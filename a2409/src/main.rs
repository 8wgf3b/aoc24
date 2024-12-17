use clap::Parser;
use std::iter::repeat_n as rn;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

fn part1(contents: &str) {
    let mut disk = vec![];
    let b2u = |x: u8| (x - b'0') as usize;
    let mut di = 0;
    let mut end = 0;
    let contents = contents.trim();
    for (i, b) in contents.bytes().enumerate() {
        let m = if i % 2 == 0 {
            di += 1;
            di - 1
        } else {
            -1
        };
        disk.extend(rn(m, b2u(b)));
    }
    let n = disk.len();
    let mut e = n - 1;
    for b in 0..n {
        if b >= e {
            end = e;
            break;
        }
        while disk[e] == -1 {
            e -= 1;
        }
        if disk[b] == -1 {
            disk[b] = disk[e];
            disk[e] = -1;
        }
    }
    let res: usize = disk
        .into_iter()
        .enumerate()
        .take(end + 1)
        .map(|(i, v)| i * v as usize)
        .sum();
    println!("{:?}", res);
}

fn part2(contents: &str) {
    let b2u = |x: u8| (x - b'0') as usize;
    let contents = contents.trim();
    let (mut emp, mut files) = (vec![], vec![]);
    let mut di = 0;
    for (i, b) in contents.bytes().enumerate() {
        let m = (di, b2u(b));
        if m.1 == 0 {
            continue;
        }
        if i % 2 == 0 {
            files.push(m)
        } else {
            emp.push(m)
        }
        di += m.1;
    }
    //println!("{:?} {:?}", files, emp);
    for f in files.iter_mut().rev() {
        for e in emp.iter_mut() {
            if e.1 >= f.1 && f.0 > e.0 {
                f.0 = e.0;
                e.1 -= f.1;
                e.0 += f.1;
                break;
            }
        }
    }
    //println!("{files:?}");
    let sf = |x: usize, d: usize| ((x + d - 1) * (x + d) - (x.saturating_sub(1)) * (x)) / 2;
    let res: usize = files
        .into_iter()
        .enumerate()
        .map(|(ai, (si, of))| ai * sf(si, of))
        .sum();
    println!("{:?}", res);
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
