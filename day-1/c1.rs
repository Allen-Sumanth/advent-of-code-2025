#![allow(unused_imports)]
use std::*;
use std::io::{BufWriter, Write, Read, stdin, stdout};

#[allow(dead_code)]
struct Scanner {
    iterator: str::SplitAsciiWhitespace<'static>,
    buffer: String,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        let mut buffer = String::new();
        io::Read::read_to_string(&mut io::stdin(), &mut buffer).unwrap();

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

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    // let t: usize = scan.next();
    let mut count0 = 0;
    let mut cur = 50;
    while scan.has_next() {
        let dir: String = scan.next();
        let num:i32 = dir[1..].parse().unwrap();
        match dir.chars().next() {
            Some('L') => {
                cur = ((cur-num)%100+100)%100;
            }
            Some('R') => {
                cur = (cur+num)%100;
            }
            _ => {}
        }
        if cur == 0 {
            count0 += 1;
        }
    }
    writeln!(out, "{}", count0).ok();
}
