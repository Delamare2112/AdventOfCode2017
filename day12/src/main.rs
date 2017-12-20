extern crate regex;
use regex::*;

use std::io::{BufReader, Read};
use std::fs::File;
use std::str::FromStr;

fn parse_to_program_list(input: &String) -> Vec<Vec<usize>> {
    let mut programs: Vec<Vec<usize>> = Vec::new();
    let re = Regex::new(r"\d+ <-> (\d+.*)").unwrap();
    for cap in re.captures_iter(input.as_str()) {
        programs.push(
            cap[1].split(',').map(|x|usize::from_str(x.trim()).unwrap()).collect()
        );
    }
    programs
}

fn add_connections(programs: &Vec<Vec<usize>>, mut group: &mut Vec<usize>, program: usize) {
    group.push(program);
    for pid in programs[program].iter() {
        if !group.contains(pid) {
            add_connections(&programs, &mut group, *pid);
        }
    }
}

fn split_into_groups(input: &String) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let programs = parse_to_program_list(&input);
    let mut processed_programs: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < programs.len() {
        let mut group: Vec<usize> = Vec::new();
        add_connections(&programs, &mut group, i);
        groups.push(group.clone());

        processed_programs.append(&mut group);
        while processed_programs.contains(&i) {
            i += 1;
        }
    }
    groups
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let mut file = BufReader::new(file);
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let groups = split_into_groups(&input);
    println!("{}", groups[0].len());
    println!("{}", groups.len());
}
