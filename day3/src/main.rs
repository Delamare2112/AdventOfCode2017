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
    let mid_dist = inside_dist + 1f32;
    let mut min = max - base;
    loop {
        if loc > min {
            return (inside_dist + (loc - (min + mid_dist)).abs()) as usize
        }
        min = min - (base - 1f32);
    }
}

struct Point<T> {
    x: T,
    y: T
}

mod direction {
    pub const NORTH: i8 = 0;
    pub const EAST: i8 = 1;
    pub const SOUTH: i8 = 2;
    pub const WEST: i8 = 3;
}
struct Player {
    position: Point<usize>,
    direction: i8
}
impl Player {
    fn rotate(&mut self, direction: char) {
        match direction {
            'R' => self.direction += 1,
            'L' => self.direction -= 1,
            _ => {}
        }
        if self.direction > 3 {
            self.direction = 0;
        }
        else if self.direction < 0 {
            self.direction = 3;
        }
    }

    fn move_forward(&mut self, amount: usize) {
        match self.direction {
            direction::NORTH => self.position.y += amount,
            direction::SOUTH => self.position.y -= amount,
            direction::EAST => self.position.x += amount,
            direction::WEST => self.position.x -= amount,
            _ => {}
        }
    }
}

fn get_next_largest(num: usize) -> usize {
    let size = (num as f32).log(3f32) as usize;
    let mut grid: Vec<Vec<usize>> = vec![vec![0; size]; size];
    let mid = size / 2;
    let mut player = Player {position: Point {x: mid, y: mid}, direction: direction::EAST};
    grid[player.position.x][player.position.y] = 1;
    player.move_forward(1);

    let mut len = 1usize;
    let mut i = 1usize;
    let mut rotations = 1usize;
    loop {
        // check around thy self
        grid[player.position.x][player.position.y] =
            grid[player.position.x + 1][player.position.y] +
            grid[player.position.x + 1][player.position.y + 1] +
            grid[player.position.x + 1][player.position.y - 1] +
            grid[player.position.x][player.position.y + 1] +
            grid[player.position.x - 1][player.position.y] +
            grid[player.position.x - 1][player.position.y + 1] +
            grid[player.position.x - 1][player.position.y - 1] +
            grid[player.position.x][player.position.y - 1];

        // are we large enough yet?
        if grid[player.position.x][player.position.y] > num {
            return grid[player.position.x][player.position.y];
        }

        // rotate if we need to.
        if i % len == 0 {
            player.rotate('L');
            i = 0;
            rotations += 1;
            if rotations % 3 == 0 {
                len += 1;
                rotations = 1;
            }
        }
        i += 1;
        player.move_forward(1);
    }
}

fn main() {
    println!("{}", get_manhattan_distance(T1));
    println!("{}", get_manhattan_distance(T2));
    println!("{}", get_manhattan_distance(T3));
    println!("{}", get_manhattan_distance(T4));
    println!("{}", get_manhattan_distance(I1));
    println!("{}", get_next_largest(I1));
}
