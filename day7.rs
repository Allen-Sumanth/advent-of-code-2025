#![allow(unused_imports)]
use std::*;
use std::io::prelude::*;
use std::io::{BufWriter, stdin, stdout};

#[allow(dead_code)]
struct Scanner {
    iterator: str::SplitAsciiWhitespace<'static>,
    buffer: String,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        let mut buffer = String::new();
        Read::read_to_string(&mut stdin(), &mut buffer).unwrap();

        let slice = unsafe {
            mem::transmute::<&str, &'static str>(buffer.as_str())
        };

        Self {
            buffer,
            iterator: slice.split_ascii_whitespace(),
        }
    }
    fn has_next(&self) -> bool {
        self.iterator.clone().next().is_some()
    }
    fn next<T: str::FromStr>(&mut self) -> T
        where T::Err: fmt::Debug
    {
        self.iterator.next().unwrap().parse().unwrap()
    }
}

fn part1() {
    let buffer: Vec<Vec<char>> = stdin().lines().map(|l| l.unwrap().chars().collect()).collect();
    let out = &mut BufWriter::new(stdout());

    let man_size: usize = buffer[0].len(); // manifold size   
    let mut beams: Vec<bool> = Vec::with_capacity(man_size);
    beams = vec![false;man_size];
    let mut splits = 0;

    for line in buffer.into_iter() {
        for (i, sym) in line.iter().enumerate() {
            if sym == &'.' {continue;}
            if sym == &'S' {beams[i] = true;}
            else if beams[i] {
                splits += 1;
                beams [i] = false;
                if i > 0 {beams[i-1] = true;}
                if i < man_size-1 {beams[i+1] = true;}
            }
        }
    }
    writeln!(out, "{}", splits).ok();
}

fn part2() {
    let buffer: Vec<Vec<char>> = stdin().lines().map(|l| l.unwrap().chars().collect()).collect();
    let out = &mut BufWriter::new(stdout());

    let man_size: usize = buffer[0].len(); // manifold size   
    let mut beams: Vec<i64> = vec![0; man_size]; // no of realities each beam is in 

    for line in buffer.into_iter() {
        for (i, sym) in line.iter().enumerate() {
            if sym == &'.' {continue;}
            if sym == &'S' {beams[i] = 1;}
            else if beams[i] > 0 {
                if i > 0 {beams[i-1] += beams[i];}
                if i < man_size-1 {beams[i+1] += beams[i];}
                beams[i] = 0;
            }
        }
    }

    let ans: i64 = beams.into_iter().sum();
    writeln!(out, "{}", ans).ok();

}

fn main() {

    // let t: usize = scan.next();
    // part1();
    part2();
    // writeln!(out, "output").ok();
}
