use std::{thread, sync::{Arc, RwLock}};

use fxhash::FxHashMap;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input(5);
    advent_of_code::answer(1, Some(484023871), part1(&input));
    advent_of_code::answer(2, Some(46294175), part2(&input));
}

fn part1(input: &String) -> i64 {
    let mut map: FxHashMap<&str, Vec<Vec<i64>>> = FxHashMap::default();
    let categories = input.split("\n\n").collect::<Vec<_>>();
    for category in categories {
        let lines = category.split(":").collect::<Vec<_>>();
        let numbers = match lines[0] {
            "seeds" => lines[1].split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>(),
            _ => lines[1].split(['\n', ' ']).filter(|&x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()
        };

        let mut nums = vec![];

        if lines[0] == "seeds" {
            nums.push(numbers.clone());
        } else {
            for i in (0..numbers.len()).step_by(3) {
                nums.push(vec![numbers[i], numbers[i+1], numbers[i+2]]);
            }
        }

        map.insert(lines[0], nums);
    }

    let mut min = 1000000000;
    let seeds = &map["seeds"][0];
    for seed in seeds {
        let soil = map["seed-to-soil map"].iter().find(|&v| seed >= &v[1] && seed <= &(&v[1] + &v[2])).map(|v| seed - v[1] + v[0]).unwrap_or(*seed);
        let fertilizer = map["soil-to-fertilizer map"].iter().find(|&v| soil >= v[1] && soil <= &v[1] + &v[2]).map(|v| soil - v[1] + v[0]).unwrap_or(soil);
        let water = map["fertilizer-to-water map"].iter().find(|&v| fertilizer >= v[1] && fertilizer <= &v[1] + &v[2]).map(|v| fertilizer - v[1] + v[0]).unwrap_or(fertilizer);
        let light = map["water-to-light map"].iter().find(|&v| water >= v[1] && water <= &v[1] + &v[2]).map(|v| water - v[1] + v[0]).unwrap_or(water);
        let temperature = map["light-to-temperature map"].iter().find(|&v| light >= v[1] && light <= &v[1] + &v[2]).map(|v| light - v[1] + v[0]).unwrap_or(light);
        let humidity = map["temperature-to-humidity map"].iter().find(|&v| temperature >= v[1] && temperature <= &v[1] + &v[2]).map(|v| temperature - v[1] + v[0]).unwrap_or(temperature);
        let location = map["humidity-to-location map"].iter().find(|&v| humidity >= v[1] && humidity <= &v[1] + &v[2]).map(|v| humidity - v[1] + v[0]).unwrap_or(humidity);
        if location <= min {
            min = location;
        }
    }

    min
}

fn part2(input: &String) -> i64 {
    let input = input.to_owned();
    let mut map: FxHashMap<String, Vec<Vec<i64>>> = FxHashMap::default();
    let categories = input.split("\n\n").collect::<Vec<_>>();
    for category in categories {
        let lines = category.split(":").collect::<Vec<_>>();
        let numbers = match lines[0] {
            "seeds" => lines[1].split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>(),
            _ => lines[1].split(['\n', ' ']).filter(|&x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()
        };

        let mut nums = vec![];

        if lines[0] == "seeds" {
            nums.push(numbers.clone());
        } else {
            for i in (0..numbers.len()).step_by(3) {
                nums.push(vec![numbers[i], numbers[i+1], numbers[i+2]]);
            }
        }

        map.insert(lines[0].to_owned(), nums);
    }

    let seed_ranges = &map["seeds"][0];

    let num_seed_ranges = seed_ranges.len();

    let seed_ranges = seed_ranges.clone();

    let seed_ranges_arc = Arc::new(RwLock::new(seed_ranges));
    let map_arc = Arc::new(RwLock::new(map));

    let mut handles = vec![];

    for i in (0..num_seed_ranges).step_by(2) {
        let seed_ranges_clone = Arc::clone(&seed_ranges_arc);
        let map_clone = Arc::clone(&map_arc);
        let handle = thread::spawn(move || {
            let seed_ranges = seed_ranges_clone.read().expect("RwLock poisoned");
            let map = map_clone.read().expect("RwLock poisoned");
            let mut min = 100000000000;
            for seed in seed_ranges[i]..seed_ranges[i]+seed_ranges[i+1] {
                let soil = map["seed-to-soil map"].iter().find(|&v| seed >= v[1] && seed <= &v[1] + &v[2]).map(|v| seed - v[1] + v[0]).unwrap_or(seed);
                let fertilizer = map["soil-to-fertilizer map"].iter().find(|&v| soil >= v[1] && soil <= &v[1] + &v[2]).map(|v| soil - v[1] + v[0]).unwrap_or(soil);
                let water = map["fertilizer-to-water map"].iter().find(|&v| fertilizer >= v[1] && fertilizer <= &v[1] + &v[2]).map(|v| fertilizer - v[1] + v[0]).unwrap_or(fertilizer);
                let light = map["water-to-light map"].iter().find(|&v| water >= v[1] && water <= &v[1] + &v[2]).map(|v| water - v[1] + v[0]).unwrap_or(water);
                let temperature = map["light-to-temperature map"].iter().find(|&v| light >= v[1] && light <= &v[1] + &v[2]).map(|v| light - v[1] + v[0]).unwrap_or(light);
                let humidity = map["temperature-to-humidity map"].iter().find(|&v| temperature >= v[1] && temperature <= &v[1] + &v[2]).map(|v| temperature - v[1] + v[0]).unwrap_or(temperature);
                let location = map["humidity-to-location map"].iter().find(|&v| humidity >= v[1] && humidity <= &v[1] + &v[2]).map(|v| humidity - v[1] + v[0]).unwrap_or(humidity);
                if location <= min {
                    min = location;
                }
            }
            min
        });

        handles.push(handle);
    }

    let mut min_total = 100000000000;
    for handle in handles {
        let min = handle.join().unwrap();
        if min <= min_total {
            min_total = min;
        }
    }

    min_total
}
