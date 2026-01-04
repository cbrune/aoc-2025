use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Device {
    id: String,
}

struct DeviceConnectionReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl DeviceConnectionReader {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let lines = io::BufReader::new(file).lines();
        Ok(Self { lines })
    }
}

impl Iterator for DeviceConnectionReader {
    type Item = (Device, Vec<Device>);

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let line_str = line.expect("Bad string");

            // source device
            let parts: Vec<&str> = line_str.split(':').collect();
            if parts.len() != 2 {
                panic!("Bad input line: {line_str}");
            }
            let device = Device {
                id: parts[0].to_string(),
            };

            // device list
            let parts: Vec<&str> = line_str.split(' ').collect();
            let mut outputs = Vec::new();
            for dev in &parts[1..] {
                outputs.push(Device {
                    id: dev.to_string(),
                });
            }
            (device, outputs)
        })
    }
}

fn data_init(
    prob_file: &str,
    head_str: &str,
) -> Result<(Option<(Device, Vec<Device>)>, HashMap<Device, Vec<Device>>), Box<dyn std::error::Error>>
{
    let device_connection_reader = DeviceConnectionReader::new(prob_file)?;

    let mut devices = HashMap::new();
    let mut head = None;
    let mut max_len = 0;
    let mut max_outputs = None;
    for (device, outputs) in device_connection_reader {
        // println!("device:{device:?}, outputs:{outputs:?}");
        if device.id == head_str {
            // head node
            head = Some((device.clone(), outputs.clone()));
        } else {
            devices.insert(device.clone(), outputs.clone());
        }
        if outputs.len() > max_len {
            max_len = outputs.len();
            max_outputs = Some((device, outputs));
        }
    }

    // Add "out" last
    let out = "out".to_string();
    devices.insert(Device { id: out }, Vec::new());

    println!("max_outputs: {max_outputs:?}");

    Ok((head, devices))
}

fn traverse1(node: (&Device, &Vec<Device>), map: &HashMap<Device, Vec<Device>>) -> usize {
    // println!("traverse: node:{node:?}");

    let mut paths = 0;
    for node in node.1 {
        if node.id == "out" {
            return 1;
        }
        let outputs = map.get(node).expect("node not found");
        paths += traverse1((node, outputs), map);
    }

    paths
}

fn prob1(prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (head, devices) = data_init(prob_file, "you")?;

    let head = head.expect("Empty head node");
    // println!("Head: {head:#?}");
    // println!("Devices: {devices:#?}");

    let paths = traverse1((&head.0, &head.1), &devices);

    Ok(paths)
}

#[derive(Debug, Clone, Default)]
struct CacheItem {
    found_dac: bool,
    found_fft: bool,
}

fn traverse2<'a>(
    current: &Device,
    cache: &mut HashMap<String, usize>,
    map: &HashMap<Device, Vec<Device>>,
    mut found_dac: bool,
    mut found_fft: bool,
    //    path_str: String,
) -> usize {
    // println!("t2: node:{in_node:?}");

    if current.id == "out" {
        if found_dac && found_fft {
            println!("Found both: start:{current:?}");
            return 1;
        }
    }

    let key = format!("{}{}{}", current.id, found_dac, found_fft);
    if let Some(history) = cache.get(&key) {
        println!(
            "t2:  cache hit:{}{}{}: {}",
            current.id, found_dac, found_fft, history
        );
        return *history;
    }

    let mut paths = 0;

    for node in map.get(current).expect("Node not found") {
        paths += traverse2(
            node,
            cache,
            map,
            found_dac || (node.id == "dac"),
            found_fft || (node.id == "fft"),
        );
    }
    cache.insert(key, paths);
    paths
}

fn prob2(mut prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    if prob_file == "sample.txt" {
        prob_file = "sample2.txt";
    }
    let (head, mut devices) = data_init(prob_file, "svr")?;
    let head = head.expect("Empty head node");
    let h2 = head.clone();

    devices.insert(h2.0, h2.1);

    let mut cache = HashMap::new();

    let paths = traverse2(&head.0, &mut cache, &devices, false, false);

    println!("p2: paths: {paths}");
    Ok(paths)
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
        assert_eq!(prob1("sample.txt").unwrap(), 5);
    }

    #[test]
    fn check_prob2() {
        assert_eq!(prob2("sample2.txt").unwrap(), 2);
    }
}
