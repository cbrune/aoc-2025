use std::fmt::{Debug, Display};
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

/// The outer vector is columns
/// The inner vector is rows
#[derive(Clone)]
struct NumberBlock {
    data: Vec<Vec<Option<usize>>>,
}

impl NumberBlock {
    fn new(col_width: usize, n_rows: usize) -> Self {
        let mut data = Vec::new();

        for _ in 0..col_width {
            let rows = vec![None; n_rows];
            data.push(rows);
        }

        Self { data }
    }

    fn set(&mut self, row: usize, col: usize, val: usize) {
        let row_data = &mut self.data[col];
        row_data[row] = Some(val);
    }

    fn get(&self, row: usize, col: usize) -> Option<usize> {
        let row_data = &self.data[col];
        // println!("get: ({row}, {col}): {:?}", row_data[row]);
        row_data[row]
    }

    // convert column of digits into a number
    fn col_num(&self, col: usize) -> usize {
        let n_rows = self.data[col].len();
        let mut value = 0;
        let mut n_idx = 0;
        for i in 0..n_rows {
            let idx = n_rows - i - 1;
            if let Some(digit) = self.get(idx, col) {
                value += 10usize.pow(n_idx) * digit;
                n_idx += 1;
            }
        }
        value
    }

    fn operands(&self) -> Vec<usize> {
        let mut operands = Vec::new();
        for i in 0..self.data.len() {
            operands.push(self.col_num(i));
        }
        operands
    }
}

impl Display for NumberBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n_rows = self.data[0].len();
        for i in 0..n_rows {
            for j in 0..self.data.len() {
                write!(f, "{:?} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for NumberBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

impl Problems {
    fn parse1(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut problems = Vec::new();
        let mut cols = Vec::new();
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            // split line by spaces
            let problem_str = line?;
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
                    // println!("Found prob: {}, len {}, {op:?}", i, cols[i].len());
                    problems.push(Problem {
                        operands: cols[i].clone(),
                        op,
                    });
                };
            }
        }
        Ok(Problems(problems))
    }

    // returns a vector of the opers, number of rows, and vector of column widths
    fn scan(
        path: &str,
    ) -> Result<(Vec<Op>, usize, Vec<(usize, usize)>), Box<dyn std::error::Error>> {
        let mut ops = Vec::new();
        let mut cols = Vec::new();
        let mut nrows = 0;
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            let prob_str = line?;
            // go char by char
            let mut in_op = false;
            let mut col_start = 0;
            for (i, c) in prob_str.chars().enumerate() {
                // println!("row: >{prob_str}<");
                // println!("i:{i} : |{c}|");
                if c == '*' || c == '+' {
                    // last row, gather positions
                    if in_op {
                        // push out previous
                        // println!("Pushing out ({col_start}, {i})");
                        cols.push((col_start, i));
                    }
                    in_op = true;
                    col_start = i;
                    if c == '*' {
                        ops.push(Op::Multiply);
                    } else if c == '+' {
                        ops.push(Op::Add);
                    }
                }
            }
            // push out last col
            if in_op {
                // println!("Pushing out last ({col_start}, {})", prob_str.len());
                cols.push((col_start, prob_str.len() + 1));
            }
            nrows += 1;
        }

        Ok((ops, nrows - 1, cols))
    }

    fn parse2(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (ops, nrows, col_widths) = Self::scan(path)?;
        let ncols = ops.len();
        // println!("ops: {ops:?}, ncols: {ncols}, nrows: {nrows}, col_widths: {col_widths:?}");

        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        let mut cols = Vec::new();
        for i in 0..ncols {
            let col = NumberBlock::new(col_widths[i].1 - col_widths[i].0 - 1, nrows);
            cols.push(col);
        }

        for (row_idx, line) in lines.enumerate() {
            if row_idx == nrows {
                // oper rows
                break;
            }
            let problem_str = line?;
            // println!("row: {row_idx}, {problem_str}");
            let mut col_idx = 0;
            let mut col_start = col_widths[col_idx].0;
            let mut col_end = col_widths[col_idx].1 - 1;
            for (i, c) in problem_str.chars().enumerate() {
                if i >= col_widths[col_idx].1 {
                    col_idx += 1;
                    col_start = col_widths[col_idx].0;
                    col_end = col_widths[col_idx].1 - 1;
                }
                if i == col_end {
                    // should be separator space
                    if c != ' ' {
                        panic!("Unexpected char: {c} found");
                    }
                    continue;
                }

                let cell_idx = i - col_start;
                // println!("i: {i}, col_idx: {col_idx}, row_idx: {row_idx}, cell_idx: {cell_idx}");
                // println!("before: i: {i}, col_idx: {col_idx}: {}", cols[col_idx]);
                if c != ' ' {
                    let col = &mut cols[col_idx];
                    let val = c.to_digit(10).expect("bad digit") as usize;

                    col.set(row_idx, cell_idx, val);
                }
                // println!("after : i: {i}, col_idx: {col_idx}: {}", cols[col_idx]);
            }
        }

        // println!("cols: {:?}", cols);
        // mush the rows/cells into problem values
        let mut problems = Vec::new();
        for (i, col) in cols.iter().enumerate() {
            let operands = col.operands();
            problems.push(Problem {
                operands,
                op: ops[i],
            });
        }
        // println!("Problems: {:?}", problems);
        Ok(Problems(problems))
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let problems = Problems::parse1(prob_file)?;
    // println!("Found problems: {problems:?}");

    let mut total = 0;
    for problem in &problems.0 {
        total += match problem.op {
            Op::Add => problem.operands.iter().fold(0, |acc, x| acc + x),
            Op::Multiply => problem.operands.iter().fold(1, |acc, x| acc * x),
        };
    }

    Ok(total)
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let problems = Problems::parse2(prob_file)?;
    // println!("Found problems: {problems:?}");

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

    let total = prob2(&input_file)?;
    println!("Part 2 - total: {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 4277556);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 3263827);
    }
}
