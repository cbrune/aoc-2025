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

// Find all divisors of n (excluding n itself)
fn get_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    if n <= 1 {
        return divisors;
    }

    for i in 1..n {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

// Calculate how many times a pattern of length d repeats in a number with n digits
fn calculate_repetitions(n: usize, d: usize) -> usize {
    assert!(d > 0, "Pattern length must be greater than 0");
    n / d
}

// Calculate the multiplier for a repeating pattern
// For example, pattern "123" repeated 3 times = 123 * (1 + 1000 + 1000000) = 123123123
// multiplier = 1 + 10^pattern_length + 10^(2*pattern_length) + ... + 10^((repetitions-1)*pattern_length)
fn calculate_multiplier(pattern_length: usize, repetitions: usize) -> usize {
    let mut multiplier = 0;
    let base = 10usize.pow(pattern_length as u32);

    for i in 0..repetitions {
        multiplier += base.pow(i as u32);
    }

    multiplier
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
        // println!("Checking: {range:?}, n_digits: {n_digits}, split: {split}");

        let mut s0 = range.0;
        while s0 <= range.1 {
            let s_upper = s0 / split;
            let s_lower = s0 % split;
            let test_id = (s_upper * split) + s_upper;
            // println!("  s_upper: {s_upper}, s_lower: {s_lower}, split: {split}");
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

    //println!("Bad IDs: {bad_ids:#?}");
    let mut sum = 0;
    for id in bad_ids {
        sum += id;
    }

    println!("Bad ID Sum: {sum}");

    Ok(sum)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    use std::collections::HashSet;

    let range_sets = load_ranges(prob_file)?;
    let mut bad_ids = HashSet::new();

    for range in range_sets {
        let n_digits = n_digits(range.0) as usize;

        // Get all possible pattern lengths (divisors of n_digits)
        let divisors = get_divisors(n_digits);

        for pattern_length in divisors {
            let repetitions = calculate_repetitions(n_digits, pattern_length);

            // Must repeat at least twice
            if repetitions < 2 {
                continue;
            }

            let multiplier = calculate_multiplier(pattern_length, repetitions);

            // Find the range of patterns that could produce invalid IDs within our range
            // pattern_min: smallest pattern that could be in range
            // pattern_max: largest pattern that could be in range

            // The minimum pattern is either the smallest d-digit number (10^(d-1))
            // or the smallest pattern that when multiplied gives us >= range.0
            let min_pattern_for_digits = 10usize.pow((pattern_length as u32).saturating_sub(1));
            let pattern_min = std::cmp::max(
                min_pattern_for_digits,
                (range.0 + multiplier - 1) / multiplier, // ceiling division
            );

            // The maximum pattern is either the largest d-digit number (10^d - 1)
            // or the largest pattern that when multiplied gives us <= range.1
            let max_pattern_for_digits = 10usize.pow(pattern_length as u32) - 1;
            let pattern_max = std::cmp::min(max_pattern_for_digits, range.1 / multiplier);

            // Generate all invalid IDs for patterns in this range
            for pattern in pattern_min..=pattern_max {
                let invalid_id = pattern * multiplier;

                // Verify it's actually in the range
                if invalid_id >= range.0 && invalid_id <= range.1 {
                    bad_ids.insert(invalid_id);
                }
            }
        }
    }

    let sum: usize = bad_ids.iter().sum();
    println!("Part 2 - Bad ID Sum: {}", sum);
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prob1("input.txt")?;
    prob2("input.txt")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 1227775554);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 4174379265);
    }

    #[test]
    fn test_get_divisors() {
        assert_eq!(get_divisors(1), vec![]);
        assert_eq!(get_divisors(2), vec![1]);
        assert_eq!(get_divisors(4), vec![1, 2]);
        assert_eq!(get_divisors(6), vec![1, 2, 3]);
        assert_eq!(get_divisors(12), vec![1, 2, 3, 4, 6]);
    }

    #[test]
    fn test_calculate_repetitions() {
        assert_eq!(calculate_repetitions(6, 3), 2); // 6-digit number, 3-digit pattern = 2 reps
        assert_eq!(calculate_repetitions(9, 3), 3); // 9-digit number, 3-digit pattern = 3 reps
        assert_eq!(calculate_repetitions(10, 2), 5); // 10-digit number, 2-digit pattern = 5 reps
        assert_eq!(calculate_repetitions(7, 1), 7); // 7-digit number, 1-digit pattern = 7 reps
    }

    #[test]
    fn test_calculate_multiplier() {
        // Pattern "1" repeated 2 times = 11 = 1 * (1 + 10) = 1 * 11
        assert_eq!(calculate_multiplier(1, 2), 11);

        // Pattern "1" repeated 3 times = 111 = 1 * (1 + 10 + 100) = 1 * 111
        assert_eq!(calculate_multiplier(1, 3), 111);

        // Pattern "12" repeated 2 times = 1212 = 12 * (1 + 100) = 12 * 101
        assert_eq!(calculate_multiplier(2, 2), 101);

        // Pattern "123" repeated 3 times = 123123123 = 123 * (1 + 1000 + 1000000)
        assert_eq!(calculate_multiplier(3, 3), 1001001);

        // Pattern "12" repeated 5 times = 1212121212 = 12 * (1 + 100 + 10000 + 1000000 + 100000000)
        assert_eq!(calculate_multiplier(2, 5), 101010101);
    }

    #[test]
    fn test_invalid_id_generation() {
        // Verify we can generate invalid IDs correctly using pattern * multiplier
        // 11 = 1 * 11
        assert_eq!(1 * calculate_multiplier(1, 2), 11);

        // 999 = 9 * 111
        assert_eq!(9 * calculate_multiplier(1, 3), 999);

        // 123123 = 123 * 1001
        assert_eq!(123 * calculate_multiplier(3, 2), 123123);

        // 565656 = 56 * 10101
        assert_eq!(56 * calculate_multiplier(2, 3), 565656);
    }
}
