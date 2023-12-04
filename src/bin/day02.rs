use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Game {
    red: u8,
    green: u8,
    blue: u8,
}

impl Game {
    fn is_valid(&self, red: u8, green: u8, blue: u8) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }

    fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game {
            red: 0,
            green: 0,
            blue: 0,
        };
        let colors = s.split(',').map(|c| c.trim()).collect::<Vec<_>>();
        for color in colors {
            let c = color.split(' ').map(|s| s.trim()).collect::<Vec<_>>();
            match (c[0], c[1]) {
                (v, "red") => game.red = v.parse::<u8>().unwrap(),
                (v, "green") => game.green = v.parse::<u8>().unwrap(),
                (v, "blue") => game.blue = v.parse::<u8>().unwrap(),
                (v, x) => {
                    println!("Invalid color {} - {}", x, v);
                    return Err("Invalid color".to_string());
                }
            };
        }

        Ok(game)
    }
}

struct Games(Vec<Game>);

impl Games {
    fn iter(&self) -> std::slice::Iter<'_, Game> {
        self.0.iter()
    }

    fn fewest_possible(&self) -> Game {
        Game {
            red: self.iter().map(|g| g.red).max().unwrap(),
            green: self.iter().map(|g| g.green).max().unwrap(),
            blue: self.iter().map(|g| g.blue).max().unwrap(),
        }
    }
}

impl FromStr for Games {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(':').collect::<Vec<_>>().get(1).unwrap().trim(); // Take only the second half of the string
        let games = s
            .split(';')
            .map(|game| game.parse::<Game>().unwrap())
            .collect::<Vec<_>>();

        Ok(Games(games))
    }
}

fn main() {
    let input = read_to_string("input/day02.txt").unwrap();

    let games: Vec<Games> = input
        .lines()
        .map(|line| line.parse::<Games>().unwrap())
        .collect();

    let mut count = 0;
    for (idx, game) in games.iter().enumerate() {
        if game.iter().all(|g| g.is_valid(12, 13, 14)) {
            count += idx + 1;
        }
    }

    println!("part 1: {}", count);

    let mut count = 0;
    for game in games.iter() {
        let fewest = game.fewest_possible();
        count += fewest.power();
    }
    println!("part 2: {}", count);
}
