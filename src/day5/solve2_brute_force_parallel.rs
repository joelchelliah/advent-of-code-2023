use std::io::{BufRead, BufReader};
use std::fs::File;
use std::thread;
use std::sync::{Arc, Mutex};

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

fn look_up_in_maps(maps: &[Map], mut look_up_number: usize) -> usize {
    for map in maps {
        if look_up_number >= map.source_start && look_up_number <= map.source_start + map.length {
            let look_up_position = look_up_number - map.source_start;
            look_up_number = map.dest_start + look_up_position;
            break;
        }
    }

    look_up_number
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

    let mut seeds: Vec<usize> = Vec::new();
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
            // [79, .., 92, 55, .., 67]
            seeds = line.split(":").collect::<Vec<&str>>()[1].trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
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

    let seed_to_soil_maps = Arc::new(seed_to_soil_maps);
    let soil_to_fertilizer_maps = Arc::new(soil_to_fertilizer_maps);
    let fertilizer_to_water_maps = Arc::new(fertilizer_to_water_maps);
    let water_to_light_maps = Arc::new(water_to_light_maps);
    let light_to_temperature_maps = Arc::new(light_to_temperature_maps);
    let temperature_to_humidity_maps = Arc::new(temperature_to_humidity_maps);
    let humidity_to_location_maps = Arc::new(humidity_to_location_maps);

    let seeds = seeds.clone();
    let seed_params = seeds.chunks(2).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<usize>>>();
    let lowest_location_number = Arc::new(Mutex::new(std::usize::MAX));


    let mut handles = vec![];

    for seed_param in seed_params {
        let lowest_location_number = Arc::clone(&lowest_location_number);

        let seed_to_soil_maps = Arc::clone(&seed_to_soil_maps);
        let soil_to_fertilizer_maps = Arc::clone(&soil_to_fertilizer_maps);
        let fertilizer_to_water_maps = Arc::clone(&fertilizer_to_water_maps);
        let water_to_light_maps = Arc::clone(&water_to_light_maps);
        let light_to_temperature_maps = Arc::clone(&light_to_temperature_maps);
        let temperature_to_humidity_maps = Arc::clone(&temperature_to_humidity_maps);
        let humidity_to_location_maps = Arc::clone(&humidity_to_location_maps);


        if let [start, num] = seed_param[..] {
            let handle = thread::spawn(move || {
                for seed in (start)..(start + num) {
                    let location = [
                        &seed_to_soil_maps,
                        &soil_to_fertilizer_maps,
                        &fertilizer_to_water_maps,
                        &water_to_light_maps,
                        &light_to_temperature_maps,
                        &temperature_to_humidity_maps,
                        &humidity_to_location_maps
                    ].iter()
                    .fold(seed, |look_up_number, maps| look_up_in_maps(maps, look_up_number));

                    let mut lowest = lowest_location_number.lock().unwrap();
                    if location < *lowest {
                        *lowest = location;
                    }
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Answer: {}", *lowest_location_number.lock().unwrap());
}
