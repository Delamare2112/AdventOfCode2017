use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn count_valid_p1(lines: &Vec<String>) -> usize {
    let mut count = 0usize;
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let unique: HashMap<&&str, usize> = words.iter().map(|x| (x,0)).collect();
        if words.len() == unique.len() {
            count += 1;
        }
    }
    count
}

fn count_valid_p2(lines: &Vec<String>) -> usize {
    let mut count = 0usize;
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut count_maps: Vec<HashMap<char, usize>> = Vec::new();
        let mut dup_found = false;
        for word in words {
            let mut count_map: HashMap<char, usize> = HashMap::new();
            for c in word.chars() {
                *count_map.entry(c).or_insert(0) += 1;
            }
            if count_maps.iter().any(|x| *x == count_map) {
                dup_found = true;
                break;
            }
            count_maps.push(count_map);
        }
        if !dup_found {
            count += 1;
        }
    }
    count
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().filter_map(|x| x.ok()).collect();
    println!("{}", count_valid_p1(&lines));
    println!("{}", count_valid_p2(&lines));
}
