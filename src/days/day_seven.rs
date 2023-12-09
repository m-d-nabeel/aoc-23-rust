use std::collections::HashMap;

#[allow(dead_code)]
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
