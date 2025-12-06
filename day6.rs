#![allow(unused_imports)]
use std::*;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufWriter};

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
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let mut ans: i64 = 0;
    let mut question: Vec<i64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    while scan.has_next() {
        let inp:String = scan.next();
        if inp.parse::<i64>().is_ok() {
            let number:i64 = inp.parse::<i64>().unwrap();
            question.push(number);
        } else {
            // writeln!(out, "{:?}", question).ok();
            // we've reached the final line
            ops.push(inp.chars().next().unwrap());
        }
    }
    let page_width = ops.len();
    let num_ops = question.len() / page_width;
    for (i, op) in ops.into_iter().enumerate() {
        let mut ans_step = if op == '*' {1} else {0};
        for step_num in 0..num_ops {
            if op == '*' {
                ans_step *= question[i+step_num*page_width];
            } else {
                ans_step += question[i+step_num*page_width];
            }
        }
        ans += ans_step;
    }
    writeln!(out, "{}", ans).ok();

}

fn part2() {
    let buffer: Vec<Vec<char>> = stdin().lines().map(|l| l.unwrap().chars().collect()).collect();
    let out = &mut BufWriter::new(stdout());

    let num_rows: usize = buffer.len()-1;
    let (mut ans, mut ans_step): (i64, i64) = (0, 0);
    let  mut is_mul = false;
    
    for (idx, ptr) in buffer.last().unwrap().iter().enumerate() {
        let mut op_num: i64 = 0;
        for i in 0..num_rows{
            let Some(next_dig) = buffer[i][idx].to_digit(10) else {
                continue;
            };
            op_num = op_num*10+ next_dig as i64;
        }
        if op_num == 0 {continue;}

       if ptr == &'*'|| ptr == &'+' {
          is_mul = ptr == &'*';
          ans += ans_step;            
          ans_step = op_num;
       } else if is_mul {ans_step *= op_num;}
       else {ans_step += op_num;}

    }
    ans += ans_step;
    writeln!(out, "{}", ans).ok();
}

fn main() {

    // let t: usize = scan.next();
   // part1(); 
   part2();
    // writeln!(out, "output").ok();
}
