use std::collections::HashSet;

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

