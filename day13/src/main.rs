use std::io::{BufReader, Read};
use std::fs::File;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Scanner {
    range: usize,
    scan_index: usize,
    direction_mod: isize
}
impl Scanner {
    fn new(range: usize) -> Scanner {
        Scanner {range, scan_index: 0, direction_mod: 1}
    }

    fn step(&mut self) {
        if self.scan_index <= 0 {
            self.direction_mod = 1;
        }
        else if self.scan_index >= self.range - 1 {
            self.direction_mod = -1;
        }
        self.scan_index = (self.scan_index as isize + self.direction_mod) as usize;
    }
}

fn parse_input(input: &String) -> (HashMap<usize, Scanner>, usize) {
    let mut scanners: HashMap<usize, Scanner> = HashMap::new();
    let mut max_depth = 0usize;
    for line in input.lines() {
        let mut split = line.split(": ");
        let depth = usize::from_str(split.next().unwrap()).unwrap();
        scanners.insert(depth, Scanner::new(usize::from_str(split.last().unwrap()).unwrap()));
        max_depth = depth;
    }
    (scanners, max_depth)
}

fn get_severity(scanners: &mut HashMap<usize, Scanner>, max_depth: usize) -> usize {
    let mut severity = 0usize;
    for depth in 0..max_depth+1 {
        { // scoping for borrow
            let e = scanners.get(&depth);
            if e.is_some() && e.unwrap().scan_index == 0 {
                severity += depth * e.unwrap().range;
            }
        }
        for (_, scanner) in scanners.iter_mut() {
            scanner.step();
        }
    }
    severity
}

fn main() {
    let file = File::open("input.txt").expect("cannot open input file");
    let mut file = BufReader::new(file);
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut input = parse_input(&input);

    println!("{}", get_severity(&mut input.0, input.1));
}
