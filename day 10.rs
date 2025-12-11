#![allow(unused_imports)]
use std::*;
use std::io::prelude::*;
use std::io::{BufWriter, stdin, stdout};
use z3::{Solver, ast::{Int}};

/*
CARGO.TOML DEPENDENCIES:
[dependencies]
z3 = {version = "0.19.6", features = ["gh-release"]}
*/

fn part1() {
    let out = &mut BufWriter::new(stdout());

    let buffer: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut ans: u64 = 0;

    for line in buffer.into_iter() {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let l = (parts[0].len() - 2) as u32; // length of light string

        let mut dp: Vec<i16> = vec![i16::MAX;u32::pow(2, l) as usize];
        dp[0] = 0;

        let mut targ: u16 = 0;
        for button in parts[0].chars() {
            if button == '[' || button == ']' {continue;}
            targ <<= 1;
            if button == '#' {targ |= 1;}
        }

        let mut operands:Vec<u16> = Vec::new();
        for part in parts.into_iter().skip(1) {
            if part.starts_with('{') {break;}

            let buttons: Vec<u32> = part[1..part.len()-1].split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            let mut op = 0u16;
            for button in buttons {
                op |= 1<<(l-button-1);
            }
            operands.push(op);
        }

        let mut dp2 = dp.clone(); 
        while dp[targ as usize] == i16::MAX {
            for i in 0..dp.len() {
                let steps = dp[i];
                if steps != i16::MAX {
                    for op in operands.iter() {
                        let new_lights = (i as u16 ^ op) as usize;
                        dp2[new_lights] = i16::min(dp[new_lights], steps+1);
                    }
                }
            }
            dp = dp2.clone();
        }
        ans += dp[targ as usize] as u64;
    }

    writeln!(out, "{}", ans).ok();
}

fn part2() {
    let out = &mut BufWriter::new(stdout());

    let buffer: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut ans: u64 = 0;

    for line in buffer.into_iter() {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let l = (parts[0].len() - 2) as u32; // length of light string

        let mut targ: Vec<u16> = Vec::new();
        let targ_str = parts.last().unwrap().trim_matches(['{','}']).split(',');
        for num in targ_str {
            let targ_num = num.parse::<u16>().unwrap();
            targ.push(targ_num);
        }

        let mut operands:Vec<Vec<u16>> = Vec::new();
        for part in parts.into_iter().skip(1) {
            if part.starts_with('{') {break;}

            let buttons: Vec<u32> = part[1..part.len()-1].split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            let mut op: Vec<u16> = vec![0; l as usize];
            for button in buttons {
                op[button as usize] = 1;
            }
            operands.push(op);
        }

        let solver = Solver::new();
        let mut consts = Vec::new();
        for i in 0..operands.len() {
            consts.push(Int::fresh_const(&i.to_string()));
        }
        for (i, targ_num) in targ.iter().enumerate() {
            solver.assert(
                consts.iter()
                .enumerate()
                .map(|(ind, var)| var*operands[ind][i])
                .sum::<Int>()
                .eq(*targ_num)); 
        }
        for c in consts.iter() {
            solver.assert(c.lt(300));
            solver.assert(c.ge(0));
        }
        let min_moves = solver
            .solutions(consts, false)
            .map(|sol| sol.iter()
            .map(Int::as_u64)
            .map(Option::unwrap)
            .sum::<u64>())
            .min()
            .unwrap();
        ans += min_moves;
    }
    writeln!(out, "{}", ans).ok();

}

fn main() {

    // let t: usize = scan.next();
    // part1();   
    part2();
    // writeln!(out, "output").ok();
}
