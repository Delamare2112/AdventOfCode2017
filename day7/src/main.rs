extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn find_bottom_program_name(input: &String) -> String {
    let re = Regex::new(r"([a-z]+)").unwrap();
    let mut count_map: HashMap<String, usize> = HashMap::new();
    for cap in re.captures_iter(input.as_str()) {
        *count_map.entry(String::from(&cap[0])).or_insert(0) += 1;
    }
    count_map.iter().filter(|&(k,v)| *v == 1).next().unwrap().0.clone()
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let mut file = BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{}", find_bottom_program_name(&contents));
}
