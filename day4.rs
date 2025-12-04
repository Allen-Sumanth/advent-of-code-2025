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

    const ROW_SIZE: usize = 139;

    let mut ans = 0;
    let (mut prev, mut cur) = ([-1; ROW_SIZE], [-1; ROW_SIZE]);
    while scan.has_next() {
        let inp: String = scan.next();
        let inp_arr: Vec<char> = inp.chars().collect();
        
        for i in 0..ROW_SIZE {
            if inp_arr[i] == '.' {continue;}

            cur[i] = 0;

            if i > 0 && inp_arr[i-1] == '@' {
                cur[i] += 1;
            }
            if i < ROW_SIZE-1 && inp_arr[i+1] == '@' {
                cur[i] += 1;
            }
            if prev[i] != -1 {
                cur[i] += 1;
                prev[i] += 1;
            }
            if i > 0 && prev[i-1] != -1 {
                cur[i] += 1;
                prev[i-1] += 1;
            }
            if i < ROW_SIZE-1 && prev[i+1] != -1 {
                cur[i] += 1;
                prev[i+1] += 1;
            }
        }
       
        for neighbours in prev.iter() {
            if *neighbours > -1 && *neighbours < 4 { ans += 1; }
        }
        prev = cur; cur = [-1; ROW_SIZE];
    }
    for neighbours in prev.iter() {
        if *neighbours > -1 && *neighbours < 4 { ans += 1; }
    }


    writeln!(out, "{}", ans).ok();
}

fn part2 () {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    const ROW_SIZE: usize = 139;

    let mut ans = 0;
    
    let mut mat: [[i32; ROW_SIZE]; ROW_SIZE+1] = [[-1; ROW_SIZE]; ROW_SIZE+1]; // extra row to
                                                                               // buffer for row -1
    let mut rown: usize = 1;

    while scan.has_next() {
        let inp: String = scan.next();
        let inp_arr: Vec<char> = inp.chars().collect();
        for i in 0..ROW_SIZE {
            if inp_arr[i] == '.' {continue;}

            mat[rown][i] = 0;

            if i > 0 && inp_arr[i-1] == '@' {
                mat[rown][i] += 1;
            }
            if i < ROW_SIZE-1 && inp_arr[i+1] == '@' {
                mat[rown][i] += 1;
            }
            if mat[rown-1][i] != -1 {
                mat[rown][i] += 1;
                mat[rown-1][i] += 1;
            }
            if i > 0 && mat[rown-1][i-1] != -1 {
                mat[rown][i] += 1;
                mat[rown-1][i-1] += 1;
            }
            if i < ROW_SIZE-1 && mat[rown-1][i+1] != -1 {
                mat[rown][i] += 1;
                mat[rown-1][i+1] += 1;
            }
        }
        rown += 1;
    }

    for i in 1..ROW_SIZE+1 {
        for j in 0..ROW_SIZE {
            ans += cleanup(&mut mat, i, j);
        }
    }

    fn cleanup (matrix: &mut[[i32; ROW_SIZE]; ROW_SIZE+1] , row_num: usize, col_num: usize) -> i32 {
        if matrix[row_num][col_num] < 0 || matrix[row_num][col_num] > 3 {return 0;}

        let mut removed = 1; // we're removing the current cell
        matrix[row_num][col_num] = -1;
        let (r, c): (i64, i64) = (row_num.try_into().unwrap(), col_num.try_into().unwrap());
        for rown in r-1..=r+1 {
            for coln in c-1..=c+1 {
                if rown < 0 || rown > ROW_SIZE as i64 || coln < 0 || coln >= ROW_SIZE as i64 {continue;}

                matrix[rown as usize][coln as usize] -= 1;
                removed += cleanup(matrix, rown as usize, coln as usize);
            }
        }

        removed
    }

    writeln!(out, "{}", ans).ok();
    
}

fn main() {

    // let t: usize = scan.next();
   // part1(); 
   part2();
    // writeln!(out, "output").ok();
}   
