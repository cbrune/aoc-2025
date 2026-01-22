use std::fs::File;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Present {
    foot_print: Vec<Vec<bool>>,
    x_dim: usize,
    y_dim: usize,
}

impl Present {
    fn squares(&self) -> usize {
        let mut total = 0;
        for row in &self.foot_print {
            for item in row.iter() {
                if *item {
                    total += 1;
                }
            }
        }

        total
    }

    fn rotate_90(&self) -> Present {
        let new_y = self.x_dim;
        let new_x = self.y_dim;
        let mut new_fp = vec![vec![false; new_x]; new_y];
        
        for y in 0..self.y_dim {
            for x in 0..self.x_dim {
                new_fp[x][self.y_dim - 1 - y] = self.foot_print[y][x];
            }
        }
        
        Present {
            foot_print: new_fp,
            x_dim: new_x,
            y_dim: new_y,
        }
    }

    fn flip_horizontal(&self) -> Present {
        let mut new_fp = vec![vec![false; self.x_dim]; self.y_dim];
        
        for y in 0..self.y_dim {
            for x in 0..self.x_dim {
                new_fp[y][self.x_dim - 1 - x] = self.foot_print[y][x];
            }
        }
        
        Present {
            foot_print: new_fp,
            x_dim: self.x_dim,
            y_dim: self.y_dim,
        }
    }

    fn all_orientations(&self) -> Vec<Present> {
        let mut orientations = Vec::new();
        let mut current = self.clone();
        
        // 4 rotations
        for _ in 0..4 {
            orientations.push(current.clone());
            current = current.rotate_90();
        }
        
        // Flip and 4 more rotations
        current = self.flip_horizontal();
        for _ in 0..4 {
            orientations.push(current.clone());
            current = current.rotate_90();
        }
        
        // Remove duplicates
        orientations.sort_by_key(|p| (p.y_dim, p.x_dim, format!("{:?}", p.foot_print)));
        orientations.dedup();
        orientations
    }
}

#[derive(Debug, Clone)]
struct Region {
    x_dim: usize,
    y_dim: usize,
    presents: Vec<usize>,
}

fn data_init(path: &str) -> Result<(Vec<Present>, Vec<Region>), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut presents = Vec::new();
    let mut regions = Vec::new();

    let mut present_rows = Vec::new();
    let mut in_present = false;
    for line in lines {
        let line_str = line.expect("Bad string");
        if line_str.ends_with(":") {
            in_present = true;
            present_rows.clear();
            continue;
        }
        if in_present {
            if line_str.contains("#") || line_str.contains(".") {
                let mut present_row = Vec::new();
                for c in line_str.chars() {
                    match c {
                        '#' => present_row.push(true),
                        '.' => present_row.push(false),
                        _ => panic!("bad present row"),
                    }
                }
                present_rows.push(present_row);
            }
            if line_str == "" {
                // end of present
                let present = Present {
                    foot_print: present_rows.clone(),
                    x_dim: present_rows[0].len(),
                    y_dim: present_rows.len(),
                };
                presents.push(present);
                in_present = false;
            }
            continue;
        }
        let parts: Vec<&str> = line_str.split(":").collect();
        if parts.len() == 2 {
            let dim_strs: Vec<&str> = parts[0].split("x").collect();
            let x = dim_strs[0].parse::<usize>().expect("bad x dim");
            let y = dim_strs[1].parse::<usize>().expect("bad y dim");
            let mut present_strs: Vec<&str> = parts[1].split(" ").collect();
            present_strs.remove(0);
            let mut presents = Vec::new();
            for pres in &present_strs {
                presents.push(pres.parse::<usize>().expect("bad present region"));
            }
            regions.push(Region {
                x_dim: x,
                y_dim: y,
                presents,
            });
        }
    }

    Ok((presents, regions))
}

