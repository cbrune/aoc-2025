// use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

use geo::{Contains, Intersects};
use geo_types::{point, LineString, Polygon};
use inpoly::inpoly2;
use ndarray::{arr2, Array, Array2, ArrayView};

struct TilePointReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl TilePointReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }
}

impl Iterator for TilePointReader {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line_str = line.expect("Bad string");
            let coord_strs: Vec<&str> = line_str.split(',').collect();
            if coord_strs.len() != 2 {
                panic!("Bad tile point string: {line_str}");
            }
            let x = coord_strs[0].parse::<f64>().expect("Bad X coord");
            let y = coord_strs[1].parse::<f64>().expect("Bad Y coord");
            (x, y)
        })
    }
}

fn data_init(prob_file: &str) -> Result<Array2<f64>, Box<dyn std::error::Error>> {
    let tile_point_reader = TilePointReader::new(prob_file)?;

    let mut array = Array::zeros((0, 2));
    for (x, y) in tile_point_reader {
        // println!("tp: {tile_point:?}");
        array.push_row(ArrayView::from(&[x, y])).unwrap();
    }

    Ok(array)
}

fn delta(p0: f64, p1: f64) -> f64 {
    // the delta is inclusive of the end point
    if p0 < p1 {
        p1 - p0 + 1.
    } else {
        p0 - p1 + 1.
    }
}

fn prob1(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let array = data_init(prob_file)?;
    let mut max_area = 0.;
    //    println!("array: {array:?}");

    for i in 0..(array.nrows() - 1) {
        for j in (i + 1)..array.nrows() {
            let x0 = array[[i, 0]];
            let y0 = array[[i, 1]];
            let x1 = array[[j, 0]];
            let y1 = array[[j, 1]];
            let delta_x = delta(x0, x1);
            let delta_y = delta(y0, y1);
            let area = delta_x * delta_y;
            // println!("Area: {area}, p1:{p1:?}, p2:{p2:?}");
            if area > max_area {
                // println!("  Found new max: {area}");
                max_area = area;
            }
        }
    }

    Ok(max_area)
}

fn prob2(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let array = data_init(prob_file)?;
    let mut max_area = 0.;
    //    println!("array: {array:?}");

    //    let total_calcs = array.nrows() * (array.nrows() - 1);
    //    println!("Total calcs: {total_calcs}");
    let mut iter = 0;
    let mut areas = Vec::new();
    for i in 0..(array.nrows() - 1) {
        for j in (i + 1)..array.nrows() {
            //            if (iter % 1000) == 0 {
            //                println!("Calc: {iter}/{total_calcs}");
            //            }
            //            iter += 1;
            let x0 = array[[i, 0]];
            let y0 = array[[i, 1]];
            let x1 = array[[j, 0]];
            let y1 = array[[j, 1]];

            let delta_x = delta(x0, x1);
            let delta_y = delta(y0, y1);
            let area = delta_x * delta_y;
            areas.push((x0, y0, x1, y1, area));
        }
    }

    areas.sort_by(|a, b| {
        let (_, _, _, _, a1) = a;
        let (_, _, _, _, b1) = b;
        let a1 = *a1 as usize;
        let b1 = *b1 as usize;
        b1.cmp(&a1)
    });

    //println!("areas: {areas:?}");

    //    let total_calcs = areas.len();
    //    let mut iter = 0;
    //    println!("Total calcs2: {total_calcs}");
    for (x0, y0, x1, y1, area) in areas {
        //        if (iter % 1000) == 0 {
        //            println!("Calc: {iter}/{total_calcs}");
        //        }
        //        iter += 1;
        // test if rectangle defined by points is inside the boundary
        // create array of points to check
        let mut rect = Array::zeros((0, 2));
        rect.push_row(ArrayView::from(&[x0, y0])).unwrap();
        rect.push_row(ArrayView::from(&[x0, y1])).unwrap();
        rect.push_row(ArrayView::from(&[x1, y1])).unwrap();
        rect.push_row(ArrayView::from(&[x1, y0])).unwrap();

        // println!("rect1: ({x0}, {y0}), ({x1}, {y1})");
        // println!("rect: {rect:?}");

        let (inside, on_boundry) = inpoly2(&rect, &array, None, None);
        //            println!("Check: inside: {inside:?}");

        // proceed if all inside
        let inside2 = inside.iter().fold(true, |acc, e| acc && *e);
        // let boundry = on_boundry.iter().fold(false, |acc, e| acc || *e);
        //            println!("Final inside: {inside}");

        if inside2 {
            println!(
                "Found max area: {area} - ({x0}, {y0}), ({x1}, {y1}) - {inside}, {on_boundry}, {iter}"
            );
            max_area = area;
            break;
        }
    }

    Ok(max_area)
}

