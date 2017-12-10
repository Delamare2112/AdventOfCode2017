use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn calc_score(input: &String) -> usize {
    let mut score = 0usize;
    let mut curl_level = 0usize;
    let mut in_garbage = false;
    let mut skip = false;
    for c in input.chars() {
        if skip {
            skip = false;
        } else if c == '!' {
            skip = true;
        } else if in_garbage && c != '>' {
            continue;
        } else if c == '>' {
            in_garbage = false;
        } else if c == '<' {
            in_garbage = true;
        } else if c == '{' {
            curl_level += 1;
            score += curl_level;
        } else if c == '}' {
            curl_level -= 1;
        }
    }
    score
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file.");
    let mut file = BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file into string.");

    println!("test: {}", calc_score(&String::from("{{<a!>},{<a!>},{<a!>},{<ab>}}")));
    println!("score: {}", calc_score(&contents));
}
