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
