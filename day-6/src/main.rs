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
                    println!("Found prob: {}, len {}, {op:?}", i, cols[i].len());
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
                println!("row: >{prob_str}<");
                println!("i:{i} : |{c}|");
                if c == '*' || c == '+' {
                    if in_op {
                        // push out previous
                        println!("Pushing out ({col_start}, {i})");
                        cols.push((col_start, i));
                    }
                    in_op = true;
                    col_start = i;
                    // last row, gather positions
                    // cols.push(i);
                    if c == '*' {
                        ops.push(Op::Multiply);
                    } else if c == '+' {
                        ops.push(Op::Add);
                    }
                }
            }
            // push out last col
            if in_op {
                println!("Pushing out last ({col_start}, {})", prob_str.len());
                cols.push((col_start, prob_str.len() + 1));
            }
            nrows += 1;
        }

        Ok((ops, nrows - 1, cols))
    }

    fn parse2(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (ops, nrows, col_widths) = Self::scan(path)?;
        let ncols = ops.len();
        println!("ops: {ops:?}, ncols: {ncols}, nrows: {nrows}, col_widths: {col_widths:?}");

        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        let mut cols = Vec::new();
        for i in 0..ncols {
            let mut col = Vec::new();
            for _ in 0..nrows {
                let width = col_widths[i].1 - col_widths[i].0 - 1;
                col.push(vec![0; width]);
            }
            cols.push(col);
        }

        for (row_idx, line) in lines.enumerate() {
            if row_idx == nrows {
                // oper rows
                break;
            }
            let problem_str = line?;
            println!("row: {row_idx}, {problem_str}");
            // println!("cols: {cols:?}");
            let mut col_idx = 0;
            let mut col_start = col_widths[col_idx].0;
            let mut col_end = col_widths[col_idx].1 - 1;
            for (i, c) in problem_str.chars().enumerate() {
                if i >= col_widths[col_idx].1 {
                    col_idx += 1;
                    col_start = col_widths[col_idx].0;
                    col_end = col_widths[col_idx].1 - 1;
                }
                println!("i: {i}, col_end: {col_end}");
                if i == col_end {
                    // should be separator space
                    if c != ' ' {
                        panic!("Unexpected char: {c} found");
                    }
                    continue;
                }

                let cell_idx = i - col_start;
                println!("i: {i}, col_idx: {col_idx}, row_idx: {row_idx}, cell_idx: {cell_idx}");
                println!("before: i: {i}, col_idx: {col_idx}: {:?}", cols[col_idx]);
                let col = &mut cols[col_idx];
                let cell = &mut col[row_idx];
                cell[cell_idx] = if c == ' ' {
                    0
                } else {
                    c.to_digit(10).expect("bad digit") as usize
                };
                println!("after : i: {i}, col_idx: {col_idx}: {:?}", cols[col_idx]);
            }
        }

        // mush the rows/cells into problem values
        let mut problems = Vec::new();
        for (i, col) in cols.iter().enumerate() {
            //            for (j, cell) in col.iter().enumerate() {
            println!("col: {i}: {:?}", col);
            let mut values = vec![0; col.len()];
            for k in 0..col.len() {
                let idx = col.len() - 1 - k;
                println!("col: {i}:{k}: {:?}", col[idx]);
                for (j, d) in col[idx].iter().enumerate() {
                    println!("j:{j}, k:{k}, d:{d}");
                    values[j] += 10usize.pow(k as u32) * d;
                }
            }
            problems.push(Problem {
                operands: values,
                op: ops[i],
            });
        }
        Ok(Problems(problems))
    }
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let problems = Problems::parse1(prob_file)?;
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

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let problems = Problems::parse2(prob_file)?;
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
