extern crate KnotHash;
use KnotHash::knot_hash;

type Grid = [[bool; 128]; 128];

struct GridCreationResult {
    grid: Grid,
    num_used: usize
}

fn make_grid(input: String) -> GridCreationResult {
    let mut grid = [[false; 128]; 128];
    let mut num_used = 0usize;
    for row in 0..128usize {
        let hash = knot_hash((input.clone() + "-" + row.to_string().as_str()).as_bytes().to_vec());
        let mut column = 0usize;
        for c in hash.chars() {
            let x = c.to_digit(16).unwrap() as u8;
            for i in [3, 2, 1, 0].iter() {
                if x & 1<<*i != 0 {
                    grid[row][column] = true;
                    num_used += 1;
                }
                column += 1;
            }
        }
    }
    GridCreationResult {grid, num_used}
}

fn display_grid(grid: &Grid) {
    for row in 0..128usize {
        for col in 0..128usize {
            if grid[row][col] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let grid_r = make_grid("vbqugkhl".to_string());
    display_grid(&grid_r.grid);
    println!("used: {}", grid_r.num_used);
}
