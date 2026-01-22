fn test_bounds() {
    let start_x: usize = 0;
    let start_y: usize = 0;
    let grid_x: usize = 4;
    let grid_y: usize = 4;
    let orientation_x_dim: usize = 3;
    let orientation_y_dim: usize = 3;
    
    let min_y = start_y.saturating_sub(orientation_y_dim - 1);
    let max_y = start_y.min(grid_y - orientation_y_dim);
    let min_x = start_x.saturating_sub(orientation_x_dim - 1);
    let max_x = start_x.min(grid_x - orientation_x_dim);
    
    println!("start: ({}, {}), grid: {}x{}, piece: {}x{}", start_x, start_y, grid_x, grid_y, orientation_x_dim, orientation_y_dim);
    println!("y range: {}..={}", min_y, max_y);
    println!("x range: {}..={}", min_x, max_x);
    
    // Should include (0,0) which is valid
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if y + orientation_y_dim <= start_y || y > start_y ||
               x + orientation_x_dim <= start_x || x > start_x {
                println!("Skipping ({}, {}) - doesn't cover start", x, y);
                continue;
            }
            println!("Would try placement at ({}, {})", x, y);
        }
    }
}

fn main() {
    test_bounds();
}
