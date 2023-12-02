static INPUT_PATH: &str = "src/2023/day2/input";

#[derive(Debug)]
#[derive(Clone)]
struct GameDraw {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Game {
    draws: Vec<GameDraw>,
    id: u32,
}


fn main() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    let lines = input.split("\n");

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        //parse the line: "Game 48: 8 green, 10 red; 6 green, 5 red; 12 green, 2 blue; 17 green, 5 red, 1 blue; 14 green, 3 blue, 16 red; 1 blue, 5 red"
        let mut parts = line.split(": ");

        //get the game id
        let id = parts.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();

        let mut game = Game {
            draws: Vec::new(),
            id,
        };

        //get the draws
        let draws = parts.next().unwrap().split("; ");

        for draw in draws {
            let mut draw_parts = draw.split(", ");


            let mut draw = GameDraw {
                blue: 0,
                red: 0,
                green: 0,
            };

            while let Some(draw_part) = draw_parts.next() {
                let draw_part = draw_part.split(" ");

                let color = draw_part.clone().nth(1).unwrap();
                let count = draw_part.clone().nth(0).unwrap().parse::<u32>().unwrap();

                match color {
                    "blue" => draw.blue = count,
                    "red" => draw.red = count,
                    "green" => draw.green = count,
                    _ => panic!("Unknown color: {}", color),
                }
            }
            game.draws.push(draw);
        }

        games.push(game);
    }

    // println!("{:#?}", games);
    println!("Part 1: {}", part1(games.clone()));
    println!("Part 2: {}", part2(games));
}

fn part1(games: Vec<Game>) -> u32 {
    //filter out the games that have a invalid draw
    let valid_games = games.iter().filter(|game| {
        for draw in &game.draws {
            if draw.blue > 14 || draw.red > 12 || draw.green > 13 {
                return false;
            }
        }
        true
    }).collect::<Vec<_>>();

    // sum of the ids of the valid games
    valid_games.iter().map(|game| game.id).sum::<u32>()
}

fn part2(games: Vec<Game>) -> u32 {
    // we calculate the max of each cube color for each game
    let mut sum_power = 0;
    for game in games {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for draw in &game.draws {
            if draw.blue > max_blue {
                max_blue = draw.blue;
            }
            if draw.red > max_red {
                max_red = draw.red;
            }
            if draw.green > max_green {
                max_green = draw.green;
            }
        }


        sum_power += max_blue * max_red * max_green;
    }

    sum_power
}