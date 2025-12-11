#![allow(unused_imports)]
use std::*;
use std::io::prelude::*;
use std::io::{BufWriter, stdin, stdout};

fn part1() {
    let out = &mut BufWriter::new(stdout());

    let buffer: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut graph = collections::HashMap::new();

    for line in buffer.into_iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let par: String = parts[0].trim_matches(':').to_string();
        let children: Vec<String> = parts[1..parts.len()].iter().map(|p| p.to_string()).collect();
        graph.insert(
            par, // String
            children // vec of strings
        );
    }

    fn search (graph: &collections::HashMap<String, Vec<String>>, s: String) -> u64 {
        if s == "out" {return 1;}
        let mut tot_paths = 0;
        for neigh in graph[&s].iter() {
            tot_paths += search(graph, neigh.to_string());
        }
        tot_paths
    }
    let ans = search(&graph, "you".to_string());
    writeln!(out, "{}", ans).ok();

}

fn part2() {
    let out = &mut BufWriter::new(stdout());

    let buffer: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
    let mut graph = collections::HashMap::new();

    for line in buffer.into_iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let par: String = parts[0].trim_matches(':').to_string();
        let children: Vec<String> = parts[1..parts.len()].iter().map(|p| p.to_string()).collect();
        graph.insert(
            par, // String
            children // vec of strings
        );
    }
    let mut dp = collections::HashMap::<String, (u64, u64, u64, u64)>::new();
    fn search (
        graph: &collections::HashMap<String, Vec<String>>, 
        s: String, 
        dp: &mut collections::HashMap<String, (u64, u64, u64, u64)>
    ) -> (u64, u64, u64, u64) {
        if dp.contains_key(&s) {return dp[&s];}
        if s == "out" {return (1, 0, 0, 0);}
        let (mut tot_paths, mut dac_found, mut fft_found, mut both_found) = (0, 0, 0, 0);

        for neigh in graph[&s].iter() {
            let (paths, dac, fft, both) = search(graph, neigh.to_string(), dp);
            tot_paths += paths;
            dac_found += dac;
            fft_found += fft;
            both_found += both;
        }
        if s == "fft" {
            fft_found += tot_paths;
            both_found += dac_found;
        }
        if s == "dac" {
            dac_found += tot_paths;
            both_found += fft_found;
        }
        dp.insert(s, (tot_paths, dac_found, fft_found, both_found));
        (tot_paths, dac_found, fft_found, both_found)
    }

    let (_, _, _, ans) = search(&graph, "svr".to_string(), &mut dp);
    writeln!(out, "{}", ans).ok();

}

fn main() {

    // let t: usize = scan.next();
    // part1();    
    part2();
    // writeln!(out, "output").ok();
}
