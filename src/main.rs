const MAT_SIZE: usize = 10;

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
fn main() {
    const MAT_SIZE : usize = 10;
    let mut world = [[0u8; MAT_SIZE]; MAT_SIZE];
    display_world(&world);
}
