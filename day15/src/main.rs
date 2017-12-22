struct Generator {
    previous: usize,
    factor: usize
}
impl Generator {
    fn next(&mut self) -> usize {
        self.previous = (self.previous * self.factor) % 2147483647;
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
    let mut ga = Generator {factor: 16807, previous: 591};
    let mut gb = Generator {factor: 48271, previous: 393};

    println!("judge: {}", judge(&mut ga, &mut gb, 40000000));
}
