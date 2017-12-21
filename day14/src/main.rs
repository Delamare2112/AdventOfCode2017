extern crate KnotHash;
use KnotHash::knot_hash;

type Grid = [[isize; 128]; 128];

struct GridCreationResult {
    grid: Grid,
    num_used: usize
}

fn make_grid(input: String) -> GridCreationResult {
    let mut grid = [[-1isize; 128]; 128];
    let mut num_used = 0usize;
    for row in 0..128usize {
        let hash = knot_hash((input.clone() + "-" + row.to_string().as_str()).as_bytes().to_vec());
        let mut column = 0usize;
        for c in hash.chars() {
            let x = c.to_digit(16).unwrap() as u8;
            for i in [3, 2, 1, 0].iter() {
                if x & 1<<*i != 0 {
                    grid[row][column] = 0;
                    num_used += 1;
                }
                column += 1;
            }
        }
    }
    GridCreationResult {grid, num_used}
}

fn mark_group(group_id: usize, row: usize, col: usize, mut grid: &mut Grid) -> bool {
    if grid[row][col] == 0 {
        grid[row][col] = group_id as isize;
        if col != 127 {
            mark_group(group_id, row, col+1, &mut grid);
        }
        if col != 0 {
            mark_group(group_id, row, col-1, &mut grid);
        }
        if row != 127 {
            mark_group(group_id, row+1, col, &mut grid);
        }
        if row != 0 {
            mark_group(group_id, row-1, col, &mut grid);
        }
        return true
    }
    false
}

fn group_grid(mut grid: &mut Grid) -> usize {
    let mut groups = 1usize;
    for row in 0..128usize {
        for col in 0..128usize {
            if mark_group(groups, row, col, &mut grid) {
                groups += 1;
            }
        }
    }
    groups - 1
}

fn display_grid(grid: &Grid, size: usize) {
    for row in 0..size {
        for col in 0..size {
            if grid[row][col] != -1 {
//                print!("#");
                print!("{}\t", grid[row][col]);
            } else {
                print!(".\t");
            }
        }
        println!();
    }
}

fn main() {
    let mut grid_r = make_grid("vbqugkhl".to_string());
    println!("groups: {}", group_grid(&mut grid_r.grid));
//    display_grid(&grid_r.grid, 12);
    println!("used: {}", grid_r.num_used);
}
