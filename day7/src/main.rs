extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weight: usize,
    holding_names: Vec<String>,
    children: Vec<Program>
}

fn form(collection: &Vec<Program>, program: &mut Program) {
    for child in program.holding_names.iter_mut() {
        if *child == "" {
            continue;
        }
            if child.as_bytes()[0] == ' ' as u8 {
                child.remove(0);
            }
        let mut p = collection.iter().filter(|x|x.name == *child).next().unwrap().clone();
        form(&collection, &mut p);
        program.children.push(p);
    }
}

fn generate_programs(input: &String) -> Program {
    let mut programs: Vec<Program> = Vec::new();
    let re = Regex::new(r"([a-z]*) \((\d*)\) ?-?>? ?(.*)").unwrap();
    for cap in re.captures_iter(input.as_str()) {
        programs.push(Program {
            name: String::from(&cap[1]),
            weight: usize::from_str_radix(&cap[2],10).unwrap(),
            holding_names: cap[3].split(",").map(|x|String::from(x)).collect(),
            children: Vec::new()
        });
    }
    let bot = find_bottom_program_name(&input);
    let mut root = programs.iter().filter(|x|x.name == bot).next().unwrap().clone();
    form(&programs, &mut root);
    root
}

fn find_bottom_program_name(input: &String) -> String {
    let re = Regex::new(r"([a-z]+)").unwrap();
    let mut count_map: HashMap<String, usize> = HashMap::new();
    for cap in re.captures_iter(input.as_str()) {
        *count_map.entry(String::from(&cap[0])).or_insert(0) += 1;
    }
    count_map.iter().filter(|&(_,v)| *v == 1).next().unwrap().0.clone()
}

fn get_total_weight(p: &Program) -> usize {
    let mut total = p.weight;
    for child in p.children.iter() {
        total += get_total_weight(child);
    }
    total
}

fn _solve_2(program: &Program) -> usize {
    let mut v: Vec<(&Program, usize)> = Vec::new();
    for child in program.children.iter() {
        v.push((child, get_total_weight(child)));
    }
    let mut count_map: HashMap<usize, usize> = HashMap::new();
    for p in v.iter() {
        *count_map.entry(p.1).or_insert(0) += 1;
    }
    let x = count_map.values().max().unwrap();
    let x = count_map.iter().filter(|i|i.1==x).next().unwrap().0;

    let mut ret = 0usize;
    for p in v.iter() {
        if *x != p.1 && p.0.children.len() > 0 {
            let children_balance = _solve_2(p.0);
            if children_balance == 0 {
                ret = p.0.weight - (p.1 - x);
                break
            }
            else {
                ret = children_balance;
            }
        }
    }
    ret
}

fn solve_2(input: &String) -> usize {
    _solve_2(&generate_programs(&input))
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let mut file = BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    println!("{}", find_bottom_program_name(&contents));
    println!("{}", solve_2(&contents));
}
