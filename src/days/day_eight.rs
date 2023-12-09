use std::collections::HashMap;

use num::integer::lcm;

// LLR
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
pub fn day_eight_part_one() -> u32 {
    if let Ok(text_file) = std::fs::read_to_string("day_eight.txt") {
        let path = text_file.lines().nth(0).unwrap();
        let path: Vec<_> = path.chars().collect();
        let mut index = 2;
        let mut graph_left = HashMap::new();
        let mut graph_right = HashMap::new();
        loop {
            if let Some(line) = text_file.lines().nth(index) {
                let (source, destination) = line.trim().split_once(" = ").unwrap();
                let source = source.to_string();
                let mut destination = destination.to_string();
                destination.pop();
                destination.remove(0);
                let (left, right) = destination.split_once(", ").unwrap();
                graph_left.insert(source.clone(), left.to_string());
                graph_right.insert(source, right.to_string());
            } else {
                break;
            }
            index += 1;
        }
        let mut current = "AAA".to_string();
        let mut count = 0;
        loop {
            if current == "ZZZ" {
                return count;
            }
            let (left, right) = (graph_left.get(&current).unwrap(), graph_right.get(&current).unwrap());
            if path[(count % path.len() as u32) as usize] == 'L' {
                current = left.to_string();
            } else {
                current = right.to_string();
            }
            count += 1;
        }
    } else {
        eprintln!("Error reading file");
    }
    1u32
}

pub fn day_eight_part_two() -> u128 {
    match std::fs::read_to_string("day_eight.txt") {
        Ok(text_file) => {
            let path = text_file.lines().nth(0).unwrap();
            let path: Vec<_> = path.chars().collect();
            let mut index = 2;
            let mut graph_left = HashMap::new();
            let mut graph_right = HashMap::new();
            let mut source_list = vec![];
            loop {
                if let Some(line) = text_file.lines().nth(index) {
                    let (source, destination) = line.trim().split_once(" = ").unwrap();
                    let source = source.to_string();
                    if source.chars().nth(2).unwrap() == 'A' {
                        source_list.push(source.clone());
                    }
                    let mut destination = destination.to_string();
                    destination.pop();
                    destination.remove(0);
                    let (left, right) = destination.split_once(", ").unwrap();
                    graph_left.insert(source.clone(), left.to_string());
                    graph_right.insert(source, right.to_string());
                } else {
                    break;
                }
                index += 1;
            }
            let source_count = source_list.len();
            let mut count_array: Vec<u128> = vec![0; source_count];
            // println!("{:?}", source_list);
            // println!("{:?}", count_array);

            for i in 0..source_count {
                loop {
                    if source_list[i].chars().nth(2).unwrap() == 'Z' {
                        break;
                    }
                    let (left, right) = (graph_left.get(&source_list[i]).unwrap(), graph_right.get(&source_list[i]).unwrap());
                    if path[(count_array[i] % path.len() as u128) as usize] == 'L' {
                        source_list[i] = left.to_string();
                    } else {
                        source_list[i] = right.to_string();
                    }
                    count_array[i] += 1;
                }
            }
            // println!("{:?}", count_array);
            return count_array.iter().cloned().fold(1u128, lcm);
        }
        Err(..) => {
            eprintln!("Error reading file");
        }
    }
    1u128
}