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

struct Djs {
    size: Vec<usize>,
    parent: Vec<usize>
}

impl Djs {
    fn new(n: usize) -> Self {
        Self {
            size: vec![1;n],
            parent: (0..n).collect()
        }
    }    

    fn find_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {node}
        else {
            self.parent[node] = self.find_parent(self.parent[node]);
            self.parent[node]
        }
    }

    fn union(&mut self, node1: usize, node2: usize) -> usize {
        let (p1, p2) = (self.find_parent(node1), self.find_parent(node2));
        if p1 == p2 {return self.size[p1];}
        let (s1, s2) = (self.size[p1], self.size[p2]);
        if s1 >= s2 {
            self.parent[p2] = p1;
            self.size[p1] += self.size[p2];
        } else {
            self.parent[p1] = p2;
            self.size[p2] += self.size[p1];
        }
        s1+s2
    }
}

fn part1() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut coords: Vec<[f64;3]> = Vec::new();
    while scan.has_next() {
        let inp: Vec<f64> = scan.next::<String>().split(',').map(|num| num.parse::<f64>().unwrap()).collect();
        let coord = [inp[0], inp[1], inp[2]];
        coords.push(coord);
    }
    let mut closest_pairs = collections::BinaryHeap::new();
    
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let (coord1, coord2) = (coords[i], coords[j]);
            let mut dsum: f64 = 0f64;
            for k in 0..3 {dsum += (coord2[k]-coord1[k])*(coord2[k]-coord1[k]);}
            let dist: i64 = f64::sqrt(dsum) as i64;
            closest_pairs.push(cmp::Reverse((dist, i, j)));
        }
    }


    let mut djs = Djs::new(coords.len());
    const TOP_N: usize = 1000;

    for _ in 0..TOP_N {
        let conn = closest_pairs.pop().unwrap().0;
        djs.union(conn.1, conn.2);
    }
    let mut top_three = collections::BinaryHeap::new();
    for node in 0..coords.len() {
        let parent = djs.find_parent(node);
        if node != parent {continue;}
        let s = djs.size[parent];
        top_three.push(s);
    }
    let ans = top_three.pop().unwrap()*top_three.pop().unwrap()*top_three.pop().unwrap();
    writeln!(out, "{}", ans).ok();
}

fn part2() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut coords: Vec<[f64;3]> = Vec::new();
    while scan.has_next() {
        let inp: Vec<f64> = scan.next::<String>().split(',').map(|num| num.parse::<f64>().unwrap()).collect();
        let coord = [inp[0], inp[1], inp[2]];
        coords.push(coord);
    }
    let mut closest_pairs = collections::BinaryHeap::new();
    
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let (coord1, coord2) = (coords[i], coords[j]);
            let mut dsum: f64 = 0f64;
            for k in 0..3 {dsum += (coord2[k]-coord1[k])*(coord2[k]-coord1[k]);}
            let dist: i64 = f64::sqrt(dsum) as i64;
            closest_pairs.push(cmp::Reverse((dist, i, j)));
        }
    }


    let mut djs = Djs::new(coords.len());
    let ans: f64;
    loop {
        let conn = closest_pairs.pop().unwrap().0;
        let p_size = djs.union(conn.1, conn.2);
        if p_size == coords.len() {
            ans = coords[conn.1][0]*coords[conn.2][0];
            break;
        }
    }
    writeln!(out, "{}", ans).ok();
}


fn main() {

    // let t: usize = scan.next();
    // part1();
    part2();
    
    // writeln!(out, "output").ok();
}
