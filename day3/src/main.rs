const T1: usize = 1;
const T2: usize = 12;
const T3: usize = 23;
const T4: usize = 1024;
const I1: usize = 361527;

fn get_manhattan_distance(loc: usize) -> usize {
    let loc = loc as f32;
    let mut base = (loc).sqrt().ceil();
    if base % 2f32 == 0f32 {
        base += 1f32;
    }

    let max = base.powi(2);
    let inside_dist = (base / 2f32).floor();

    let mid: f32;
    {
        let mid_dist = inside_dist + 1f32;
        let mut min = max - base;
        loop {
            if loc > min {
                mid = min + mid_dist;
                break;
            }
            min = min - (base - 1f32);
        }
    }
    (inside_dist + (loc - mid).abs()) as usize
}

fn main() {
    println!("{}", get_manhattan_distance(T1));
    println!("{}", get_manhattan_distance(T2));
    println!("{}", get_manhattan_distance(T3));
    println!("{}", get_manhattan_distance(T4));
    println!("{}", get_manhattan_distance(I1));
}
