use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input/day03.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    part_one(&input);
    part_two(&input);
}

fn build_number(y: usize, x: usize, input: &[Vec<char>]) -> (usize, HashSet<(usize, usize)>) {
    // Work leftways, until we are at the start.
    if x > 0 {
        let c = input.get(y).and_then(|l| l.get(x - 1));
        if let Some(cx) = c {
            if cx.is_numeric() {
                return build_number(y, x - 1, input);
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert((y, x));
    let mut number = input[y][x].to_digit(10).unwrap() as usize;
    let mut xx = x;
    loop {
        xx += 1;
        match input.get(y).and_then(|l| l.get(xx)) {
            Some(c) if c.is_numeric() => {
                visited.insert((y, xx));
                number = number * 10 + c.to_digit(10).unwrap() as usize;
            }
            _ => break,
        }
    }
    (number, visited)
}

fn flood_neighbours(y: usize, x: usize, input: &[Vec<char>]) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut out = vec![];
    for (yn, xn) in get_neighbours(y, x) {
        if visited.contains(&(yn, xn)) {
            continue;
        }
        visited.insert((yn, xn));
        match input.get(yn).and_then(|l| l.get(xn)) {
            Some(c) if c.is_numeric() => {
                let (number, extravisited) = build_number(yn, xn, input);
                visited.extend(extravisited);
                out.push(number);
            }
            _ => continue,
        }
    }

    out
}

fn part_two(input: &[Vec<char>]) {
    let mut total = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '*' {
                let ns = flood_neighbours(y, x, input);
                if ns.len() == 2 {
                    total += ns[0] * ns[1];
                }
            }
        }
    }

    println!("part 2: {}", total);
}

fn part_one(input: &[Vec<char>]) {
    // part 1
    let mut valid = vec![];
    for (y, line) in input.iter().enumerate() {
        let mut number = vec![];
        let mut is_valid = false;
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                for (yn, xn) in get_neighbours(y, x) {
                    match input.get(yn).and_then(|l| l.get(xn)) {
                        Some('.') | Some('0') | Some('1') | Some('2') | Some('3') | Some('4')
                        | Some('5') | Some('6') | Some('7') | Some('8') | Some('9') | None => {
                            continue
                        }
                        _ => {
                            is_valid = true;
                            break;
                        }
                    }
                }
                number.push(c);
                continue;
            }
            if !number.is_empty() {
                if is_valid {
                    valid.push(
                        number
                            .into_iter()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                    );
                }
                number = vec![];
                is_valid = false;
            }
        }
        // just finished a line, push the number
        if !number.is_empty() && is_valid {
            valid.push(
                number
                    .into_iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }

    println!("part 1: {}", valid.into_iter().sum::<u64>());
}

fn get_neighbours(y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut out = vec![(y, x + 1), (y + 1, x + 1), (y + 1, x)];
    if y > 0 {
        if x > 0 {
            out.push((y - 1, x - 1));
        }
        out.push((y - 1, x));
        out.push((y - 1, x + 1));
    }
    if x > 0 {
        out.push((y, x - 1));
        out.push((y + 1, x - 1));
    }

    out
}
