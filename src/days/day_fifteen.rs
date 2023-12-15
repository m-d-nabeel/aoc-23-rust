use std::fs;

pub fn day_fifteen_part_one() {
    fn calculate_value(item: &str) -> u32 {
        let mut current_value = 0;
        for ch in item.chars() {
            current_value += ch as u32;
            current_value = (current_value * 17) % 256;
        }
        current_value
    }

    let arr = file_handle("day_fifteen.txt");
    let mut lens_map: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];

    for item in arr {
        let to_erase = item.contains('-');
        if to_erase {
            if let Some((name, _)) = item.split_once('-') {
                let current_value = calculate_value(name);
                if let Some(ascii_vec) = lens_map.get_mut(current_value as usize) {
                    if let Some(index) = ascii_vec.iter().position(|(x, _)| x == name) {
                        ascii_vec.remove(index);
                    }
                }
            }
        } else {
            if let Some((name, value_str)) = item.split_once('=') {
                let value = value_str.parse::<u32>().unwrap();
                let current_value = calculate_value(name);
                if let Some(ascii_vec) = lens_map.get_mut(current_value as usize) {
                    if let Some((_, existing_value)) = ascii_vec.iter_mut().find(|(x, _)| x == name) {
                        *existing_value = value;
                    } else {
                        ascii_vec.push((name.to_string(), value));
                    }
                }
            }
        }
    }

    let result: u32 = lens_map.iter().enumerate().map(|(j, lens)| {
        lens.iter().enumerate().map(|(i, (_, num))| {
            (j + 1) as u32 * (i + 1) as u32 * num
        }).sum::<u32>()
    }).sum();

    println!("{:?}", result);
}

pub fn file_handle(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(",")
        .map(String::from)
        .collect()
}


// pub fn day_fifteen_part_one() -> u32 {
//     let arr = ["rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"];
//     let mut lens_map: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
//     arr.into_iter().map(|item| {
//         let mut current_value = 0;
//         let char_s = item.chars();
//         let char_s_clone = char_s.clone();
//         for ch in char_s_clone.into_iter() {
//             let ascii_value = ch as u32;
//             current_value += ascii_value;
//             current_value *= 17;
//             current_value %= 256;
//             print!("{}, ", current_value);
//         }
//         let mut to_erase = false;
//         if char_s.into_iter().nth(2).unwrap() == '-' {
//             to_erase = true;
//         }
//
//         let (name, value) = item
//             .split_once("=")
//             .map(|(fir, sec)| (fir.to_string(), sec.parse::<u32>().unwrap()))
//             .unwrap();
//
//         if to_erase {
//             let ascii_vec = lens_map.get_mut(current_value as usize).unwrap();
//             for (i, (x, _)) in ascii_vec.iter().enumerate() {
//                 if x == &name {
//                     ascii_vec.remove(i);
//                     break;
//                 }
//             }
//         } else {
//             let ascii_vec = lens_map.get_mut(current_value as usize).unwrap();
//             let mut found = false;
//             for (x, y) in ascii_vec.iter_mut() {
//                 if x == &name {
//                     found = true;
//                     *y = value;
//                     break;
//                 }
//             }
//             if !found {
//                 ascii_vec.push((name, value));
//             }
//         }
//         println!(" => {:?} ", current_value);
//         current_value
//     }).sum()
// }
//
// pub fn file_handle(filename: &str) -> Vec<String> {
//     std::fs::read_to_string(filename)
//         .expect("Something went wrong reading the file")
//         .split(",")
//         .map(|item| item.to_string())
//         .collect()
// }