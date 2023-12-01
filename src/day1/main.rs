/**
Advent of Code 2023
Day 1, parts 1 & 2

Marcos Traverso, 12/1/2023
 **/

use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    //open the input file "input" and split into lines
    let input = std::fs::read_to_string("./src/day1/input").unwrap();
    let lines = input.split("\n");

    //for each line, grab the first digit found, then the last digit found in the string
    let mut total = 0;
    for line in lines {
        let first = line.find(char::is_numeric).unwrap();
        let last = line.rfind(char::is_numeric).unwrap();


        //combine the first and last digits (1st digit concatenated with last digit)
        let first_digit = line.chars().nth(first).unwrap();
        let last_digit = line.chars().nth(last).unwrap();

        let first_digit = first_digit.to_digit(10).unwrap();
        let last_digit = last_digit.to_digit(10).unwrap();
        let combined = first_digit * 10 + last_digit;

        total += combined;
    }

    total
}

fn part2() -> u32 {
    // Map of "numeric" phrases to their actual numeric value
    let phrase_map: HashMap<&str, u32> = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ].iter().cloned().collect();

    // List of phrases for iteration
    let phrases = phrase_map.keys().cloned().collect::<Vec<_>>();

    //open the input file "input" and split into lines
    let input = std::fs::read_to_string("./src/day1/input").unwrap();
    let lines = input.split("\n");

    let mut total = 0;
    for line in lines {
        let first = find_forward(line, &phrases).unwrap();
        let last = find_backward(line, &phrases).unwrap();

        let first_digit = phrase_map.get(first).unwrap();
        let last_digit = phrase_map.get(last).unwrap();

        let combined = first_digit * 10 + last_digit;
        total += combined;
    }

    total
}

fn find_forward<'a>(line: &'a str, phrases: &Vec<&'a str>) -> Option<&'a str> {
    //go letter by letter through the line and check if any of the phrases are found
    let mut letters_so_far = String::new();
    for c in line.chars() {
        letters_so_far.push(c);
        for phrase in phrases {
            if letters_so_far.contains(phrase) {
                return Some(phrase);
            }
        }
    }
    None
}

fn find_backward<'a>(line: &'a str, phrases: &Vec<&'a str>) -> Option<&'a str> {
    //go letter by letter through the line in reverse and check if any of the phrases are found
    let mut letters_so_far = String::new();
    for c in line.chars().rev() {
        letters_so_far.insert(0, c);
        for phrase in phrases {
            if letters_so_far.contains(phrase) {
                return Some(phrase);
            }
        }
    }
    None
}
