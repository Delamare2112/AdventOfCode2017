extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

type SystemState = HashMap<String, isize>;

fn compare(a: isize, b: isize, token: &str) -> bool {
    match token {
        "==" => a == b,
        ">=" => a >= b,
        "<=" => a <= b,
        "!=" => a != b,
        ">" => a > b,
        "<" => a < b,
        _ => false
    }
}

fn interpret(input: &String) -> SystemState {
    let mut system = SystemState::new();
    let re = Regex::new(r"([a-z]+) (dec|inc) (-?\d+) if ([a-z]+) ([<=!>]+) (-?\d+)").unwrap();
    for cap in re.captures_iter(input.as_str()) {
        let a = *system.entry(String::from(&cap[4])).or_insert(0);
        let b = isize::from_str_radix(&cap[6], 10).unwrap();
        if compare(a, b, &cap[5]) {
            let dest = system.entry(String::from(&cap[1])).or_insert(0);
            let src = isize::from_str_radix(&cap[3], 10).unwrap();
            if cap[2] == *"inc" {
                *dest += src;
            }
            else {
                *dest -= src;
            }
        }
    }
    system
}

fn find_largest_register_value(system: &SystemState) -> isize {
    *system.values().max().unwrap()
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file.");
    let mut file = BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("the file could not be read to a string.");
    println!("{}", find_largest_register_value(&interpret(&contents)));
}
