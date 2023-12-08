use std::collections::{HashMap, HashSet};

use num::integer::lcm;

pub fn find_first_digit(temp_line: &mut String) -> (u32, u32) {
    for (i, ch) in temp_line.chars().enumerate() {
        if ch.is_digit(10) {
            return (i as u32, ch.to_digit(10).unwrap());
        }
    }
    (u32::MAX, 0)
}

pub fn find_last_digit(temp_line: &mut String) -> (u32, u32) {
    let temp_line: Vec<_> = temp_line.chars().rev().collect();
    for i in 0..temp_line.len() {
        if temp_line[i].is_digit(10) {
            return (i as u32, temp_line[i].to_digit(10).unwrap());
        }
    }
    (u32::MAX, 0)
}

pub fn day_one_part_two() -> u32 {
    let mut digit_map = HashMap::new();

    digit_map.insert("zero", 0);
    digit_map.insert("one", 1);
    digit_map.insert("two", 2);
    digit_map.insert("three", 3);
    digit_map.insert("four", 4);
    digit_map.insert("five", 5);
    digit_map.insert("six", 6);
    digit_map.insert("seven", 7);
    digit_map.insert("eight", 8);
    digit_map.insert("nine", 9);

    std::fs::read_to_string("day_one.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let modified_line = line.to_string();
            let (mut first_index, mut last_index) = (u32::MAX, u32::MAX);
            let (mut first_digit, mut last_digit) = (0, 0);
            for (k, v) in digit_map.iter() {
                let mut temp_line = modified_line.replace(k, &v.to_string());
                let (temp_first_index, digit1) = find_first_digit(&mut temp_line);
                let (temp_last_index, digit2) = find_last_digit(&mut temp_line);
                if temp_first_index < first_index {
                    first_index = temp_first_index;
                    first_digit = digit1;
                }
                if temp_last_index < last_index {
                    last_index = temp_last_index;
                    last_digit = digit2;
                }
            }
            first_digit * 10 + last_digit
        })
        .sum::<u32>()
}

