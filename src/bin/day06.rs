use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day06.txt").unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut lines = input.lines();
    let times = lines.next().unwrap(); //.split_whitespace();
    let distances = lines.next().unwrap(); //.split_whitespace();

    let times = times
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = distances
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut answer = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for i in 0..=*t {
            let distance = i * (t - i);
            if distance > *d {
                count += 1;
            }
        }

        answer *= count;
    }
    println!("part one: {}", answer);
}

fn part_two(input: &str) {
    let mut lines = input.lines();
    let times = lines.next().unwrap(); //.split_whitespace();
    let distances = lines.next().unwrap(); //.split_whitespace();
                                           //
    let time = times
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distances
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut count = 0;
    for i in 0..=time {
        let d = i * (time - i);
        if d > distance {
            count += 1;
        }
    }

    println!("part two: {}", count);
}
