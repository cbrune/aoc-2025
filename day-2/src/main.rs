use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Range(usize, usize);

#[derive(Debug)]
struct RangeReader {
    lines: io::Lines<io::BufReader<File>>,
    range_line: Vec<Range>,
    current: usize,
}

impl RangeReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self {
            lines,
            range_line: Vec::new(),
            current: 0,
        })
    }
}

impl Iterator for RangeReader {
    type Item = Range;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.range_line.len() {
            // read another line
            match self.lines.next() {
                Some(Ok(line)) => {
                    let range_strings: Vec<_> = line.split(',').collect();
                    self.range_line.clear();
                    for range_string in &range_strings {
                        let values: Vec<_> = range_string.split('-').collect();
                        let start = values[0].parse().expect("Not an integer: {range_string}");
                        let end = values[1].parse().expect("Not an integer: {range_string}");
                        assert!(start <= end);
                        self.range_line.push(Range(start, end));
                    }
                    self.current = 0;
                }
                Some(_) => panic!("Bad line in file"),
                None => return None,
            }
        }

        let value = self.range_line[self.current];
        self.current += 1;
        Some(value)
    }
}

// Count the number of  base-10 digits in an integer
fn n_digits(n: usize) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut temp = n;
    while temp > 0 {
        temp /= 10;
        count += 1;
    }
    count
}

fn load_ranges(prob_file: &str) -> Result<Vec<Range>, Box<dyn std::error::Error>> {
    // println!("Using file: {prob_file}");
    let ranges = RangeReader::new(&prob_file)?;
    // println!("ranges: {ranges:?}");
    let mut range_sets = Vec::new();
    for range in ranges {
        // println!("range: {range:?}");
        let mut s0 = range.0;
        let e0 = range.1;
        while n_digits(s0) != n_digits(e0) {
            /*
                        println!(
                            "n_digits s0: {}, n_digits  e0: {}",
                            n_digits(s0),
                            n_digits(e0)
                        );
            */
            let s1 = s0;
            let e1 = 10usize.pow(n_digits(s0)) - 1;
            // println!("Adding range: {s1}, {e1}");
            assert_eq!(n_digits(s1), n_digits(e1));
            range_sets.push(Range(s1, e1));
            s0 = e1 + 1;
        }
        if n_digits(s0) == n_digits(e0) {
            range_sets.push(Range(s0, e0));
        }
    }

    // println!("range_sets: {range_sets:#?}");
    Ok(range_sets)
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let range_sets = load_ranges(prob_file)?;

    // At this point we have a set of ranges that all have the
    // same number of digits.

    let mut bad_ids = Vec::new();
    for range in range_sets {
        // skip if number of digits is odd
        let n_digits = n_digits(range.0);
        if (n_digits & 0x1) == 0x1 {
            println!("Skipping range with odd range: {range:?}");
            continue;
        }

        let split = 10usize.pow(n_digits / 2);
        println!("Checking: {range:?}, n_digits: {n_digits}, split: {split}");

        /*
        let s_upper = range.0 / split;
        let s_lower = range.0 % split;

        let e_upper = range.1 / split;
        let e_lower = range.1 % split;

        // upper bound
        let mut count = e_upper - s_upper;
        if s_lower <= e_lower {
            count += 1;
        }
        */

        let mut s0 = range.0;
        while s0 <= range.1 {
            let s_upper = s0 / split;
            let s_lower = s0 % split;
            let test_id = (s_upper * split) + s_upper;
            println!("  s_upper: {s_upper}, s_lower: {s_lower}, split: {split}");
            // does upper equal lower?
            if s_upper == s_lower {
                let bad_id = (s_upper * split) + s_lower;
                println!("bad_id: upper == lower: {bad_id}");
                bad_ids.push(bad_id);
            } else if (test_id >= s0) && (test_id <= range.1) {
                println!("bad_id: in range: {test_id}");
                bad_ids.push(test_id);
            }

            // only one possible bad ID per split
            // move to next split
            s0 = (s_upper + 1) * split; // lower is zero
        }
    }
    println!("Bad IDs: {bad_ids:#?}");
    let mut sum = 0;
    for id in bad_ids {
        sum += id;
    }
    println!("Bad ID Sum: {sum}");

    Ok(sum)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let _range_sets = load_ranges(prob_file)?;

    Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prob1("input.txt")?;
    prob2("sample.txt")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 1227775554);
    }
}
