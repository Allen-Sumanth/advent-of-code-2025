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
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let mut coords: Vec<[i64; 2]> = Vec::new();
    while scan.has_next() {
        let inp: Vec<i64> = scan.next::<String>().split(',').map(|i| i.parse::<i64>().unwrap()).collect();
        coords.push([inp[0], inp[1]]);
    }
    
    let mut largest_ar: i64= 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let ar = (coords[i][0]-coords[j][0]+1).abs() * (coords[i][1]-coords[j][1]+1);
            largest_ar = i64::max(largest_ar, ar);
        }
    }

    writeln!(out, "{}", largest_ar).ok();
}

fn part2() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let mut xcoords: Vec<[i64; 2]> = Vec::new();
    let mut ycoords: Vec<[i64; 2]> = Vec::new();
    while scan.has_next() {
        let inp: Vec<i64> = scan.next::<String>().split(',').map(|i| i.parse::<i64>().unwrap()).collect();
        xcoords.push([inp[0], inp[1]]);
        ycoords.push([inp[1], inp[0]]);
    }
    xcoords.sort_by(|a, b| a[0].cmp(&b[0])); 
    ycoords.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut largest_ar: i64= 0;
    let l = xcoords.len();
    for i in 0..l {
        for j in i+1..l {
            let (x1, y1): (i64, i64) = (xcoords[i][0], xcoords[i][1]);
            let (x2, y2): (i64, i64) = (xcoords[j][0], xcoords[j][1]);
            let (xmax, xmin) = (i64::max(x1, x2), i64::min(x1, x2));
            let (ymax, ymin) = (i64::max(y1, y2), i64::min(y1, y2));

            let ar = (xmax-xmin+1) * (ymax-ymin+1);
            if ar < largest_ar {continue;}
            
            let x_split = xcoords.partition_point(|x| x[0] <= xmin);
            let y_split = ycoords.partition_point(|y| y[0] <= ymin);

            let mut invalid = false;
            
            // check if each point between xmin and xmax are not intersected by any edges
            for xi in (x_split..l-1).step_by(2){
                let (x, y) = (xcoords[xi][0], xcoords[xi][1]);
                let (_xnext, ynext) = (xcoords[xi+1][0], xcoords[xi+1][1]);
                
                if x == xmax {
                    break;
                }

                if (y > ymin && y < ymax) || (ynext > ymin && ynext < ymax) || (ymin >= i64::min(y, ynext) && ymax <= i64::max(y, ynext)) {
                    invalid = true;
                    break;
                }
            }

            if invalid {continue;}

            // check the same for ymin and ymax
            for yi in (y_split..l-1).step_by(2) {
                let (y, x) = (ycoords[yi][0], ycoords[yi][1]);
                let (_ynext, xnext) = (ycoords[yi+1][0], ycoords[yi+1][1]);

                if y == ymax {
                    break;
                }

                if (x > xmin && x < xmax) || (xnext > xmin && xnext < xmax) || (xmin >= i64::min(x, xnext) && xmax <= i64::max(x, xnext)) {
                    invalid = true;
                    break;
                }

           }

            // check if the entire rect is inside the polygon or not
            if invalid {continue;}
            let mut num_intersections = 0;
            for i in 0..x_split {
                let (_x, y) = (xcoords[i][0], xcoords[i][1]);
                let (_xnext, ynext) = (xcoords[i+1][0], xcoords[i+1][1]);

                if (y > ymin && y < ymax) || (ynext > ymin && ynext < ymax) || (ymin >= i64::min(y, ynext) && ymax <= i64::max(y, ynext)) {
                    num_intersections += 1;
                }
            }

            if !invalid && num_intersections%2 == 0 {
                // writeln!(out, "{} {}, {} {}, {}", x1, y1, x2, y2, ar).ok();
                largest_ar = i64::max(ar, largest_ar);
            }
            
        }
    }

    writeln!(out, "{}", largest_ar).ok();
}

fn main() {

    // let t: usize = scan.next();
    // part1();
    part2();
    // writeln!(out, "output").ok();
}
