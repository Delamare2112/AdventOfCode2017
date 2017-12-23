fn make_buffer(steps: usize) -> Vec<usize> {
    let mut buf = vec![0usize];
    let mut i = 0usize;
    for x in 1..2018usize {
        i = ((i + steps) % buf.len()) + 1;
        if i == buf.len() {
            buf.push(x);
        }
        else {
            buf.insert(i, x);
        }
    }
    buf
}

fn main() {
    let test = make_buffer(3);
    let s1 = make_buffer(324);
    assert_eq!(638, test[test.iter().position(|x| *x == 2017).unwrap() + 1], "make_buffer broke");

    println!("{}", s1[s1.iter().position(|x| *x == 2017).unwrap() + 1]);
}
