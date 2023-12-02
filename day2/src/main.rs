#[derive(Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_line(line: &str) -> Game {
    let id = line
        .split(":")
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let rounds = line
        .split(":")
        .last()
        .unwrap()
        .split(";")
        .map(|round| {
            let components = round
                .split(",")
                .map(|round| round.trim())
                .collect::<Vec<&str>>();

            let mut green = 0;
            let mut blue = 0;
            let mut red = 0;

            for component in components {
                let split_component: Vec<&str> = component.split(" ").collect();
                let value = split_component[0].parse::<u32>().unwrap();
                let color = split_component[1];
                match color {
                    "red" => red = u32::max(red, value),
                    "green" => green = u32::max(green, value),
                    "blue" => blue = u32::max(blue, value),
                    _ => panic!("Invalid color"),
                }
            }

            Round { red, blue, green }
        })
        .collect::<Vec<Round>>();

    Game { id, rounds }
}

fn main() {
    // read lines from input.txt
    let lines = std::fs::read_to_string("input.txt").unwrap();

    // split lines into a vector of strings
    let lines: Vec<Game> = lines.lines().map(|line| parse_line(line)).collect();

    // calculate sum of games
    let mut sum: u32 = 0;

    for game in &lines {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for round in &game.rounds {
            max_red = u32::max(max_red, round.red);
            max_blue = u32::max(max_blue, round.blue);
            max_green = u32::max(max_green, round.green);
        }

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += game.id;
        }
    }

    println!("Solution to part 1: {}", sum);

    // calculate sum of the power of the games
    let mut power: u32 = 0;

    for game in &lines {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for round in &game.rounds {
            max_red = u32::max(max_red, round.red);
            max_blue = u32::max(max_blue, round.blue);
            max_green = u32::max(max_green, round.green);
        }

        power += max_red * max_blue * max_green;
    }

    println!("Solution to part 2: {}", power);
}
