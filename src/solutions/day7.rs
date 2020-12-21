use helpers;
use std::collections::HashMap;
use regex::Regex;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-7-example-input.txt");
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut reverseRules: HashMap<String, Vec<String>> = HashMap::new();
    let rgx = Regex::new(r"(\d*) (\w* \w*) bags{0,1}[\.,]").unwrap();
    for line in lines {
        let splits = line.split(" bags contain ").collect::<Vec<&str>>();
        rules.insert(
            splits[0].to_string(),
            rgx.captures_iter(splits[1]).map(|capture| capture[2].to_string()).collect()
        );
    }
    println!("{:?}", rules);
}
