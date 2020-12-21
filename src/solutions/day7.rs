use helpers;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-7-input.txt");
    let mut results: HashSet::<String> = HashSet::new();
    let mut rules: HashMap::<String, Vec<(String, i64)>> = HashMap::new();
    let mut color_to_count: HashMap::<String, i64> = HashMap::new();
    results.insert("shiny gold".to_string());

    let rgx = Regex::new(r"(\d*) (\w* \w*) bags{0,1}[\.,]").unwrap();
    for line in lines {
        let splits = line.split(" bags contain ").collect::<Vec<&str>>();
        let mut vec: Vec<(String, i64)> = vec![];
        for capture in rgx.captures_iter(splits[1]) {
            vec.push((capture[2].to_string(), capture[1].parse::<i64>().unwrap()));
        }
        if vec.is_empty() { color_to_count.insert(splits[0].to_string(), 1); }
        rules.insert(
            splits[0].to_string(),
            vec
        );
    }

    loop {
        let mut retry = false;
        'mid: for (key, xs) in rules.iter() {
            if results.contains(key) { continue 'mid; }
            for x in xs {
                if results.contains(&x.0) {
                    results.insert(key.to_string());
                    retry = true;
                }
            }
        }

        if !retry { break; }
    }

    loop {
        let mut retry = false;
        'mid: for (key, xs) in rules.iter() {
            if color_to_count.contains_key(key) { continue 'mid; }
            let mut sum = 1;
            for x in xs {
                if !color_to_count.contains_key(&x.0) { continue 'mid; }
                sum += x.1 * color_to_count.get(&x.0).unwrap();
            }
            color_to_count.insert(key.to_string(), sum);
            retry = true;
        }

        if !retry { break; }
    }
    println!("Day 7, part 1: {:?}", results.len() - 1);
    println!("Day 7, part 2: {:?}", color_to_count.get("shiny gold").unwrap() - 1);
}

