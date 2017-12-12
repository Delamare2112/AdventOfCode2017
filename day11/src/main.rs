use std::io::{BufRead, BufReader};
use std::fs::File;

const S3_2: f64 = 0.866025403784438646763723170752936183471402626905190314027f64;

fn close_to_zero(f: f64) -> bool {
    f < 2.22e-10 && f > -2.22e-10
}

#[derive(Default, Debug, Copy, Clone)]
struct Position {
    x: f64,
    y: f64
}

impl Position {
    fn go(&mut self, direction: &str) {
        match direction {
            "n" => self.y += 1f64,
            "s" => self.y -= 1f64,
            "nw" => {self.y += 0.5; self.x -= S3_2}
            "sw" => {self.y -= 0.5; self.x -= S3_2}
            "se" => {self.y -= 0.5; self.x += S3_2}
            "ne" => {self.y += 0.5; self.x += S3_2}
            _ => {}
        }
    }
    fn len(&self) -> usize {
//        (self.x.powi(2) + self.y.powi(2)).sqrt() // that would be too euclidean!
        let mut p = self.clone();
        let mut len = 0usize;
        while !close_to_zero(p.x) || !close_to_zero(p.y) {
            if close_to_zero(p.x) {
                if p.y > 0.0 {
                    p.go("s");
                } else {
                    p.go("n");
                }
            } else if p.x > 0.0 {
                if p.y > 0.0 {
                    p.go("sw");
                } else {
                    p.go("nw");
                }
            } else if p.x < 0.0 {
                if p.y > 0.0 {
                    p.go("se");
                } else {
                    p.go("ne");
                }
            }
            len += 1;
        }
        len
    }
}

fn solve(input: &Vec<&str>) -> (usize, usize) {
    let mut pos = Position::default();
    let mut max_pos = Position::default();
    for direction in input {
        pos.go(direction);
        if pos.len() > max_pos.len() {
            max_pos = pos.clone();
        }
    }
    (pos.len(), max_pos.len())
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let mut file = BufReader::new(file);
    let mut input = String::new();
    file.read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split(',').collect();

    println!("test 1: {}", solve(&"ne,ne,ne".split(',').collect()).0);
    println!("test 2: {}", solve(&"ne,ne,sw,sw".split(',').collect()).0);
    println!("test 3: {}", solve(&"ne,ne,s,s".split(',').collect()).0);
    println!("test 4: {}", solve(&"se,sw,se,sw,sw".split(',').collect()).0);
    let solution = solve(&input);
    println!("part 1: {}", solution.0);
    println!("part 2: {}", solution.1);
}
