use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day04.txt").unwrap();
    let mut total = 0;

    for line in input.lines() {
        let line = line.split(':').collect::<Vec<&str>>()[1];
        let mut map = HashMap::new();
        line.trim().split(' ').for_each(|x| {
            if let Ok(n) = x.parse::<u64>() {
                map.entry(n).and_modify(|x| *x += 1).or_insert(1);
            }
        });

        let num_winning = map.into_values().filter(|&v| v >= 2).count();
        if num_winning > 0 {
            total += 2_u64.pow(num_winning as u32 - 1);
        }
    }
    println!("part one {}", total);

    let mut copies = HashMap::new();
    for (idx, _) in input.lines().enumerate() {
        // to start, we have one copy of every card.
        copies.insert(idx, 1);
    }
    for (idx, line) in input.lines().enumerate() {
        let line = line.split(':').collect::<Vec<&str>>()[1];
        let mut map = HashMap::new();
        line.trim().split(' ').for_each(|x| {
            if let Ok(n) = x.parse::<u64>() {
                map.entry(n).and_modify(|x| *x += 1).or_insert(1);
            }
        });

        let num_winning = map.into_values().filter(|&v| v >= 2).count();
        if num_winning > 0 {
            for i in idx + 1..=idx + num_winning {
                let count = *copies.get(&idx).unwrap(); // count of current hand
                copies.entry(i).and_modify(|x| *x += count);
            }
        }
    }
    println!("part two {}", copies.values().sum::<u64>());
}
