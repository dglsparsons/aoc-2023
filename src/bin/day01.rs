use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day01.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        let number = match (digits.first(), digits.last()) {
            (Some(x), Some(y)) => format!("{}{}", x, y).parse().unwrap(),
            _ => 0,
        };
        total += number;
    }
    println!("part 1: {}", total);

    let mut total = 0;
    for line in input.lines() {
        let numbers = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);

        let (_, first) = numbers
            .iter()
            .filter_map(|(key, val)| line.find(key).map(|pos| (pos, val)))
            .min_by_key(|&(pos, _)| pos)
            .unwrap();
        let reversed_line = line.chars().rev().collect::<String>();
        let (_, last) = numbers
            .iter()
            .filter_map(|(key, val)| {
                let reversed_key = key.chars().rev().collect::<String>();
                reversed_line.find(&reversed_key).map(|pos| (pos, val))
            })
            .min_by_key(|&(pos, _)| pos)
            .unwrap();

        total += first * 10 + last;
    }
    println!("part 2: {}", total);
}
