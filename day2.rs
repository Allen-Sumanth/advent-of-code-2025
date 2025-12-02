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
fn prob1() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let ranges: String = scan.next();
    let ranges_arr: Vec<&str> = ranges.split(',').collect();
    let mut ans: i64 = 0;
    for range in ranges_arr.iter() {
        let range_arr: Vec<&str> = range.split('-').collect();

        let mut pat_num: i64; // will store first half of each pattern
        let (l_lim, u_lim) = (range_arr[0].parse::<i64>().unwrap(), range_arr[1].parse::<i64>().unwrap());

        let mut dig_count:u32 = range_arr[0].len().try_into().unwrap();
        // dig_count holds the length of the lower limit now

        if dig_count%2 == 1 {
            dig_count = (dig_count+1)/2;
            pat_num = i64::pow(10, dig_count-1);
        } else {
            dig_count /= 2;
            pat_num = l_lim/(i64::pow(10, dig_count));
        }

        //dig_count holds the length of the pat_num now
        loop {
            let pattern = pat_num + pat_num*i64::pow(10, dig_count);

            if pattern > u_lim {break;}
            if pattern >= l_lim {ans += pattern;}

            pat_num+= 1;
            if i64::ilog10(pat_num) >= dig_count {dig_count+=1;}
        }

    }
    writeln!(out, "{}", ans).ok();

}

fn prob2() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let ranges: String = scan.next();
    let ranges_arr: Vec<&str> = ranges.split(',').collect();

    let mut ids = collections::HashSet::new();
    let mut ans: i64 = 0;

    for range in ranges_arr.iter() {
        let range_arr: Vec<&str> = range.split('-').collect();

        let mut pat_num: String; // will store first half of each pattern
        let (l_lim, u_lim) = (range_arr[0].parse::<i64>().unwrap(), range_arr[1].parse::<i64>().unwrap());

        for rep in 2..10 {
            let lower_dig_count = range_arr[0].len(); //number of digits in lower limit
            if lower_dig_count.is_multiple_of(rep) {
                pat_num = range_arr[0][0..(lower_dig_count/rep)].to_string();
            } else {
                pat_num = i64::pow(10, (lower_dig_count.div_ceil(rep)-1).try_into().unwrap()).to_string();
            }

            loop {
                let pattern = pat_num.repeat(rep).parse::<i64>().unwrap();
                if pattern > u_lim {break;}
                if pattern >= l_lim && !ids.contains(&pattern) {
                    ids.insert(pattern);
                    ans += pattern;
                }
                pat_num = (pat_num.parse::<i64>().unwrap()+1).to_string(); //literally hurts my eye
            }
        }
    }
    writeln!(out, "{}", ans).ok();

}

fn main() {
       // prob1();
       prob2();

}
