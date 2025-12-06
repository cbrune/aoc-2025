use std::fs::File;
use std::io::{self, BufRead};

struct Grid {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();

        let cells: Vec<Vec<char>> = lines
            .map(|line| line.expect("Failed to read line").chars().collect())
            .collect();

        let rows = cells.len();
        let cols = if rows > 0 { cells[0].len() } else { 0 };

        Ok(Self { cells, rows, cols })
    }

    fn count_adjacent_rolls(&self, row: usize, col: usize) -> usize {
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        let mut count = 0;
        for (dr, dc) in directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_row < self.rows as isize
                && new_col >= 0 && new_col < self.cols as isize
            {
                let r = new_row as usize;
                let c = new_col as usize;
                if self.cells[r][c] == '@' {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_accessible_rolls(&self) -> usize {
        let mut accessible = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cells[row][col] == '@' {
                    let adjacent_count = self.count_adjacent_rolls(row, col);
                    if adjacent_count < 4 {
                        accessible += 1;
                    }
                }
            }
        }

        accessible
    }

    fn find_accessible_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cells[row][col] == '@' {
                    let adjacent_count = self.count_adjacent_rolls(row, col);
                    if adjacent_count < 4 {
                        positions.push((row, col));
                    }
                }
            }
        }

        positions
    }

    fn remove_rolls(&mut self, positions: &[(usize, usize)]) {
        for &(row, col) in positions {
            self.cells[row][col] = '.';
        }
    }

    fn count_total_removable_rolls(&mut self) -> usize {
        let mut total_removed = 0;

        loop {
            let accessible = self.find_accessible_positions();
            if accessible.is_empty() {
                break;
            }

            let count = accessible.len();
            total_removed += count;
            self.remove_rolls(&accessible);
        }

        total_removed
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());

    // Part 1
    let grid = Grid::from_file(&input_file)?;
    let accessible = grid.count_accessible_rolls();
    println!("Part 1 - Accessible rolls: {}", accessible);

    // Part 2
    let mut grid2 = Grid::from_file(&input_file)?;
    let total_removed = grid2.count_total_removable_rolls();
    println!("Part 2 - Total removable rolls: {}", total_removed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let grid = Grid::from_file("sample.txt").unwrap();
        assert_eq!(grid.count_accessible_rolls(), 13);
    }

    #[test]
    fn test_part2_sample() {
        let mut grid = Grid::from_file("sample.txt").unwrap();
        assert_eq!(grid.count_total_removable_rolls(), 43);
    }
}
