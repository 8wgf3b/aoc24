use clap::Parser;
use std::cmp::Reverse as R;
use std::collections::BinaryHeap;
use std::{fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..3))]
    part: u8,
}

#[derive(Debug)]
struct Cpu {
    reg: [usize; 3],
    out: Vec<usize>,
}

impl Cpu {
    fn new(s: &str) -> Self {
        let mut res = Self {
            reg: [0; 3],
            out: vec![],
        };
        for (i, l) in s.lines().enumerate() {
            //println!("{}", &l[12..]);
            res.reg[i] = l[12..].parse::<usize>().unwrap();
        }
        res
    }

    fn combo(&self, v: usize) -> usize {
        match v {
            n @ 0..=3 => n,
            n => self.reg[n - 4],
        }
    }

    fn adv(&mut self, v: usize) {
        self.reg[0] /= 2usize.pow(self.combo(v).try_into().unwrap())
    }

    fn bdv(&mut self, v: usize) {
        self.reg[1] = self.reg[0] / 2usize.pow(self.combo(v).try_into().unwrap())
    }

    fn cdv(&mut self, v: usize) {
        self.reg[2] = self.reg[0] / 2usize.pow(self.combo(v).try_into().unwrap())
    }

    fn bxl(&mut self, v: usize) {
        self.reg[1] ^= v;
    }

    fn bst(&mut self, v: usize) {
        self.reg[1] = self.combo(v) % 8;
    }

    fn jnz(&self, v: usize) -> Option<usize> {
        match self.reg[0] {
            0 => None,
            _ => Some(v),
        }
    }
    fn bxc(&mut self) {
        self.reg[1] ^= self.reg[2]
    }

    fn out(&mut self, v: usize) {
        self.out.push(self.combo(v) % 8);
    }

    fn run(&mut self, inst: &[usize]) {
        let n = inst.len();
        let mut i = 0;
        while i < n {
            let v = inst[i + 1];
            match inst[i] {
                0 => self.adv(v),
                1 => self.bxl(v),
                2 => self.bst(v),
                3 => {
                    i = self.jnz(v).unwrap_or(i + 2);
                    continue;
                }
                4 => self.bxc(),
                5 => self.out(v),
                6 => self.bdv(v),
                7 => self.cdv(v),
                _ => (),
            }
            i += 2;
        }
    }

    fn mina(&mut self, inst: &[usize]) -> usize {
        let n = inst.len();
        let cut = (0..).position(|i| inst[2 * i] == 3).unwrap_or(usize::MAX);
        //println!("{:?}", &inst[0..2 * cut]);
        let mut h = BinaryHeap::from_iter((0..8).map(|z| R((z, n - 1))));
        let rinst = &inst[0..2 * cut];
        //println!("{rinst:?}");
        while let Some(R((a, i))) = h.pop() {
            let sc = self.simrun(rinst, a, inst[i]);
            //println!("{a} {i} {} {sc}", inst[i]);
            if i == 0 && sc {
                return a;
            } else if sc {
                for da in 0..8 {
                    h.push(R((da + (8 * a), i - 1)))
                }
            }
        }
        42069
    }

    fn simrun(&mut self, inst: &[usize], a: usize, target: usize) -> bool {
        self.reg[0] = a;
        self.out.clear();
        self.run(inst);
        target == self.out.last().map_or(21, |&x| x)
    }

    fn print(&self) {
        let s = self
            .out
            .iter()
            .fold(String::new(), |a, &x| format!("{a},{x}"));
        println!("{}", &s[1..])
    }
}

fn part1(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let mut cpu = Cpu::new(s1);
    let inst: Vec<_> = s2[9..]
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    cpu.run(&inst);
    //println!("{cpu:?}");
    cpu.print();
}

fn part2(contents: &str) {
    let (s1, s2) = contents.split_once("\r\n\r\n").unwrap();
    let mut cpu = Cpu::new(s1);
    let inst: Vec<_> = s2[9..]
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    //cpu.run(inst);
    //println!("{cpu:?}");
    let res = cpu.mina(&inst);
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
