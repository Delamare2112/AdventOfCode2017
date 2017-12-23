use std::io::{BufReader, Read};
use std::fs::File;
use std::str::FromStr;

fn solve_1(mut vec: &mut Vec<char>, input: &String) {
    for command in input.split(',') {
        let op = command.as_bytes()[0] as char;
        if op == 's' {
            let x = usize::from_str(command.split('s').last().unwrap()).unwrap();
            let split_point = vec.len()-x;
            let mut split = vec.split_off(split_point);
            split.append(&mut vec);
            *vec = split;
        }
        else if op == 'x' {
            let mut split = command.split('/');
            let mut a: String = split.next().unwrap().to_string();
            a.remove(0);
            let a = usize::from_str(a.as_str()).unwrap();
            let b = usize::from_str(split.next().unwrap()).unwrap();
            vec.swap(a, b);
        }
        else if op == 'p' {
            let mut split = command.split('/');
            let a = split.next().unwrap().as_bytes()[1] as char;
            let b = split.next().unwrap().as_bytes()[0] as char;
            let a = vec.iter().position(|x| *x == a).unwrap();
            let b = vec.iter().position(|x| *x == b).unwrap();
            vec.swap(a, b);
        }
    }
}

fn solve_2(mut vec: &mut Vec<char>, input: &String) -> String {
    let mut results: Vec<String> = vec![vec.clone().iter().collect()];
    loop {
        solve_1(&mut vec, &input);
        let x: String = vec.iter().collect();
        if x == "abcdefghijklmnop" {
            return results[1000000000 % results.len()].clone();
        }
        results.push(x);
    }
}

fn main() {
    let mut vec: Vec<char> = vec!('a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p');
    let file = File::open("input.txt").expect("cannot open input file");
    let mut file = BufReader::new(file);
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut vec1 = vec.clone();
    solve_1(&mut vec1, &input);
    let s: String = vec1.iter().collect();
    println!("{}", s);
    println!("{}", solve_2(&mut vec, &input));
}