pub fn day_three_part_one() -> u32 {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    std::fs::read_to_string("day_two.txt")
        .unwrap()
        .lines()
        .filter(|line| {
            let line = line.to_string();
            let (_, mut second_half) = line.split_once(":").unwrap();
            second_half = second_half.trim();
            let s: Vec<_> = second_half.split("; ").collect();
            for i in 0..s.len() {
                // println!("{:?}", s[i].split(", ").collect::<Vec<_>>());
                for ele in s[i].split(", ").collect::<Vec<_>>() {
                    let (n, color) = ele.split_once(" ").unwrap();
                    let cubes = n.parse::<u32>().unwrap();
                    if color == "green" {
                        if cubes > 13 {
                            return false;
                        }
                    }
                    if color == "red" {
                        if cubes > 12 {
                            return false;
                        }
                    }
                    if color == "blue" {
                        if cubes > 14 {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .map(|line| {
            let line = line.to_string();
            let (first_half, _) = line.split_once(":").unwrap();
            let (_, game_id) = first_half.split_once(" ").unwrap();
            game_id.parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

pub fn day_three_part_two() -> u32 {
    std::fs::read_to_string("day_two.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.to_string();
            let (_, mut second_half) = line.split_once(":").unwrap();
            second_half = second_half.trim();
            let s: Vec<_> = second_half.split("; ").collect();
            let (mut green, mut red, mut blue) = (0, 0, 0);
            for i in 0..s.len() {
                for ele in s[i].split(", ").collect::<Vec<_>>() {
                    let (n, color) = ele.split_once(" ").unwrap();
                    let cubes = n.parse::<u32>().unwrap();
                    if color == "green" {
                        green = green.max(cubes);
                    }
                    if color == "red" {
                        red = red.max(cubes);
                    }
                    if color == "blue" {
                        blue = blue.max(cubes);
                    }
                }
            }
            red * green * blue
        })
        .sum::<u32>()
}

// input_format = Card   1: 26 36 90  2 75 32  3 21 59 18 | 47 97 83 82 43  7 61 73 57  2 67 31 69 11 44 38 23 52 10 21 45 36 86 49 14
pub fn day_four_part_one() -> u32 {
    std::fs::read_to_string("day_four.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.to_string();
            let (_, second_half) = line.split_once(": ").unwrap();
            let (win_numbers, my_numbers) = second_half.split_once(" | ").unwrap();
            let win_list: HashSet<_> = win_numbers.split(" ").filter(|ch| ch != &"").collect();
            let my_nums: Vec<_> = my_numbers.split(" ").collect();
            let count = my_nums.iter().filter(|&ch| win_list.contains(ch)).count();
            if count == 0 {
                return 0;
            }
            2u32.pow((count - 1) as u32)
        })
        .sum::<u32>()
}

// pub fn day_four_part_two() -> u32 {
//     let mut cards_copy: Vec<u32> = vec![1; 10000];
//     let mut card_number = 1;
//     std::fs::read_to_string("day_four.txt")
//         .unwrap()
//         .lines()
//         .map(|line| {
//             let line = line.to_string();
//             let (_, second_half) = line.split_once(": ").unwrap();
//             let (win_numbers, my_numbers) = second_half.split_once(" | ").unwrap();
//             let win_list: HashSet<_> = win_numbers.split(" ").filter(|ch| ch != &"").collect();
//             let my_nums: Vec<_> = my_numbers.split(" ").collect();
//             let count = my_nums.iter().filter(|&ch| win_list.contains(ch)).count();

//             for i in (card_number + 1)..=(card_number + count) {
//                 cards_copy[i] += cards_copy[card_number];
//             }

//             card_number += 1;
//             cards_copy[card_number]
//         })
//         .sum::<u32>()
// }
pub fn day_four_part_two() -> u32 {
    let mut cards_copy: Vec<u32> = vec![1; 500];
    let mut card_number = 1;
    let ans;

    if let Ok(contents) = std::fs::read_to_string("day_four.txt") {
        ans = contents
            .lines()
            .map(|line| {
                let (win_numbers, my_numbers) = line
                    .split_once(": ")
                    .unwrap_or(("", ""))
                    .1
                    .split_once(" | ")
                    .unwrap_or(("", ""));
                let win_list: HashSet<_> =
                    win_numbers.split(" ").filter(|&num| num != "").collect();
                let my_nums: Vec<_> = my_numbers.split(" ").collect();
                let matching_count = my_nums.iter().filter(|&num| win_list.contains(num)).count();

                for i in (card_number + 1)..=(card_number + matching_count) {
                    cards_copy[i] += cards_copy[card_number];
                }

                card_number += 1;
                cards_copy[card_number]
            })
            .sum::<u32>()
    } else {
        eprintln!("Error reading file");
        return 1;
    }
    ans
}

pub fn day_five_part_one() -> u128 {
    let lines: Vec<_> = std::fs::read_to_string("day_five.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let index = 0;
    let seeds: Vec<_> = lines[index]
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|ch| ch.parse::<u128>().unwrap_or_default())
        .collect();

    seeds
        .into_iter()
        .map(|seed| {
            let mut index = index + 3;
            let mut soil = seed;
            let mut found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let seed_to_soil = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = seed_to_soil[0];
                let src_start = seed_to_soil[1];
                let rng_length = seed_to_soil[2];
                if src_start <= seed && (src_start + rng_length) >= seed && !found {
                    let diff = seed - src_start;
                    soil = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            let mut fertilizer = soil;
            found = false;
            index += 2;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let soil_to_fertilizer = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = soil_to_fertilizer[0];
                let src_start = soil_to_fertilizer[1];
                let rng_length = soil_to_fertilizer[2];
                if src_start <= soil && src_start + rng_length >= soil && !found {
                    let diff = soil - src_start;
                    fertilizer = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            index += 2;
            let mut water = fertilizer;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let fertilizer_to_water = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = fertilizer_to_water[0];
                let src_start = fertilizer_to_water[1];
                let rng_length = fertilizer_to_water[2];
                if src_start <= fertilizer && src_start + rng_length >= fertilizer && !found {
                    let diff = fertilizer - src_start;
                    water = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            index += 2;
            let mut light = water;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let water_to_light = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = water_to_light[0];
                let src_start = water_to_light[1];
                let rng_length = water_to_light[2];
                if src_start <= water && src_start + rng_length >= water && !found {
                    let diff = water - src_start;
                    light = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            index += 2;
            let mut temperature = light;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let light_to_temperature = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = light_to_temperature[0];
                let src_start = light_to_temperature[1];
                let rng_length = light_to_temperature[2];
                if src_start <= light && src_start + rng_length >= light && !found {
                    let diff = light - src_start;
                    temperature = dest_start + diff;
                    found = true;
                }
                index += 1;
            }

            index += 2;
            let mut humidity = temperature;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let temperature_to_humidity = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = temperature_to_humidity[0];
                let src_start = temperature_to_humidity[1];
                let rng_length = temperature_to_humidity[2];
                if src_start <= temperature && src_start + rng_length >= temperature && !found {
                    let diff = temperature - src_start;
                    humidity = dest_start + diff;
                    found = true;
                }
                index += 1;
            }

            index += 2;
            let mut location = humidity;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let humidity_to_location = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = humidity_to_location[0];
                let src_start = humidity_to_location[1];
                let rng_length = humidity_to_location[2];
                if src_start <= humidity && src_start + rng_length >= humidity && !found {
                    let diff = humidity - src_start;
                    location = dest_start + diff;
                    found = true;
                }
                index += 1;
            }

            // println!(
            //     "seed-{}, soil-{}, fertilizer-{}, water-{}, light-{}, temperature-{}, humidity-{}, location-{}",
            //     seed, soil, fertilizer, water, light, temperature, humidity, location
            // );
            location
        })
        .min()
        .unwrap()
}

pub fn day_five_part_two() -> u128 {
    let lines: Vec<_> = std::fs::read_to_string("day_five.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let index = 0;
    let seeds_pair: Vec<_> = lines[index]
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|ch| ch.parse::<u128>().unwrap_or_default())
        .collect();

    let mut seeds = Vec::new();

    for i in (0..seeds_pair.len()).step_by(2) {
        if let Some(&end) = seeds_pair.get(i + 1) {
            let start = seeds_pair[i];
            seeds.push((start, end));
        }
    }

    seeds
        .into_iter()
        .map(|(_, seed)| {
            let mut index = index + 3;
            let mut soil = seed;
            let mut found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let seed_to_soil = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = seed_to_soil[0];
                let src_start = seed_to_soil[1];
                let rng_length = seed_to_soil[2];
                if src_start <= seed && (src_start + rng_length) >= seed && !found {
                    let diff = seed - src_start;
                    soil = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            let mut fertilizer = soil;
            found = false;
            index += 2;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let soil_to_fertilizer = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = soil_to_fertilizer[0];
                let src_start = soil_to_fertilizer[1];
                let rng_length = soil_to_fertilizer[2];
                if src_start <= soil && src_start + rng_length >= soil && !found {
                    let diff = soil - src_start;
                    fertilizer = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            index += 2;
            let mut water = fertilizer;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let fertilizer_to_water = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = fertilizer_to_water[0];
                let src_start = fertilizer_to_water[1];
                let rng_length = fertilizer_to_water[2];
                if src_start <= fertilizer && src_start + rng_length >= fertilizer && !found {
                    let diff = fertilizer - src_start;
                    water = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            index += 2;
            let mut light = water;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let water_to_light = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = water_to_light[0];
                let src_start = water_to_light[1];
                let rng_length = water_to_light[2];
                if src_start <= water && src_start + rng_length >= water && !found {
                    let diff = water - src_start;
                    light = dest_start + diff;
                    found = true;
                }
                index += 1;
            }
            index += 2;
            let mut temperature = light;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let light_to_temperature = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = light_to_temperature[0];
                let src_start = light_to_temperature[1];
                let rng_length = light_to_temperature[2];
                if src_start <= light && src_start + rng_length >= light && !found {
                    let diff = light - src_start;
                    temperature = dest_start + diff;
                    found = true;
                }
                index += 1;
            }

            index += 2;
            let mut humidity = temperature;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let temperature_to_humidity = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = temperature_to_humidity[0];
                let src_start = temperature_to_humidity[1];
                let rng_length = temperature_to_humidity[2];
                if src_start <= temperature && src_start + rng_length >= temperature && !found {
                    let diff = temperature - src_start;
                    humidity = dest_start + diff;
                    found = true;
                }
                index += 1;
            }

            index += 2;
            let mut location = humidity;
            found = false;
            while let Some(line) = lines.get(index) {
                if line == "" {
                    break;
                }
                let humidity_to_location = line
                    .split(" ")
                    .map(|ch| ch.parse::<u128>().unwrap_or_default())
                    .collect::<Vec<_>>();
                let dest_start = humidity_to_location[0];
                let src_start = humidity_to_location[1];
                let rng_length = humidity_to_location[2];
                if src_start <= humidity && src_start + rng_length >= humidity && !found {
                    let diff = humidity - src_start;
                    location = dest_start + diff;
                    found = true;
                }
                index += 1;
            }

            // println!(
            //     "seed-{}, soil-{}, fertilizer-{}, water-{}, light-{}, temperature-{}, humidity-{}, location-{}",
            //     seed, soil, fertilizer, water, light, temperature, humidity, location
            // );
            location
        })
        .min()
        .unwrap()
}

// Time:        46     68     98     66
// Distance:   358   1054   1807   1080

// Time:      7  15   30
// Distance:  9  40  200

pub fn day_six_part_one() -> u128 {
    let times = vec![46, 68, 98, 66];
    let distances = vec![358, 1054, 1807, 1080];
    // let times = vec![7, 15, 30];
    // let distances = vec![9, 40, 200];
    let mut index = 0;
    times
        .into_iter()
        .map(|time| {
            let mut count = 0;
            for i in 0..=time {
                let can_travel = (time - i) * i;
                if can_travel > distances[index] {
                    count += 1;
                }
            }
            index += 1;
            count
        })
        .product()
}

pub fn day_six_part_two() -> u128 {
    let time: u128 = 46689866;
    let distance: u128 = 358105418071080;
    let mut start = 0;
    let mut end = 46689866;
    while start < time {
        let can_travel = (time - start) * start;
        if can_travel > distance {
            break;
        }
        start += 1;
    }
    while end > 0 {
        let can_travel = (time - end) * end;
        if can_travel > distance {
            break;
        }
        end -= 1;
    }
    end - start + 1
}


pub fn day_seven_part_one() {
    if let Ok(text_file) = std::fs::read_to_string("day_seven.txt") {
        let mut list: Vec<_> = text_file.lines().map(|line| {
            let tuple = line.split_once(" ").unwrap();
            let cards = tuple.0.to_string();
            let bet = tuple.1.parse::<u32>().unwrap();
            (cards, bet)
        }).collect();
        list.sort_by(|(card_a, _), (card_b, _)| {
            let mut card_a_map = HashMap::new();
            let mut card_b_map = HashMap::new();
            card_a.chars().for_each(|ch| {
                let count = card_a_map.entry(ch).or_insert(0);
                *count += 1;
            });
            card_b.chars().for_each(|ch| {
                let count = card_b_map.entry(ch).or_insert(0);
                *count += 1;
            });
            let card_a_value = get_value(card_a_map);
            let card_b_value = get_value(card_b_map);
            card_a_value.cmp(&card_b_value)
        });
        println!("{:?}", list);
    } else {
        eprintln!("Error reading file");
    }

    fn get_value(card_map: HashMap<char, u32>) -> u32 {
        let mut value = 0;
        let const_card_map: HashMap<char, u32> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);
        for (k, v) in card_map.iter() {
            value += 10u32.pow(*v) * const_card_map.get(k).unwrap();
        }
        value
    }
}

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
            println!("{:?}", source_list);
            println!("{:?}", count_array);

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
            println!("{:?}", count_array);
            return count_array.iter().cloned().fold(1u128, lcm);
        }
        Err(..) => {
            eprintln!("Error reading file");
        }
    }
    1u128
}