use helpers;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-7-input.txt");
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut results: HashSet::<String> = HashSet::new();
    results.insert("shiny gold".to_string());
    let rgx = Regex::new(r"(\d*) (\w* \w*) bags{0,1}[\.,]").unwrap();
    for line in lines {
        let splits = line.split(" bags contain ").collect::<Vec<&str>>();
        rules.insert(
            splits[0].to_string(),
            rgx.captures_iter(splits[1]).map(|capture| capture[2].to_string()).collect()
        );
    }
    loop {
        let mut retry = false;
        'mid: for (key, xs) in rules.iter() {
            if results.contains(key) { continue 'mid; }
            for x in xs {
                if results.contains(x) {
                    results.insert(key.to_string());
                    retry = true;
                }
            }
        }

        if !retry { break; }
    }
    
    println!("Day 7, part 1: {:?}", results.len() - 1);
}
