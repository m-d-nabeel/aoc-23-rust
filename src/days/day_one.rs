use std::collections::HashMap;

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