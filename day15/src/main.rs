struct Generator {
    previous: usize,
    factor: usize,
    criteria: usize
}
impl Generator {
    fn next(&mut self) -> usize {
        loop {
            self.previous = (self.previous * self.factor) % 2147483647;
            if self.previous % self.criteria == 0 {
                break;
            }
        }
        self.previous
    }
}

fn judge(ga: &mut Generator, gb: &mut Generator, count: usize) -> usize {
    let mut matches = 0usize;
    for _ in 0..count {
        if ga.next() as u16 == gb.next() as u16 {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let mut ga = Generator {factor: 16807, previous: 591, criteria: 1};
    let mut gb = Generator {factor: 48271, previous: 393, criteria: 1};
    let mut ga_2 = Generator {factor: 16807, previous: 591, criteria: 4};
    let mut gb_2 = Generator {factor: 48271, previous: 393, criteria: 8};

    println!("judge: {}", judge(&mut ga, &mut gb, 40000000));
    println!("judge: {}", judge(&mut ga_2, &mut gb_2, 5000000));
}
