use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use crate::MapType::{FertilizerToWater, HumidityToLocation, LightToTemperature, SeedToSoil, SoilToFertilizer, TemperatureToHumidity, WaterToLight};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\s").unwrap();
}

fn main() {
    let iter = get_file_lines("E:/workspaces/advent-of-code-2023/advent-of-code-5/src/input.txt").into_iter();
    let mut maps: HashMap<MapType, HashSet<Map>> = HashMap::new();
    let mut seeds: Vec<Seed> = vec![];
    let mut current_type: MapType = SeedToSoil;
    for (index, line) in iter.enumerate() {
        if let Ok(ok_line) = line {
            if ok_line.is_empty() || ok_line.matches(r"\S").count() > 0 {
                continue;
            }

            if index == 0 {
                let mut start: i64 = -1;
                for (_, seed) in ok_line.split("seeds: ").last().unwrap().split(" ").enumerate() {
                    if start == -1 {
                        start = seed.parse::<i64>().unwrap();
                    } else {
                        let number = Some(seed.parse::<i64>().unwrap()).unwrap();
                        println!("adding seed to list: start {}, end {}", start, start + (number - 1));
                        seeds.push(Seed { start_index: start, end_index: start + (number - 1)});
                        start = -1;
                    }
                }

                continue;
            }

            let map_type = match ok_line.split(" ").take(1).last() {
                Some("seed-to-soil") => Some(SeedToSoil),
                Some("soil-to-fertilizer") => Some(SoilToFertilizer),
                Some("fertilizer-to-water") => Some(FertilizerToWater),
                Some("water-to-light") => Some(WaterToLight),
                Some("light-to-temperature") => Some(LightToTemperature),
                Some("temperature-to-humidity") => Some(TemperatureToHumidity),
                Some("humidity-to-location") => Some(HumidityToLocation),
                _ => None
            };
            if map_type.is_some() {
                current_type = map_type.unwrap();
                continue;
            }

            let existing_map = maps.iter_mut().filter(|x| x.0 == &current_type).last();
            let parsed_map: Map = parse_into_map(ok_line);
            if let Some((_, map)) = existing_map {
                map.insert(parsed_map);
            } else {
                maps.insert(current_type, HashSet::from([parsed_map]));
            }
        }
    }

    let mut processed: Vec<Seed> = vec![];
    let mut unprocessed: Vec<Seed> = seeds.clone();
    for map_type in [SeedToSoil, SoilToFertilizer, FertilizerToWater, WaterToLight, LightToTemperature, TemperatureToHumidity, HumidityToLocation] {
        'seed: while let Some(seed) = unprocessed.pop() {
            let stored_map_type = maps.get(&map_type).unwrap();
            'mapping: for this_map in stored_map_type.clone().into_iter() {
                let end = this_map.source + (this_map.range - 1);
                if seed.start_index > end && seed.end_index > end {
                    continue 'mapping;
                }

                if seed.start_index < this_map.source && seed.end_index < this_map.source {
                    continue 'mapping;
                }

                // right boundary outside
                if seed.end_index > end {
                    unprocessed.push(Seed { start_index: end + 1, end_index: seed.end_index });
                }

                // left boundary outside
                if seed.start_index < this_map.source {
                    unprocessed.push(Seed { start_index: seed.start_index, end_index: this_map.source - 1});
                }

                let out = Seed {
                    start_index: seed.start_index.max(this_map.source) + this_map.delta,
                    end_index: seed.end_index.min(end) + this_map.delta
                };
                processed.push(out);
                continue 'seed;
            }

            println!("reached a point where {:?} is not in any map for {:?}", seed, map_type);
            processed.push(seed);
        }

        unprocessed = processed.clone();
        processed = vec![];
    }

    println!("{}", unprocessed.clone().iter().map(|x| x.start_index).min().unwrap());
}

fn part_1() {
    let iter = get_file_lines("E:/workspaces/advent-of-code-2023/advent-of-code-5/src/input.txt").into_iter();
    let mut maps: HashMap<MapType, HashSet<Map>> = HashMap::new();
    let mut seeds: Vec<i64> = vec![];
    let mut current_type: MapType = MapType::SeedToSoil;
    for (index, line) in iter.enumerate() {
        if let Ok(ok_line) = line {
            if ok_line.is_empty() || ok_line.matches(r"\S").count() > 0 {
                println!("Skipping empty line");
                continue;
            }

            if index == 0 {
                for (index, seed) in ok_line.split("seeds: ").last().unwrap().split(" ").enumerate() {
                    seeds.push(seed.parse::<i64>().unwrap());
                }

                continue;
            }

            let map_type = match ok_line.split(" ").take(1).last() {
                Some("seed-to-soil") => Some(SeedToSoil),
                Some("soil-to-fertilizer") => Some(SoilToFertilizer),
                Some("fertilizer-to-water") => Some(FertilizerToWater),
                Some("water-to-light") => Some(WaterToLight),
                Some("light-to-temperature") => Some(LightToTemperature),
                Some("temperature-to-humidity") => Some(TemperatureToHumidity),
                Some("humidity-to-location") => Some(HumidityToLocation),
                _ => None
            };
            if map_type.is_some() {
                current_type = map_type.unwrap();
                continue;
            }

            let existing_map = maps.get_mut(&current_type);
            let parsed_map: Map = parse_into_map(ok_line);
            if let Some(map) = existing_map {
                map.insert(parsed_map);
            } else {
                maps.insert(current_type, HashSet::from([parsed_map]));
            }
        }
    }

    let mut locations : Vec<i64> = vec![];
    for seed in seeds {
        let mut mapping = seed;
        for map_type in [SeedToSoil, SoilToFertilizer, FertilizerToWater, WaterToLight, LightToTemperature, TemperatureToHumidity, HumidityToLocation] {
            let stored_map_type = maps.get(&map_type);
            if let Some(map) = stored_map_type {
                if let Some(found_map) = map.iter().filter(|&x| mapping <= x.source + 0.max(x.range - 1) && mapping >= x.source).last() {
                    mapping = mapping + found_map.delta;
                } else {
                    continue;
                }
            } else {
                panic!("unknown map type");
            }
        }

        locations.push(mapping);
    }

    println!("minimum {}", locations.into_iter().min().unwrap());
}

fn parse_into_map(lines: String) -> Map {
    let mut map: Map = Map::default();
    for (index, number) in lines.split(" ").enumerate() {
        match index {
            0 => map.destination = number.parse::<i64>().unwrap(),
            1 => map.source = number.parse::<i64>().unwrap(),
            2 => map.range = number.parse::<i64>().unwrap(),
            _ => panic!("unexpected number of items in map")
        }
    }

    map.delta = (map.destination - map.source) as i64;
    return map;
}

#[derive(Default, PartialEq, Eq, Hash, Debug, Clone)]
struct Map {
    source: i64,
    destination: i64,
    delta: i64,
    range: i64,
}

#[derive(Clone, Copy, Default, Debug)]
struct MappingRange {
    start: i64,
    end: i64,
}

#[derive(Clone, Debug)]
struct Seed {
    start_index: i64,
    // length: i64,
    end_index: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

pub fn get_file_lines<P: AsRef<Path>>(path: P) -> Lines<BufReader<File>> {
    let file = File::open(path);
    if let Ok(ok_file) = file {
        let reader = BufReader::new(ok_file);
        return reader.lines();
    }

    panic!("File not available");
}