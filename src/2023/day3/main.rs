use std::collections::{HashMap, HashSet};

static INPUT_PATH: &str = "src/2023/day3/input";

fn main() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines = input.split("\n");


    let puzzle = Puzzle::new(lines.collect::<Vec<_>>());
    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());
}

struct Puzzle {
    rows: Vec<Vec<char>>,
    row_len: usize,
    col_len: usize,
}

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Puzzle {
    fn new(lines: Vec<&str>) -> Self {
        let rows = lines.iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        Self {
            rows: rows.clone(),
            row_len: rows[0].len(),
            col_len: rows.len(),
        }
    }

    // Checks if the symbol at the given row and column is a symbol (not a digit nor a '.')
    fn check_for_symbol(&self, row: usize, col: usize) -> bool {
        let symbol = self.rows[row][col];
        symbol != '.' && !symbol.is_ascii_digit()
    }

    // Checks if a given row and column is adjacent to a symbol
    fn check_adjacent(&self, row: usize, col: usize) -> bool {
        let row: i32 = row as i32;
        let col: i32 = col as i32;

        for i in (row) - 1..=(row) + 1 {
            for j in col - 1..=col + 1 {
                if i == row && j == col {
                    continue;
                }

                if i < 0 || j < 0 || i >= self.col_len as i32 || j >= self.row_len as i32 {
                    continue;
                }

                if self.check_for_symbol(i as usize, j as usize) {
                    return true;
                }
            }
        }
        false
    }

    // returns the sum of numbers adjacent to a symbol
    fn part1(&self) -> u32 {
        let mut final_sum = 0;
        let mut multiplier = 1;
        let mut sum = 0;
        let mut valid_part_so_far = false;

        //go through each row and column in reverse order
        for row in (0..self.row_len).rev() {
            for col in (0..self.col_len).rev() {
                match self.rows[row][col] {
                    num @ '0'..='9' => {
                        sum += num.to_digit(10).unwrap() * multiplier;
                        multiplier *= 10;
                        valid_part_so_far |= self.check_adjacent(row, col);
                    }
                    _ => {
                        if sum != 0 && valid_part_so_far {
                            final_sum += sum;
                        }
                        sum = 0;
                        multiplier = 1;
                        valid_part_so_far = false;
                    }
                }
            }
        }

        final_sum
    }

    // Gets the adjacent positions of a given row and column
    fn get_adjacent_positions(&self, row: usize, col: usize) -> Vec<Coordinate> {
        let row: i32 = row as i32;
        let col: i32 = col as i32;

        let mut adjacent_symbols: Vec<Coordinate> = Vec::new();

        for i in (row) - 1..=(row) + 1 {
            for j in col - 1..=col + 1 {
                if i == row && j == col {
                    continue;
                }

                if i < 0 || j < 0 || i >= self.col_len as i32 || j >= self.row_len as i32 {
                    continue;
                }

                if self.rows[i as usize][j as usize] == '*' {
                    adjacent_symbols.push(Coordinate { row: i as usize, col: j as usize });
                }
            }
        }
        adjacent_symbols
    }

    // finds all the star symbols with 2 adjacent numbers. Returns the sum of the pair products
    fn part2(&self) -> usize {
        let mut star_symbol_map: HashMap<Coordinate, Vec<usize>> = HashMap::new();
        let mut multiplier = 1;
        let mut sum = 0;

        // let mut stars_so_far: Vec<Coordinate> = Vec::new();
        //should be a set since we don't want duplicates
        let mut stars_so_far: HashSet<Coordinate> = HashSet::new();

        //go through each row and column in reverse order
        for row in (0..self.row_len).rev() {
            for col in (0..self.col_len).rev() {
                match self.rows[row][col] {
                    num @ '0'..='9' => {
                        sum += num.to_digit(10).unwrap() * multiplier;
                        multiplier *= 10;
                        stars_so_far.extend(self.get_adjacent_positions(row, col));
                    }
                    _ => {
                        if sum != 0 {
                            //add the current number to the star symbol map
                            for star in &stars_so_far {
                                let star_symbols = star_symbol_map.entry(star.clone()).or_default();
                                star_symbols.push(sum as usize);
                            }
                        }
                        sum = 0;
                        multiplier = 1;
                        stars_so_far = HashSet::new();
                    }
                }
            }
        }
        println!("{:#?}", star_symbol_map);
        star_symbol_map.iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0] * v[1])
            .sum::<usize>()
    }
}