fn data_init2(prob_file: &str) -> Result<Vec<(f64, f64)>, Box<dyn std::error::Error>> {
    let tile_point_reader = TilePointReader::new(prob_file)?;

    let mut points = Vec::new();
    for (x, y) in tile_point_reader {
        // println!("tp: {tile_point:?}");
        points.push((x, y));
    }

    points.push(points[0]);
    Ok(points)
}

fn prob3(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let mut points = data_init2(prob_file)?;
    let mut max_area = 0.;

    let line_string: LineString<f64> = points.clone().into();
    //    println!("line_string: {line_string:?}");
    let poly = Polygon::new(line_string, vec![]);
    //    println!("poly: {poly:?}");

    //    let total_calcs = points.len() * (points.len() - 1);
    //    println!("Total calcs: {total_calcs}");
    //    let mut iter = 0;
    let mut areas = Vec::new();
    for i in 0..(points.len() - 1) {
        for j in (i + 1)..points.len() {
            //            if (iter % 1000) == 0 {
            //                println!("Calc: {iter}/{total_calcs}");
            //            }
            //            iter += 1;
            let (x0, y0) = points[i];
            let (x1, y1) = points[j];

            let delta_x = delta(x0, x1);
            let delta_y = delta(y0, y1);
            let area = delta_x * delta_y;
            areas.push((x0, y0, x1, y1, area));
        }
    }

    areas.sort_by(|a, b| {
        let (_, _, _, _, a1) = a;
        let (_, _, _, _, b1) = b;
        let a1 = *a1 as usize;
        let b1 = *b1 as usize;
        b1.cmp(&a1)
    });

    //    println!("areas: {areas:?}");

    //    let total_calcs = areas.len();
    let mut iter = 0;
    //    println!("Total calcs2: {total_calcs}");
    for (x0, y0, x1, y1, area) in areas {
        //        if (iter % 1000) == 0 {
        //            println!("Calc: {iter}/{total_calcs}");
        //        }
        iter += 1;
        // test if rectangle defined by points is inside the boundary
        // create array of points to check
        let mut rect = Vec::new();
        rect.push(point!(x: x0, y: y0));
        rect.push(point!(x: x0, y: y1));
        rect.push(point!(x: x1, y: y1));
        rect.push(point!(x: x1, y: y0));

        /*
                println!("rect: {rect:?}");

                for p in &rect {
                    let contains = poly.contains(p);
                    println!("contains: p:{p:?}, {contains}");
                    let intersects = poly.intersects(p);
                    println!("intersects: p:{p:?}, {intersects}");
                }
        */
        let inside = rect.iter().fold(true, |acc, e| acc && poly.intersects(e));

        if inside {
            println!("Found max area: {area} - ({x0}, {y0}), ({x1}, {y1}) - {inside}, {iter}");
            max_area = area;
            break;
        }
    }

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

    let total = prob3(&input_file)?;
    println!("Part 3 - total: {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 50.);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 24.);
    }

    #[test]
    fn check_prob3() {
        assert_eq!(prob3("sample.txt").unwrap(), 24.);
    }
}
