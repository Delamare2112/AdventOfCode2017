use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn count_valid(lines: Vec<String>) -> usize {
    let mut count = 0usize;
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let unique: HashMap<&&str, usize> = words.iter().enumerate().map(|(c,x)| (x,c)).collect();
        if words.len() == unique.len() {
            count += 1;
        }
    }
    count
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().filter_map(|x| x.ok()).collect();
    println!("{}", count_valid(lines));
}
