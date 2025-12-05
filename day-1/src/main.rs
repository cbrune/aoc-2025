use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Dial {
    value: usize,
    landings: usize,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            value: 50,
            landings: 0,
        }
    }
}

impl Dial {
    const MAX_SIZE: usize = 100;
    fn left(&mut self, clicks: usize) {
        // subtraction
        self.landings += clicks / Self::MAX_SIZE;

        let net_clicks = clicks % Self::MAX_SIZE;
        if net_clicks <= self.value {
            self.value -= net_clicks;
            if self.is_zero() && net_clicks != 0 {
                // landed on zero after moving the dial
                self.landings += 1;
            }
        } else {
            if !self.is_zero() {
                // passed through zero to get here.  If we were at
                // zero to begin with then, we don't pass through zero
                // now.
                self.landings += 1;
            }
            self.value = Self::MAX_SIZE - (net_clicks - self.value);
        }
    }

    fn right(&mut self, clicks: usize) {
        // addition
        self.landings += (self.value + clicks) / Self::MAX_SIZE;
        self.value = (self.value + clicks) % Self::MAX_SIZE;
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn landings(&self) -> usize {
        self.landings
    }
}

#[derive(Debug)]
enum Op {
    Left(usize),
    Right(usize),
}

struct OpReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl OpReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }
}

impl Iterator for OpReader {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            // pop 1st character
            let line = line.expect("Bad string");
            // println!("Line: {line}");
            let (op_str, click_str) = line.split_at(1);
            let clicks: usize = click_str.parse().expect("Bad integer string: {click_str}");
            if op_str == "R" {
                Op::Right(clicks)
            } else if op_str == "L" {
                Op::Left(clicks)
            } else {
                panic!("Unknown operation: {op_str}");
            }
        })
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = std::env::args().nth(1).expect("Error: no file given");
    println!("Using file: {input_file}");

    let mut dial = Dial::default();

    let ops = OpReader::new(&input_file)?;

    for op in ops {
        // println!("Op: {op:?}");
        match op {
            Op::Left(clicks) => dial.left(clicks),
            Op::Right(clicks) => dial.right(clicks),
        }
        // println!("After dial: {dial:?}");
    }

    println!("Dial zero landings: {}", dial.landings());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_left_right() {
        let mut dial = Dial::default();

        assert!(!dial.is_zero());

        dial.right(5);
        assert!(!dial.is_zero());
        dial.left(55);
        assert!(dial.is_zero());

        dial.left(0);
        assert!(dial.is_zero());

        dial.right(0);
        assert!(dial.is_zero());

        dial.left(5);
        assert!(!dial.is_zero());
        dial.right(5);
        assert!(dial.is_zero());

        dial.right(205);
        assert!(!dial.is_zero());
        dial.left(205);
        assert!(dial.is_zero());

        dial.left(205);
        assert!(!dial.is_zero());
        dial.right(205);
        assert!(dial.is_zero());
    }

    #[test]
    fn check_land_on_zero_from_above() {
        let mut dial = Dial::default();

        // regular - we start at 50
        dial.left(50);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 1);

        // restore
        dial.right(50);

        // with wraps
        dial.left(250);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 4);
    }

    #[test]
    fn check_land_on_zero_from_below() {
        let mut dial = Dial::default();

        // regular - we start at 50
        dial.right(50);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 1);

        // restore
        dial.right(50);

        // with wraps
        dial.right(250);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 4);
    }

    #[test]
    fn check_start_and_land_on_zero() {
        let mut dial = Dial::default();

        // regular - we start at 50
        dial.right(50);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 1);

        // turning right
        dial.right(100);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 2);

        // turning right with multiple wraps
        dial.right(200);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 4);

        // turning left
        dial.left(100);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 5);

        // turning left with multiple wraps
        dial.left(200);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 7);
    }

    #[test]
    fn check_start_on_zero() {
        let mut dial = Dial::default();

        // regular - we start at 50
        dial.right(50);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 1);

        // right - don't wrap
        dial.right(25);
        assert!(!dial.is_zero());
        assert_eq!(dial.landings(), 1);

        // restore
        dial.left(25);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 2);

        // right - with wrap
        dial.right(125);
        assert!(!dial.is_zero());
        assert_eq!(dial.landings(), 3);

        // restore
        dial.left(25);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 4);

        // left - don't wrap
        dial.left(25);
        assert!(!dial.is_zero());
        assert_eq!(dial.landings(), 4);

        // restore
        dial.right(25);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 5);

        // left - with wrap
        dial.left(125);
        assert!(!dial.is_zero());
        assert_eq!(dial.landings(), 6);

        // restore
        dial.right(25);
        assert!(dial.is_zero());
        assert_eq!(dial.landings(), 7);
    }
}
