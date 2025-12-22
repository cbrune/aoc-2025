use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Problem {
    operands: Vec<usize>,
    op: Op,
}

#[derive(Debug, Clone)]
struct Problems(Vec<Problem>);

impl From<&str> for Problems {
    fn from(path: &str) -> Self {
        let mut problems = Vec::new();
        let mut cols = Vec::new();
        let file = File::open(path).expect("Unable to open path");
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            // split line by spaces
            let problem_str = line.expect("Bad string");
            let elems: Vec<_> = problem_str.split_ascii_whitespace().collect();
            if cols.len() == 0 {
                for _ in 0..elems.len() {
                    cols.push(Vec::new());
                }
            }
            if cols.len() != elems.len() {
                panic!(
                    "Unexpected: elems.len(): {}, cols.len(): {}",
                    elems.len(),
                    cols.len()
                );
            }
            for (i, elem) in elems.into_iter().enumerate() {
                if let Ok(val) = elem.parse::<usize>() {
                    cols[i].push(val);
                } else {
                    // try op
                    let op = if elem == "*" {
                        Op::Multiply
                    } else if elem == "+" {
                        Op::Add
                    } else {
                        panic!("Bad operator: {elem}");
                    };
                    // found entire problem
                    println!("Found prob: {}, len {}, {op:?}", i, cols[i].len());
                    problems.push(Problem {
                        operands: cols[i].clone(),
                        op,
                    });
                };
            }
        }
        Problems(problems)
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let problems = Problems::from(prob_file);
    println!("Found problems: {problems:?}");

    let mut total = 0;
    for problem in &problems.0 {
        total += match problem.op {
            Op::Add => problem.operands.iter().fold(0, |acc, x| acc + x),
            Op::Multiply => problem.operands.iter().fold(1, |acc, x| acc * x),
        };
    }

    Ok(total)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());

    let total = prob1(&input_file)?;
    println!("Part 1 - total: {total}");

    //    let total_fresh = prob2(&input_file)?;
    //    println!("Part 2 - Total fresh ingredient IDs: {}", total_fresh);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 4277556);
    }

    //    #[test]
    //    fn check_prob2() {
    //        assert_eq!(prob2("sample.txt").unwrap(), 14);
    //    }
}
