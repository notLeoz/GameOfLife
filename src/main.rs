use std::{thread, time::Duration};

const MAT_SIZE: usize = 40;
const GENERATIONS: u8 = 140;
const GEN_SPEED: u64 = 200;


fn get_sum_neighbors(world: &[[u8; MAT_SIZE]; MAT_SIZE], pos_x: u32,pos_y: u32) -> u32 {
    let mut sum: u32 = 0;
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];
    for &(dx, dy) in &neighbors {
        let new_x = pos_x as i32 + dx;
        let new_y = pos_y as i32 + dy;
        if new_x >= 0 && new_x < MAT_SIZE as i32 && new_y >= 0 && new_y < MAT_SIZE as i32 {
            sum += world[new_x as usize][new_y as usize] as u32;
        }
    }
    sum
}
fn display_world(world: &[[u8; MAT_SIZE]; MAT_SIZE]) {
    world.iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{} ", cell));
        println!();
    });
}

fn get_new_cell_value(is_curr_cell_alive: bool,sum_neighbors: u32) -> u8 {
    match is_curr_cell_alive {
       true => {
        match sum_neighbors {
           0..=1 => 0,
           2..=3 => 1,
           _ => 0
        }
       } 
       false => {
        match sum_neighbors {
            3 => 1,
            _ => 0
        }
       }
    }
}

fn create_new_world(curr_world:&[[u8; MAT_SIZE];MAT_SIZE]) -> [[u8; MAT_SIZE];MAT_SIZE] {
    let mut new_world = [[0u8; MAT_SIZE]; MAT_SIZE];
    for i in 0..MAT_SIZE {
        for j in 0..MAT_SIZE {
            new_world[i][j] = get_new_cell_value(if curr_world[i][j] == 1 {true} else {false}, get_sum_neighbors(curr_world, i as u32, j as u32));
        }
    }
    new_world
}

fn copy_new_world(curr_world:&mut[[u8; MAT_SIZE]], new_world: [[u8;MAT_SIZE];MAT_SIZE]) {
    for i in 0..MAT_SIZE {
        for j in 0..MAT_SIZE {
            curr_world[i][j] = new_world[i][j];
        }
    }
}

fn apply_die_hard_configuration(world:&mut[[u8; MAT_SIZE]]) {
    world[14][18] = 1;
    world[16][17] = 1;
    world[16][18] = 1;
    world[16][19] = 1;
    world[15][12] = 1;
    world[15][13] = 1;
    world[16][13] = 1;
}

fn main() {
    let mut world = [[0u8; MAT_SIZE]; MAT_SIZE];
    apply_die_hard_configuration(&mut world);
    for _ in 0..GENERATIONS {
        print!("{}[2J", 27 as char);
        display_world(&world);
        thread::sleep(Duration::from_millis(GEN_SPEED));
        let new_world = create_new_world(&world);
        copy_new_world(&mut world, new_world);
        
    }
}
