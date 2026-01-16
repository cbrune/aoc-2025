use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
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

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (presents, regions) = data_init(prob_file)?;

    // println!("presents: {presents:#?}");
    // println!("regions: {regions:#?}");

    let mut n_fit = 0;
    for region in &regions {
        let region_area = region.x_dim * region.y_dim;
        let mut presents_area = 0;
        for (i, count) in region.presents.iter().enumerate() {
            println!(
                "present[{i}].squares: {}, count: {count}",
                presents[i].squares()
            );
            presents_area += count * presents[i].squares();
        }
        println!("present_area: {presents_area}, region_area: {region_area}");
        if presents_area <= region_area {
            n_fit += 1;
        }
    }

    Ok(n_fit)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
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
        assert_eq!(prob2("sample2.txt").unwrap(), 2);
    }
}
