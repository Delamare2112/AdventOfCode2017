struct Input {
    sequence_size: usize,
    lengths: Vec<u8>
}

fn sparse_hash(input: &Input, rounds: usize) -> Vec<usize> {
    let mut list: Vec<usize> = (0..input.sequence_size).collect();
    let mut i = 0usize;
    let mut skip_size = 0usize;
    let len = list.len();
    for _round in 0..rounds {
        for length in input.lengths.iter() {
            for mid in 0..(length / 2) {
                list.swap((i + mid as usize) % len, (i + *length as usize - mid as usize - 1) % len);
            }

            i += *length as usize + skip_size;
            skip_size += 1;
        }
    }
    list
}

fn dense_hash(sparse: &Vec<usize>) -> String {
    let mut knot_hash = String::new();
    for block in sparse.chunks(16) {
        let mut xored = 0u8;
        for num in block {
            xored ^= *num as u8;
        }
        let mut new_str = format!("{:x}", xored);
        if new_str.len() < 2 {
            new_str.insert(0, '0');
        }
        knot_hash += new_str.as_str();
    }
    knot_hash
}

fn main() {
    let mut salt = vec!(17, 31, 73, 47, 23);
    let mut input_2_len = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113".as_bytes().to_vec();
    input_2_len.append(&mut salt);
    let input_2 = Input {sequence_size: 256, lengths: input_2_len};
    println!("hash: {}", dense_hash(&sparse_hash(&input_2, 64)));
}
