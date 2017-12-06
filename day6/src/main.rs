
fn solve_1(input: &Vec<usize>) -> usize {
    let mut count = 0usize;
    let mut state = input.clone();
    let mut states: Vec<Vec<usize>> = Vec::new();

    while !states.contains(&state) {
        states.push(state.clone());
        // find largest bank location and size
        let mut bank = 0usize;
        let mut max = 0usize;
        for (c, x) in state.iter().enumerate() {
            if *x > max {
                bank = c;
                max = *x;
            }
        }
        // redistribute
        let mut i = max;
        state[bank] = 0;
        while i > 0 {
            bank += 1;
            if bank >= input.len() {
                bank = 0;
            }
            state[bank] += 1;
            i -= 1;
        }
        count += 1;
    }
    count
}

fn main() {
    let input: Vec<usize> = vec![11,11,13,7,0,15,5,5,4,4,1,1,7,1,15,11];
    let test_input: Vec<usize> = vec![0,2,7,0];
    println!("{}", solve_1(&test_input));
    println!("{}", solve_1(&input)); // 100 is wrong!
}
