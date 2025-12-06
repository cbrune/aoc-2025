use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct JoltageReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl JoltageReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }
}

impl Iterator for JoltageReader {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line = line.expect("Bad line in file");
            let mut line_str = line.as_str();
            let mut joltages = Vec::new();
            while line_str.len() > 0 {
                let digit;
                (digit, line_str) = line_str.split_at(1);
                let value = digit.parse().expect("Not an integer: {e}");
                joltages.push(value);
            }
            joltages
        })
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let joltage_reader = JoltageReader::new(prob_file)?;

    let mut total_joltage = 0;

    for joltages in joltage_reader {
        // println!("joltages: {joltages:?}");
        let n_joltages = joltages.len();
        let mut max_10s = (0, 0);

        // find the max for 10s digit -- skip the last joltage
        for i in 0..(n_joltages - 1) {
            if joltages[i] > max_10s.0 {
                max_10s = (joltages[i], i);
            }
        }
        // println!("max: pos {}: {}", max_10s.1, max_10s.0);

        // from this max position scan for the next max -- we are
        // guaranteed to have at least one more digit to the right of
        // the max 10s digit.
        let mut max_1s = 0;
        for i in (max_10s.1 + 1)..n_joltages {
            max_1s = std::cmp::max(max_1s, joltages[i]);
        }
        // println!("max_1s: {max_1s}");

        let max_joltage = max_10s.0 * 10 + max_1s;
        // println!("Max joltage: {max_joltage}");
        total_joltage += max_joltage;
    }

    Ok(total_joltage)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let joltage_reader = JoltageReader::new(prob_file)?;

    let mut total_joltage = 0;

    const N_JOLTS: usize = 12;

    for joltages in joltage_reader {
        let n_joltages = joltages.len();
        // println!("joltages: {n_joltages}: {joltages:?}");

        let mut max_jolt_pos = Vec::new();
        let mut end = N_JOLTS;
        let mut next_start = 0;
        while end > 0 {
            let end_range = n_joltages - (end - 1);
            // println!("end: {end}");

            // find the max for current digit position -- skip the trailing joltages
            let mut max = (0, 0);
            // println!("Checking range: {}-{}", next_start, end_range);
            for i in next_start..end_range {
                // println!("Checking joltage[{i}]: {}", joltages[i]);
                if joltages[i] > max.0 {
                    max = (joltages[i], i);
                }
            }
            // println!("Found max: {max:?}");
            max_jolt_pos.push(max);
            next_start = max.1 + 1;
            end -= 1;
        }

        // println!("max_jolt_pos: {max_jolt_pos:?}");
        let value = max_jolt_pos
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, val)| {
                acc + (10usize.pow((N_JOLTS - 1 - i) as u32) * val.0)
            });
        // println!("max_jolt value: {value}");
        total_joltage += value;
    }

    Ok(total_joltage)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let joltage = prob1("input.txt")?;
    println!("prob1: total joltage: {joltage}");
    let joltage = prob2("input.txt")?;
    println!("prob2: total joltage: {joltage}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 357);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 3121910778619);
    }
}
