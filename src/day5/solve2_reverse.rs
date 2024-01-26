use std::io::{BufRead, BufReader};
use std::fs::File;

struct Map {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

#[derive(PartialEq)]
enum InputType {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn starts_with_digit(line: &str) -> bool {
    line.chars().next().map_or(false, |c| c.is_numeric())
}

fn create_map(line: &str) -> Map {
    let map: Vec<&str> = line.split(" ").collect();

    Map {
        dest_start: map[0].parse::<usize>().unwrap(),
        source_start: map[1].parse::<usize>().unwrap(),
        length: map[2].parse::<usize>().unwrap(),
    }
}

fn reverse_look_up_in_maps(maps: &[Map], look_up_number: usize) -> usize {
    let mut new_look_up_number = look_up_number;
    for map in maps {
        if look_up_number >= map.dest_start && look_up_number < map.dest_start + map.length {
            new_look_up_number = look_up_number - map.dest_start + map.source_start;
            break;
        }
    }
    new_look_up_number
}

pub fn solve() {
    // seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    let file = File::open("src/day5/almanac.txt").expect("💣");
    let reader = BufReader::new(file);

    let mut seed_params: Vec<Vec<usize>> = Vec::new();
    let mut seed_to_soil_maps: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer_maps: Vec<Map> = Vec::new();
    let mut fertilizer_to_water_maps: Vec<Map> = Vec::new();
    let mut water_to_light_maps: Vec<Map> = Vec::new();
    let mut light_to_temperature_maps: Vec<Map> = Vec::new();
    let mut temperature_to_humidity_maps: Vec<Map> = Vec::new();
    let mut humidity_to_location_maps: Vec<Map> = Vec::new();

    let mut current_input_type = InputType::Seeds;

    // seeds: 79 14 55 13
    for line in reader.lines() {
        let line = line.expect("Nuuuu! 💣");
        if line.trim().is_empty() {
            if current_input_type == InputType::Seeds {
                current_input_type = InputType::SeedToSoil;
                continue;
            } else if current_input_type == InputType::SeedToSoil {
                current_input_type = InputType::SoilToFertilizer;
                continue;
            } else if current_input_type == InputType::SoilToFertilizer {
                current_input_type = InputType::FertilizerToWater;
                continue;
            } else if current_input_type == InputType::FertilizerToWater {
                current_input_type = InputType::WaterToLight;
                continue;
            } else if current_input_type == InputType::WaterToLight {
                current_input_type = InputType::LightToTemperature;
                continue;
            } else if current_input_type == InputType::LightToTemperature {
                current_input_type = InputType::TemperatureToHumidity;
                continue;
            } else if current_input_type == InputType::TemperatureToHumidity {
                current_input_type = InputType::HumidityToLocation;
                continue;
            } else if humidity_to_location_maps.is_empty() {
                continue;
            } else {
                break;
            }
        }

        // seed-to-soil map:
        if current_input_type != InputType::Seeds && !starts_with_digit(&line) {
            continue
        };

        if current_input_type == InputType::Seeds {
            let seed_data = line.split(":").collect::<Vec<&str>>()[1].trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            // [[79, 14], [55, 13]]
            seed_params = seed_data.chunks(2).map(|chunk| chunk.to_vec()).collect::<Vec<_>>();
        } else if current_input_type == InputType::SeedToSoil {
            // [50, 98, 2]
            seed_to_soil_maps.push(create_map(&line));
        } else if current_input_type == InputType::SoilToFertilizer {
            // [0, 15, 37]
            soil_to_fertilizer_maps.push(create_map(&line));
        } else if current_input_type == InputType::FertilizerToWater {
            // [49, 53, 8]
            fertilizer_to_water_maps.push(create_map(&line));
        } else if current_input_type == InputType::WaterToLight {
            // [88, 18, 7]
            water_to_light_maps.push(create_map(&line));
        } else if current_input_type == InputType::LightToTemperature {
            // [45, 77, 23]
            light_to_temperature_maps.push(create_map(&line));
        } else if current_input_type == InputType::TemperatureToHumidity {
            // [0, 69, 1]
            temperature_to_humidity_maps.push(create_map(&line));
        } else if current_input_type == InputType::HumidityToLocation {
            // [60, 56, 37]
            humidity_to_location_maps.push(create_map(&line));
        }
    }


    let mut lowest_location_number = 0;

    'outer: loop {
        let seed = [
            &humidity_to_location_maps,
            &temperature_to_humidity_maps,
            &light_to_temperature_maps,
            &water_to_light_maps,
            &fertilizer_to_water_maps,
            &soil_to_fertilizer_maps,
            &seed_to_soil_maps
        ].iter()
        .fold(lowest_location_number, |look_up_number, maps| reverse_look_up_in_maps(maps, look_up_number));

        for seed_param in seed_params.clone() {
            if let [start, num] = seed_param[..] {
                if seed >= start && seed < start + num {
                    break 'outer;
                }
            }
        }

        lowest_location_number += 1;
    }

    println!("Answer: {}", lowest_location_number);
}
