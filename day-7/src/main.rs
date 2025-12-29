use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum ManifoldCell {
    Empty,
    Start,
    Splitter,
}

#[derive(Debug)]
struct ManifoldLine {
    data: Vec<ManifoldCell>,
}

impl ManifoldLine {
    fn new(size: usize) -> Self {
        Self {
            data: vec![ManifoldCell::Empty; size],
        }
    }

    fn set(&mut self, idx: usize, value: ManifoldCell) {
        self.data[idx] = value;
    }

    fn find_start(&self) -> usize {
        let mut start = None;
        for (i, cell) in self.data.iter().enumerate() {
            if let ManifoldCell::Start = cell {
                start = Some(i);
                break;
            }
        }
        start.expect("Unable to find start index")
    }

    fn check(&self, beams: &[bool]) -> (usize, Vec<bool>) {
        let mut new_beams = vec![false; beams.len()];
        let mut splits = 0;
        for (i, cell) in self.data.iter().enumerate() {
            match cell {
                ManifoldCell::Empty => {
                    if !new_beams[i] {
                        new_beams[i] = beams[i];
                    }
                }
                ManifoldCell::Splitter => {
                    if beams[i] {
                        splits += 1;
                        if (i == 0) || (i == beams.len() - 1) {
                            panic!("Splitter found at bad index: {i}");
                        }
                        // left case -- check new beams
                        if !new_beams[i - 1] {
                            new_beams[i - 1] = true;
                        }
                        // right case -- check old beams
                        if !beams[i + 1] {
                            new_beams[i + 1] = true;
                        }
                    }
                }
                _ => panic!("Unexpected cell: {:?}", cell),
            }
        }
        (splits, new_beams)
    }
}

#[derive(Debug)]
struct Manifold {
    data: Vec<ManifoldLine>,
}

impl Manifold {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn add_entry(&mut self, entry: ManifoldLine) {
        self.data.push(entry);
    }

    fn compute_paths(&self) -> usize {
        let start_entry = &self.data[0];
        let width = start_entry.data.len();
        let start = start_entry.find_start();
        // println!("start: {start:?}");

        let mut beams = vec![0; width];
        beams[start] = 1;

        for entry in &self.data[1..] {
            let mut new_beams = vec![0; width];
            // let mut total = 0;
            for (i, cell) in entry.data.iter().enumerate() {
                // println!("prob2: {i}: top   : new_beams: {new_beams:?}");
                match cell {
                    ManifoldCell::Empty => {
                        new_beams[i] += beams[i];
                    }
                    ManifoldCell::Splitter => {
                        if beams[i] > 0 {
                            if (i == 0) || (i == beams.len() - 1) {
                                panic!("Splitter found at bad index: {i}");
                            }
                            // left case -- check new beams
                            new_beams[i - 1] += beams[i];
                            // right case -- check old beams
                            new_beams[i + 1] += beams[i];
                        }
                    }
                    _ => panic!("Unexpected cell: {:?}", cell),
                }
                // println!("prob2: {i}: bottom: new_beams: {new_beams:?}");
            }
            // println!("prob2: bottom: new_beams: {new_beams:?}");
            beams = new_beams;
            // for val in &beams {
            //     total += *val;
            // }
            // println!("prob: total: {total}");
        }

        // println!("prob2: beams: {beams:?}");

        let mut total = 0;
        for val in &beams {
            total += *val;
        }
        total
    }
}

struct ManifoldReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl ManifoldReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }
}

impl Iterator for ManifoldReader {
    type Item = ManifoldLine;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line_str = line.expect("Bad string");
            let mut entry = ManifoldLine::new(line_str.len());
            for (i, c) in line_str.chars().enumerate() {
                let cell = match c {
                    'S' => ManifoldCell::Start,
                    '^' => ManifoldCell::Splitter,
                    '.' => ManifoldCell::Empty,
                    _ => panic!("Unexpected input: {}/{}", i, c),
                };
                entry.set(i, cell);
            }
            entry
        })
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut manifold_entries = ManifoldReader::new(prob_file)?;

    let start_entry = manifold_entries.next().unwrap();
    let width = start_entry.data.len();
    let start = start_entry.find_start();
    println!("start: {start:?}");

    let mut beams = vec![false; width];
    beams[start] = true;
    let mut splits = 0;

    for entry in manifold_entries {
        // println!("entry: {entry:?}");
        let (new_splits, new_beams) = entry.check(&beams);
        // println!("new_splits: {new_splits}, new_beams:  {new_beams:?}");
        splits += new_splits;
        beams = new_beams;
    }

    Ok(splits)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let manifold_entries = ManifoldReader::new(prob_file)?;
    let mut manifold = Manifold::new();

    for entry in manifold_entries {
        // println!("entry: {entry:?}");
        manifold.add_entry(entry);
    }

    let paths = manifold.compute_paths();

    Ok(paths)
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
        assert_eq!(prob1("sample.txt").unwrap(), 21);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 40);
    }
}
