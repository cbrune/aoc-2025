use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, id: usize) -> bool {
        id >= self.start && id <= self.end
    }
}

struct DatabaseReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl DatabaseReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }

    fn parse_ranges(&mut self) -> Result<Vec<Range>, Box<dyn std::error::Error>> {
        let mut ranges = Vec::new();

        for line in &mut self.lines {
            let line = line?;
            let line = line.trim();

            // Stop at blank line
            if line.is_empty() {
                break;
            }

            // Parse range format: "start-end"
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range format: {}", line).into());
            }

            let start = parts[0].parse::<usize>()?;
            let end = parts[1].parse::<usize>()?;

            ranges.push(Range { start, end });
        }

        Ok(ranges)
    }

    fn parse_ingredient_ids(&mut self) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let mut ids = Vec::new();

        for line in &mut self.lines {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let id = line.parse::<usize>()?;
            ids.push(id);
        }

        Ok(ids)
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut reader = DatabaseReader::new(prob_file)?;
    let ranges = reader.parse_ranges()?;
    let ingredient_ids = reader.parse_ingredient_ids()?;

    let mut fresh_count = 0;

    for id in ingredient_ids {
        let is_fresh = ranges.iter().any(|range| range.contains(id));
        if is_fresh {
            fresh_count += 1;
        }
    }

    Ok(fresh_count)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut reader = DatabaseReader::new(prob_file)?;
    let mut ranges = reader.parse_ranges()?;

    // Sort ranges by start value
    ranges.sort_by_key(|r| r.start);

    // Merge overlapping or adjacent ranges
    let mut merged = Vec::new();
    if let Some(mut current) = ranges.first().copied() {
        for range in ranges.iter().skip(1) {
            // If ranges overlap or are adjacent (end + 1 >= start), merge them
            if current.end >= range.start.saturating_sub(1) {
                // Merge: extend current range to include this one
                current.end = current.end.max(range.end);
            } else {
                // No overlap: save current and start new
                merged.push(current);
                current = *range;
            }
        }
        merged.push(current);
    }

    // Sum up the sizes of all merged ranges
    let total: usize = merged.iter().map(|r| r.end - r.start + 1).sum();

    Ok(total)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());

    let fresh_count = prob1(&input_file)?;
    println!("Part 1 - Fresh ingredient IDs: {}", fresh_count);

    let total_fresh = prob2(&input_file)?;
    println!("Part 2 - Total fresh ingredient IDs: {}", total_fresh);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 3);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 14);
    }

    #[test]
    fn test_range_contains() {
        let range = Range { start: 3, end: 5 };
        assert!(!range.contains(1));
        assert!(!range.contains(2));
        assert!(range.contains(3));
        assert!(range.contains(4));
        assert!(range.contains(5));
        assert!(!range.contains(6));
    }
}
