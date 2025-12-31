use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct JunkBox {
    id: usize,
    location: (isize, isize, isize),
}

struct JunkBoxReader {
    count: usize,
    lines: io::Lines<io::BufReader<File>>,
}

impl JunkBoxReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { count: 0, lines })
    }
}

impl Iterator for JunkBoxReader {
    type Item = JunkBox;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line_str = line.expect("Bad string");
            let coord_strs: Vec<&str> = line_str.split(',').collect();
            if coord_strs.len() != 3 {
                panic!("Bad junkbox string: {line_str}");
            }
            let x = coord_strs[0].parse::<isize>().expect("Bad junkbox X coord");
            let y = coord_strs[1].parse::<isize>().expect("Bad junkbox Y coord");
            let z = coord_strs[2].parse::<isize>().expect("Bad junkbox Z coord");
            let item = JunkBox {
                id: self.count,
                location: (x, y, z),
            };
            self.count += 1;
            item
        })
    }
}

#[derive(Debug, Clone)]
struct JunkBoxPair {
    j1: JunkBox,
    j2: JunkBox,
    dist: f32,
}

impl std::cmp::PartialEq for JunkBoxPair {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl std::cmp::Eq for JunkBoxPair {}

impl std::cmp::PartialOrd for JunkBoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for JunkBoxPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist < other.dist {
            std::cmp::Ordering::Greater
        } else if self.dist > other.dist {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Circuits {
    circuits: Vec<HashSet<JunkBox>>,
    max: usize,
}

impl Circuits {
    fn new(max: usize) -> Self {
        Self {
            circuits: Default::default(),
            max,
        }
    }

    fn add(&mut self, pair: (JunkBox, JunkBox)) -> bool {
        let mut p0_circuit = None;
        let mut p1_circuit = None;
        for (i, circuit) in self.circuits.iter_mut().enumerate() {
            if circuit.contains(&pair.0) && circuit.contains(&pair.1) {
                // nothing to do
                // println!("Skipping pair: {pair:?}, already in {circuit:?}");
                return false;
            }
            if circuit.contains(&pair.0) {
                // println!("Adding jb:{:?} to circuit:{i}:{circuit:?}", pair.1);
                // let _ = circuit.insert(pair.1);
                // need_new_circuit = false;
                p0_circuit = Some(i);
            }
            if circuit.contains(&pair.1) {
                // println!("Adding jb:{:?} to circuit:{i}:{circuit:?}", pair.0);
                // let _ = circuit.insert(pair.0);
                // need_new_circuit = false;
                p1_circuit = Some(i);
            }
        }

        match (p0_circuit, p1_circuit) {
            (Some(i), None) => {
                // println!("Adding jb:{:?} to circuit:{i}", pair.1);
                let _ = self.circuits[i].insert(pair.1);
            }
            (None, Some(i)) => {
                // println!("Adding jb:{:?} to circuit:{i}", pair.0);
                let _ = self.circuits[i].insert(pair.0);
            }
            (Some(i), Some(j)) => {
                if i == j {
                    panic!("Both in same circuit already checked");
                }
                println!("Merging circuits ({i}, {j})");
                // move j into i
                let (lower, upper) = if i < j { (i, j) } else { (j, i) };
                let mut old = self.circuits.remove(upper);
                for jb in old.drain() {
                    let _ = self.circuits[lower].insert(jb);
                }
            }
            (None, None) => {
                println!("Adding new circuit: {pair:?}");
                let mut circuit = HashSet::new();
                circuit.insert(pair.0);
                circuit.insert(pair.1);
                self.circuits.push(circuit);
            }
        }

        if self.circuits.len() == 1 && self.circuits[0].len() == self.max {
            // found the maximal circuit
            println!("circuit.add(): found maximal circuit");
            true
        } else {
            false
        }
    }

    fn total(&self) -> usize {
        let mut sizes = Vec::new();
        // println!("n_circ: {}", self.circuits.len());
        for circuit in &self.circuits {
            // println!("circ-len: {}", circuit.len());
            sizes.push(circuit.len());
        }
        sizes.sort();
        sizes.reverse();
        // println!("sizes: {sizes:?}");
        sizes[0] * sizes[1] * sizes[2]
    }

    fn len(&self) -> usize {
        self.circuits.len()
    }
}

fn data_init(
    prob_file: &str,
) -> Result<(Vec<JunkBox>, BinaryHeap<JunkBoxPair>), Box<dyn std::error::Error>> {
    let junkbox_reader = JunkBoxReader::new(prob_file)?;

    let mut junkboxes = Vec::new();
    for junkbox in junkbox_reader {
        // println!("junkbox: {junkbox:?}");
        junkboxes.push(junkbox);
    }

    use std::collections::BinaryHeap;
    let mut junkbox_dists = BinaryHeap::new();

    for i in 0..(junkboxes.len() - 1) {
        let j1 = junkboxes[i].clone();
        for j in (i + 1)..junkboxes.len() {
            let j2 = junkboxes[j].clone();
            let x2 = ((j2.location.0 - j1.location.0) as f32).powi(2);
            let y2 = ((j2.location.1 - j1.location.1) as f32).powi(2);
            let z2 = ((j2.location.2 - j1.location.2) as f32).powi(2);
            let d2 = x2 + y2 + z2;
            let dist = d2.sqrt();
            //            println!(
            //                "j1,j2: ({j1:?},{j2:?}): x2: {x2}, y2: {y2}, z2: {z2}, d2: {d2}, dist: {dist}"
            //            );
            let pair = JunkBoxPair { j1, j2, dist };
            junkbox_dists.push(pair);
        }
    }

    Ok((junkboxes, junkbox_dists))
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (_junkboxes, mut junkbox_dists) = data_init(prob_file)?;
    let n_connections = if prob_file == "sample.txt" { 10 } else { 1000 };

    let mut circuits = Circuits::default();
    let mut i = n_connections;
    while junkbox_dists.len() > 0 && i > 0 {
        let jbp = junkbox_dists.pop().expect("Unexpected None entry");
        // println!("jbp: {jbp:?}");
        let _ = circuits.add((jbp.j1, jbp.j2));
        i -= 1;
    }

    let total = circuits.total();

    Ok(total)
}

fn remove_junkbox(junkboxes: &mut Vec<JunkBox>, junkbox: JunkBox) {
    for i in 0..junkboxes.len() {
        if junkboxes[i].id == junkbox.id {
            junkboxes.remove(i);
            break;
        }
    }
    // not found is OK and  expected ...
}

fn prob2(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (junkboxes, mut junkbox_dists) = data_init(prob_file)?;

    let mut junkbox_ids = HashSet::new();
    for jb in &junkboxes {
        junkbox_ids.insert(jb.id);
    }

    let mut circuits = Circuits::new(junkboxes.len());
    // pop pairs until all junkboxes are in _some_ circuit
    let mut dist = 0;
    while junkbox_ids.len() > 0 {
        let jbp = junkbox_dists.pop().expect("Unexpected None entry");
        // println!("jbp: {jbp:?}");
        if circuits.add((jbp.j1, jbp.j2)) {
            // found the maximal circuit
            dist = jbp.j1.location.0 * jbp.j2.location.0;
            break;
        }
        junkbox_ids.remove(&jbp.j1.id);
        junkbox_ids.remove(&jbp.j2.id);
        //        remove_junkbox(&mut junkboxes, jbp.j1);
        //        remove_junkbox(&mut junkboxes, jbp.j2);
    }
    println!("Found dist: {dist}");
    println!(
        "after: jb.len(): {}, jbd.len(): {}, circuits.len(): {}",
        junkboxes.len(),
        junkbox_dists.len(),
        circuits.len()
    );
    println!("circuits: {:?}", circuits);

    Ok(dist as usize)
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
        assert_eq!(prob1("sample.txt").unwrap(), 40);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample.txt").unwrap(), 25272);
    }
}
