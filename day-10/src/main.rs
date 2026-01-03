// use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

use good_lp::{
    default_solver, variable, variables, Expression, ProblemVariables, Solution, SolverModel,
};
use minilp::{ComparisonOp, OptimizationDirection, Problem};

#[derive(Clone)]
struct Machine {
    on_state: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "on_state: {:#09b}, buttons: [", self.on_state)?;
        for button in &self.buttons {
            write!(f, "{:#09b}, ", button)?;
        }
        write!(f, "], joltages: {:?}", self.joltages)?;

        Ok(())
    }
}

impl Machine {
    fn activate(&self) -> usize {
        // makes no sense to press a button twice -> NOP
        let mut on_presses = Vec::new();
        for mask in 1..(1 << self.buttons.len()) {
            let mut state = 0;

            // println!("Trying mask: {mask:#b}");
            for i in 0..self.buttons.len() {
                if (mask & (1 << i)) == (1 << i) {
                    state ^= self.buttons[i];
                    // println!(
                    //     "After applying button[{i}]: {:#09b}, state: {state:#09b}",
                    //     self.buttons[i]
                    // );
                }
            }

            if state == self.on_state {
                on_presses.push((mask as usize).count_ones());
                continue;
            }
        }

        // println!("pressed: {on_presses:?}: machine: {self:?}");
        on_presses.sort();
        on_presses[0] as usize
    }

    fn joltage(&self) -> f64 {
        // minimize button presses to achieve desired joltages
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let mut vars = Vec::new();

        // add the variables
        for _button in &self.buttons {
            let var = problem.add_var(1.0, (0.0, f64::INFINITY));
            vars.push(var);
        }

        // add the constraints
        for (i, joltage) in self.joltages.iter().enumerate() {
            let mut equation = Vec::new();
            for (j, button) in self.buttons.iter().enumerate() {
                if (button & (1 << i)) == (1 << i) {
                    // this button counts towards this joltage
                    equation.push((vars[j], 1.0));
                }
            }
            // add constraint
            problem.add_constraint(&equation, ComparisonOp::Eq, *joltage as f64);
        }

        let solution = problem.solve().unwrap();
        println!("Solution: {}", solution.objective() as usize);
        for (i, var) in vars.iter().enumerate() {
            println!("  Sol[{i}]: {}", solution[*var]);
        }
        solution.objective()
    }

    fn joltage2(&self) -> f64 {
        // minimize button presses to achieve desired joltages
        let mut problem = ProblemVariables::new();
        let mut vars = Vec::new();

        // Create the variables and objective function
        for _button in &self.buttons {
            let var = problem.add(variable().integer().min(0));
            vars.push(var);
        }
        let objective: Expression = vars.iter().sum();
        let mut model = problem.minimise(objective).using(default_solver);

        // add the constraints
        for (i, joltage) in self.joltages.iter().enumerate() {
            let mut expression = Expression::with_capacity(self.buttons.len());
            for (j, button) in self.buttons.iter().enumerate() {
                if (button & (1 << i)) == (1 << i) {
                    // this button counts towards this joltage
                    expression += vars[j];
                }
            }
            // add constraint
            model.add_constraint(expression.eq(*joltage as u32));
        }

        let solution = model.solve().unwrap();
        let mut total = 0.;
        for (_i, var) in vars.iter().enumerate() {
            // println!("Solution[{i}]: {}", solution.value(*var));
            total += solution.value(*var);
        }

        total
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

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let machines = data_init(prob_file)?;
    let mut total = 0;

    for machine in &machines {
        total += machine.activate();
    }

    Ok(total)
}

fn prob2(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let machines = data_init(prob_file)?;
    let mut total = 0.;

    for machine in &machines {
        total += machine.joltage();
    }

    Ok(total)
}

fn prob3(prob_file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let machines = data_init(prob_file)?;
    let mut total = 0.;

    for machine in &machines {
        total += machine.joltage2();
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

    let total = prob3(&input_file)?;
    println!("Part 3 - total: {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_prob1() {
        assert_eq!(prob1("sample.txt").unwrap(), 7);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 33.);
    }

    #[test]
    fn check_prob3() {
        assert_eq!(prob3("sample.txt").unwrap(), 33.);
    }
}
