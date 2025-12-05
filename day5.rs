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

fn part1() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut ranges: Vec<[i64;2]> = Vec::new();
    let mut ans = 0;
    while scan.has_next() {
        let inp: String = scan.next();
        let mut nums = inp.split('-');

        let num1 = nums.next().unwrap().parse::<i64>().unwrap();
        // nums2 will not exist in the second half of the input
        if let Some(num2) = nums.next() {
            let range = [num1, num2.parse::<i64>().unwrap()];
            ranges.push(range);
        } else {
            for range in ranges.iter() {
                if num1 >= range[0] && num1 <= range[1] {
                    ans += 1;
                    break;
                }
            }
        }
   }

    writeln!(out, "{}", ans).ok();
}

fn part2() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut tot_fresh: i64 = 0;
    let mut ranges = collections::BinaryHeap::new();

    while scan.has_next() {
        let inp: String = scan.next();
        let mut nums = inp.split('-');

        let num1 = nums.next().unwrap().parse::<i64>().unwrap();
        if let Some(num2) = nums.next() {
            let range = [num1, num2.parse::<i64>().unwrap()];
            ranges.push(cmp::Reverse(range));
        } else {break;}
    }

    let (mut p1, mut p2): (i64, i64) = (0,-1);
    while let Some(arr) = ranges.pop() {
        let (l_limit, u_limit) = (arr.0[0], arr.0[1]);

        if l_limit >= p1 && l_limit <= p2 {
            // overlapping interval
            p2 = cmp::max(u_limit, p2);
        } else {
            tot_fresh += p2-p1+1;
            p1 = l_limit;
            p2 = u_limit;
        }
    }
    tot_fresh += p2-p1+1;

    writeln!(out, "{}", tot_fresh).ok();
}

fn main() {

    // let t: usize = scan.next();
    // part1();   
    part2();
    // writeln!(out, "output").ok();
}
