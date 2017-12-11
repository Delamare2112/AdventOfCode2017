struct Input {
    sequence_size: usize,
    lengths: Vec<usize>
}

fn reverse_slice(start: usize, end: usize, vec: &mut Vec<usize>) {
    let mut a = start;
    let mut b = end;
    let mut i = if start > end {
        ((vec.len()-start)+end+1)
    }else{
        end-start
    } / 2;
    while i > 0 {
        vec.swap(a,b);
        b = if b == 0 { vec.len() - 1 } else { b - 1 };
        a = if a >= vec.len()-1 { 0 } else { a + 1 };
        i -= 1;
    }
}

fn hash(input: &Input) -> Vec<usize> {
    let mut list: Vec<usize> = (0..input.sequence_size).collect();
    let mut i = 0usize;
    let mut skip_size = 0usize;
    for len in input.lengths.iter() {
        if *len != 0 {
            let mut end_of_slice = (i + len) - 1;
            if end_of_slice >= list.len() {
                end_of_slice = end_of_slice - list.len();
            }
            reverse_slice(i, end_of_slice, &mut list);
        }
        i += len + skip_size;
        while i >= list.len() {
            i = i - list.len();
        }
        skip_size += 1;
    }
    list
}

fn checksum(x: &Vec<usize>) -> usize {
    x[0] * x[1]
}

fn main() {
    let test_input = Input {sequence_size: 5, lengths: vec!(3, 4, 1, 5)};
    let input = Input {sequence_size: 256, lengths: vec!(120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113)};
    println!("test check: {}", checksum(&hash(&test_input)));
    println!("input check: {}", checksum(&hash(&input)));
}
