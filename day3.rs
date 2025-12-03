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

fn part1 () {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut ans = 0;

    while scan.has_next() {
        let num: String = scan.next();
        let mut jolts: [i32; 2]= [-1, -1];
        for n in num.chars() {
            let dig:i32 = n.to_digit(10).unwrap().try_into().unwrap();
            if dig > jolts[1] {
                if dig > jolts[0] {
                    jolts[0] = cmp::max(jolts[1], jolts[0]);
                    jolts[1] = dig;
                } else {
                    jolts[1] = dig;
                }
            } else if jolts[0] < jolts[1] {
                jolts[0] = jolts[1];
                jolts[1] = dig;
            }
        }
        ans += jolts[0]*10+jolts[1];
    }
    writeln!(out, "{}",ans).ok();
}

fn part2() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut ans = 0;
    const JOLT_SIZE: i32 = 12;
    
    let mut dig_stack = collections::VecDeque::<i64>::new();

    while scan.has_next(){
        let inp: String = scan.next();
        let digits: Vec<char> = inp.chars().collect();
        let l = digits.len();

        for i in 0..l {
            let dig:i64 = digits[i].to_digit(10).unwrap().into();
            // (number of places remaining in the 12-digit stack, number of digits remaining in the
            // cell)
            let (mut rem_stack, rem_digits) = (JOLT_SIZE-dig_stack.len() as i32, (l-i-1) as i32);
            while  !dig_stack.is_empty() && Some(&dig) > dig_stack.back() && rem_digits>=rem_stack {
                dig_stack.pop_back();
                rem_stack += 1;
            }
            if (dig_stack.len() as i32) < JOLT_SIZE {
                dig_stack.push_back(dig);
            }
        }
        let mut jolts: i64 = 0;
        while !dig_stack.is_empty() {
            let digit = dig_stack.pop_front().unwrap();
            jolts = jolts*10 + digit;
        }
        ans += jolts;
    }
    writeln!(out, "{}", ans).ok();
}

fn main() {
    // part1();
    part2();
}
