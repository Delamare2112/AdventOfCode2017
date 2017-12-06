use std::io::{BufRead, BufReader};
use std::fs::File;

fn solve_1(offsets: &Vec<isize>) -> usize {
    let mut offsets = offsets.clone();
    let mut count = 0usize;
    let mut i = 0isize;
    let len = offsets.len() as isize;
    while i < len && i >= 0 {
        let offset = offsets[i as usize];
        offsets[i as usize] += 1;
        i += offset;
        count += 1;
    }
    count
}

fn solve_2(offsets: &Vec<isize>) -> usize {
    let mut offsets = offsets.clone();
    let mut count = 0usize;
    let mut i = 0isize;
    let len = offsets.len() as isize;
    while i < len && i >= 0 {
        let offset = offsets[i as usize];
        offsets[i as usize] += if offset >= 3 {-1} else {1};
        i += offset;
        count += 1;
    }
    count
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let file = BufReader::new(file);
    let jumps: Vec<isize> = file.lines()
        .filter_map(|x| x.ok())
        .map(|y| isize::from_str_radix(y.as_str(),10).unwrap())
        .collect();

    println!("{}", solve_1(&jumps));
    println!("{}", solve_2(&jumps));
}
