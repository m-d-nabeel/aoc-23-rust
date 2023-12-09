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

