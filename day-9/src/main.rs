// use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct TilePoint {
    id: usize,
    location: (usize, usize),
}

struct TilePointReader {
    count: usize,
    lines: io::Lines<io::BufReader<File>>,
}

impl TilePointReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { count: 0, lines })
    }
}

impl Iterator for TilePointReader {
    type Item = TilePoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line_str = line.expect("Bad string");
            let coord_strs: Vec<&str> = line_str.split(',').collect();
            if coord_strs.len() != 2 {
                panic!("Bad tile point string: {line_str}");
            }
            let x = coord_strs[0].parse::<usize>().expect("Bad X coord");
            let y = coord_strs[1].parse::<usize>().expect("Bad Y coord");
            let item = TilePoint {
                id: self.count,
                location: (x, y),
            };
            self.count += 1;
            item
        })
    }
}

fn data_init(prob_file: &str) -> Result<Vec<TilePoint>, Box<dyn std::error::Error>> {
    let tile_point_reader = TilePointReader::new(prob_file)?;

    let mut points = Vec::new();
    for tile_point in tile_point_reader {
        // println!("tp: {tile_point:?}");
        points.push(tile_point);
    }
    Ok(points)
}

fn delta(p0: usize, p1: usize) -> usize {
    // the delta is inclusive of the end point
    if p0 < p1 {
        p1 - p0 + 1
    } else {
        p0 - p1 + 1
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let points = data_init(prob_file)?;
    let mut max_area = 0;

    for i in 0..(points.len() - 1) {
        for j in (i + 1)..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];
            let delta_x = delta(p1.location.0, p2.location.0);
            let delta_y = delta(p1.location.1, p2.location.1);
            let area = delta_x * delta_y;
            // println!("Area: {area}, p1:{p1:?}, p2:{p2:?}");
            if area > max_area {
                println!("  Found new max: {area}");
                max_area = area;
            }
        }
    }

    Ok(max_area)
}

#[derive(Debug, Clone, Copy)]
struct Line {
    p0: TilePoint,
    p1: TilePoint,
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let points = data_init(prob_file)?;

    let mut lines = Vec::new();
    let p0 = points[0];
    let mut last_point = p0;
    for point in &points[1..] {
        // sanity
        if (last_point.location.0 != point.location.0)
            && (last_point.location.1 != point.location.1)
        {
            panic!("Point topology errror: {last_point:?}, {point:?}");
        }
        let line = Line {
            p0: last_point,
            p1: *point,
        };
        lines.push(line);
        last_point = *point;
    }
    // wrap around
    let line = Line {
        p0: last_point,
        p1: p0,
    };
    lines.push(line);

    println!("Lines: {lines:?}");

    let max_area = 0;
    Ok(max_area)
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
        assert_eq!(prob1("sample.txt").unwrap(), 50);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 25272);
    }
}
