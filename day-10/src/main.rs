// use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Machine {
    on_state: usize,
    state: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "on_state: {:#09b}, state: {:#09b}, buttons: [",
            self.on_state, self.state
        )?;
        for button in &self.buttons {
            write!(f, "{:#09b}, ", button)?;
        }
        write!(f, "], joltages: {:?}", self.joltages)?;

        Ok(())
    }
}

struct MachineReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl MachineReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }
}

impl Iterator for MachineReader {
    type Item = Machine;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line_str = line.expect("Bad string");
            let parts: Vec<&str> = line_str.split(' ').collect();

            // light states
            let mut on_state = 0;
            for (i, c) in parts[0].chars().enumerate() {
                match c {
                    '[' => {
                        if i != 0 {
                            panic!("Unexpected '['");
                        }
                    }
                    ']' => {
                        // done with number of lights
                        // number of lights = i - 1
                    }
                    '.' => {
                        // already a zero
                    }
                    '#' => {
                        on_state |= 0x1 << (i - 1);
                    }
                    x => {
                        panic!("Unexpected lights input: {x}");
                    }
                }
            }

            // buttons and joltages
            let mut buttons = Vec::new();
            let mut joltages = Vec::new();
            for in_str in &parts[1..] {
                if in_str.starts_with("(") {
                    // button definition
                    let mut button_str = in_str.to_string();
                    let _ = button_str.pop();
                    let _ = button_str.remove(0);
                    let button_strs: Vec<&str> = button_str.split(',').collect();
                    let mut button_mask = 0;
                    for light in &button_strs {
                        let button = light.parse::<usize>().expect("Bad light number");
                        button_mask |= 0x1 << button;
                    }
                    buttons.push(button_mask);
                } else if in_str.starts_with("{") {
                    // joltages
                    let mut joltage_str = in_str.to_string();
                    let _ = joltage_str.pop();
                    let _ = joltage_str.remove(0);
                    let joltage_strs: Vec<&str> = joltage_str.split(',').collect();
                    for jolt_str in &joltage_strs {
                        let joltage = jolt_str.parse::<usize>().expect("Bad joltage number");
                        joltages.push(joltage);
                    }
                } else {
                    panic!("Unexpected input str: {in_str}");
                }
            }

            Machine {
                on_state,
                state: 0,
                buttons,
                joltages,
            }
        })
    }
}

fn data_init(prob_file: &str) -> Result<Vec<Machine>, Box<dyn std::error::Error>> {
    let machine_reader = MachineReader::new(prob_file)?;

    let mut machines = Vec::new();
    for machine in machine_reader {
        machines.push(machine);
    }

    Ok(machines)
}

fn prob1(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let machines = data_init(prob_file)?;
    let mut total = 0.;

    println!("machines: {machines:#?}");

    Ok(total)
}

fn prob2(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let machines = data_init(prob_file)?;
    let mut total = 0.;

    println!("machines: {machines:#?}");

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
        assert_eq!(prob1("sample.txt").unwrap(), 50.);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 24.);
    }
}
