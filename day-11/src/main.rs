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

fn traverse2(
    dac_paths: usize,
    fft_paths: usize,
    node: (&Device, &Vec<Device>),
    map: &HashMap<Device, Vec<Device>>,
    mut found_dac: bool,
    mut found_fft: bool,
) -> usize {
    // println!("traverse: node:{node:?}");

    let mut paths = 0;
    for node in node.1 {
        if node.id == "out" {
            if found_dac && found_fft {
                println!("out: found both");
                return 1;
            } else {
                return 0;
            }
        }
        if node.id == "dac" {
            found_dac = found_dac || true;
            if found_dac && found_fft {
                // early exit
                println!("early exit");
                paths += dac_paths;
                continue;
            }
        } else if node.id == "fft" {
            found_fft = found_fft || true;
            if found_dac && found_fft {
                // early exit
                println!("early exit");
                paths += fft_paths;
                continue;
            }
        }
        let outputs = map.get(node).expect("node not found");
        paths += traverse2(
            dac_paths,
            fft_paths,
            (node, outputs),
            map,
            found_dac,
            found_fft,
        );
    }

    paths
}

fn traverse3(
    nodes: &Vec<usize>,
    lut: &Vec<Vec<usize>>,
    dac_id: usize,
    dac_paths: usize,
    fft_id: usize,
    fft_paths: usize,
    mut found_dac: bool,
    mut found_fft: bool,
) -> usize {
    let mut paths = 0;
    // println!("t3: inspecting nodes: {nodes:?}");
    for node in nodes {
        if *node == lut.len() - 1 {
            if found_dac && found_fft {
                println!("out: found both");
                return 1;
            } else {
                return 0;
            }
        }

        if *node == dac_id {
            found_dac = found_dac || true;

            if found_dac && found_fft {
                // early exit
                println!("early exit");
                paths += dac_paths;
                continue;
            }
        } else if *node == fft_id {
            found_fft = found_fft || true;

            if found_dac && found_fft {
                // early exit
                println!("early exit");
                paths += fft_paths;
                continue;
            }
        }

        let outputs = &lut[*node];
        paths += traverse3(
            outputs, lut, dac_id, dac_paths, fft_id, fft_paths, found_dac, found_fft,
        );
    }

    paths
}

fn traverse4(nodes: &Vec<usize>, lut: &Vec<Vec<usize>>) -> usize {
    let mut paths = 0;
    for node in nodes {
        if *node == lut.len() - 1 {
            return 1;
        }

        let outputs = &lut[*node];
        paths += traverse4(outputs, lut);
    }

    paths
}

fn prob2(mut prob_file: &str) -> Result<usize, Box<dyn std::error::Error>> {
    if prob_file == "sample.txt" {
        prob_file = "sample2.txt";
    }
    let (head, devices) = data_init(prob_file, "svr")?;

    // convert devices into array of usize with a LUT for name->index;
    let mut name_map = HashMap::new();
    let mut lut = Vec::new();
    for key in devices.keys() {
        println!("Adding key: {} for index: {}", key.id, lut.len());
        name_map.insert(&key.id, lut.len());
        lut.push(Vec::new());
    }
    let out = "out".to_string();
    name_map.insert(&out, lut.len());
    lut.push(Vec::new());

    for (key, val) in devices.iter() {
        let index = name_map.get(&key.id).unwrap();
        for output in val {
            let out_id = name_map.get(&output.id).unwrap();
            println!(
                "Adding output['{}'] = {} for source['{}'] = {}",
                output.id, out_id, key.id, index,
            );
            lut[*index].push(*out_id);
        }
    }

    println!("lut: {lut:?}");

    let head = head.expect("Empty head node");
    // println!("Head: {head:#?}");

    let mut head_node = Vec::new();
    for output in &head.1 {
        let out_id = name_map.get(&output.id).unwrap();
        head_node.push(*out_id);
    }

    let dac = "dac".to_string();
    let dac_id = name_map.get(&dac).unwrap();
    let fft = "fft".to_string();
    let fft_id = name_map.get(&fft).unwrap();

    println!("dac_id: {dac_id}");
    println!("fft_id: {fft_id}");

    // how many paths from "dac" -> "out"
    let mut lut_minus_dac = lut.clone();
    let dac_devices = lut_minus_dac.remove(*dac_id);
    println!("lut-len:{}, lut-dac-len:{}", lut.len(), lut_minus_dac.len());
    let dac_paths = traverse4(&dac_devices, &lut_minus_dac);
    println!("dac_paths: {dac_paths}");

    // how many paths from fft -> out
    let mut lut_minus_fft = lut.clone();
    let fft_devices = lut_minus_fft.remove(*fft_id);
    let fft_paths = traverse4(&fft_devices, &lut_minus_fft);
    println!("fft_paths: {fft_paths}");

    let paths = traverse3(
        &head_node, &lut, *dac_id, dac_paths, *fft_id, fft_paths, false, false,
    );

    /*
        let paths = traverse3(
            dac_paths,
            fft_paths,
            (&head.0, &head.1),
            &devices,
            false,
            false,
        );
    */
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
