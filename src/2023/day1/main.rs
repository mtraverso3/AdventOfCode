/**
Advent of Code 2023
Day 1, parts 1 & 2
 **/

use std::collections::HashMap;
use fancy_regex::Regex;

static INPUT_PATH: &str = "src/2023/day1/input";

fn main() {
    let phrase_map_part1: HashMap<&str, u32> = [
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ].iter().cloned().collect();

    let phrase_map_part2: HashMap<&str, u32> = [
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9),
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ].iter().cloned().collect();


    println!("Part 1: {}", solve(phrase_map_part1));
    println!("Part 2: {}", solve(phrase_map_part2));
}

fn solve(phrase_map: HashMap<&str, u32>) -> u32 {
    // build RE of all phrases
    let re = phrase_map.keys().cloned().collect::<Vec<_>>().join("|");
    let re = Regex::new(&format!("(?=({}))", re)).unwrap();

    //open the input file "input" and split into lines
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines = input.split("\n");

    let mut total = 0;
    for line in lines {
        let matches = re.captures_iter(line).collect::<Result<Vec<_>, _>>().unwrap();

        //get the first and last match strings
        let first = matches.first().unwrap().get(1).unwrap().as_str();
        let last = matches.last().unwrap().get(1).unwrap().as_str();

        //get the first and last digits
        let first = phrase_map.get(first).unwrap();
        let last = phrase_map.get(last).unwrap();

        //combine the first and last digits (1st digit concatenated with last digit)
        let combined = first * 10 + last;

        total += combined;
    }

    total
}