fn can_place(
    grid: &Vec<Vec<bool>>,
    present: &Present,
    x: usize,
    y: usize,
    grid_x: usize,
    grid_y: usize,
) -> bool {
    if x + present.x_dim > grid_x || y + present.y_dim > grid_y {
        return false;
    }

    for py in 0..present.y_dim {
        for px in 0..present.x_dim {
            if present.foot_print[py][px] && grid[y + py][x + px] {
                return false;
            }
        }
    }

    true
}

fn place(grid: &mut Vec<Vec<bool>>, present: &Present, x: usize, y: usize) {
    for py in 0..present.y_dim {
        for px in 0..present.x_dim {
            if present.foot_print[py][px] {
                grid[y + py][x + px] = true;
            }
        }
    }
}

fn unplace(grid: &mut Vec<Vec<bool>>, present: &Present, x: usize, y: usize) {
    for py in 0..present.y_dim {
        for px in 0..present.x_dim {
            if present.foot_print[py][px] {
                grid[y + py][x + px] = false;
            }
        }
    }
}

fn solve(
    grid: &mut Vec<Vec<bool>>,
    present_list: &[(usize, Vec<Present>)],
    idx: usize,
    grid_x: usize,
    grid_y: usize,
    start_time: Instant,
    timeout: Duration,
) -> bool {
    if start_time.elapsed() > timeout {
        return false;
    }

    if idx == present_list.len() {
        return true;
    }

    let (_present_idx, orientations) = &present_list[idx];

    for orientation in orientations {
        for y in 0..grid_y {
            if y + orientation.y_dim > grid_y {
                continue;
            }
            for x in 0..grid_x {
                if x + orientation.x_dim > grid_x {
                    continue;
                }
                
                if can_place(grid, orientation, x, y, grid_x, grid_y) {
                    place(grid, orientation, x, y);
                    if solve(grid, present_list, idx + 1, grid_x, grid_y, start_time, timeout) {
                        return true;
                    }
                    unplace(grid, orientation, x, y);
                }
            }
        }
    }

    false
}

fn can_fit_all_impl(
    presents: &Vec<Present>,
    region: &Region,
    timeout: Duration,
) -> bool {
    // Early exit: check if total area fits
    let mut total_area = 0;
    for (idx, &count) in region.presents.iter().enumerate() {
        total_area += count * presents[idx].squares();
    }
    let region_area = region.x_dim * region.y_dim;
    if total_area > region_area {
        return false;
    }

    let mut grid = vec![vec![false; region.x_dim]; region.y_dim];

    // Build list of presents with orientations (don't sort - maintain order)
    let mut present_list = Vec::new();
    for (idx, &count) in region.presents.iter().enumerate() {
        let orientations = presents[idx].all_orientations();
        for _ in 0..count {
            present_list.push((idx, orientations.clone()));
        }
    }

    let start_time = Instant::now();
    solve(&mut grid, &present_list, 0, region.x_dim, region.y_dim, start_time, timeout)
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    prob1_with_timeout(prob_file, Duration::from_secs(3))
}

fn prob1_with_timeout(prob_file: &str, timeout: Duration) -> Result<usize, Box<dyn std::error::Error>> {
    let (presents, regions) = data_init(prob_file)?;

    let mut n_fit = 0;
    for (i, region) in regions.iter().enumerate() {
        let start = Instant::now();
        let fits = can_fit_all_impl(&presents, region, timeout);
        let elapsed = start.elapsed();
        
        if elapsed > Duration::from_millis(500) {
            println!("Region {}: {} (took {:?})", i, if fits { "FITS" } else { "TIMEOUT/NO FIT" }, elapsed);
        }
        
        if fits {
            n_fit += 1;
        }
    }

    Ok(n_fit)
}

fn prob2(_prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());

    let total = prob1(&input_file)?;
    println!("Part 1 - total: {total}");

    let total = prob2(&input_file)?;
    println!("Part 2 - total: {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 2);
    }

    #[test]
    fn check_prob2() {
        // Part 2 is just a completion message, no actual problem to solve
        assert_eq!(prob2("sample.txt").unwrap(), 0);
    }
}
