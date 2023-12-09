pub fn day_nine() -> (i128, i128) {
    if let Ok(text_file) = std::fs::read_to_string("day_nine.txt") {
        let puzzle_input: Vec<_> = text_file
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|num_str| num_str.parse::<i128>().unwrap_or_default())
                    .collect::<Vec<_>>()
            })
            .collect();
        let result_end: i128 = puzzle_input.iter().map(|input| {
            let mut curr_list = input.clone();
            let mut next_list = vec![];
            let mut next_value = 0;
            loop {
                next_value += *curr_list.last().unwrap_or(&0);
                if curr_list.iter().all(|num| *num == 0) {
                    break;
                }
                for i in 1..curr_list.len() {
                    next_list.push(curr_list[i] - curr_list[i - 1]);
                }
                curr_list = next_list;
                next_list = vec![];
            }
            next_value
        })
            .sum();
        let result_start: i128 = puzzle_input.iter().map(|input| {
            let mut curr_list = input.clone();
            curr_list.reverse();
            let mut next_list = vec![];
            let mut next_value = 0;
            loop {
                next_value += *curr_list.last().unwrap_or(&0);
                if curr_list.iter().all(|num| *num == 0) {
                    break;
                }
                for i in 1..curr_list.len() {
                    next_list.push(curr_list[i] - curr_list[i - 1]);
                }
                curr_list = next_list;
                next_list = vec![];
            }
            next_value
        })
            .sum();
        (result_start, result_end)
    } else {
        println!("Error reading file");
        (1i128, 1i128)
    }
}