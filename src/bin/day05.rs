use std::{fs::read_to_string, str::FromStr};

fn main() {
    let input = read_to_string("input/day05.txt").unwrap();

    part_one(&input);
    part_two(&input);
}

#[derive(Debug)]
struct Map {
    dest: u64,
    src: u64,
    range: u64,
}

impl Map {
    fn try_convert(&self, value: u64) -> Option<u64> {
        if value >= self.src && value < self.src + self.range {
            Some(value - self.src + self.dest)
        } else {
            None
        }
    }

    fn rev_lookup(&self, value: u64) -> Option<u64> {
        if value >= self.dest && value < self.dest + self.range {
            Some(value - self.dest + self.src)
        } else {
            None
        }
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split the string into 3
        let bits = s.split(' ').collect::<Vec<&str>>();
        if bits.len() != 3 {
            return Err(format!("Wrong length {}", bits.len()));
        }

        Ok(Map {
            dest: bits[0].parse::<u64>().unwrap(),
            src: bits[1].parse::<u64>().unwrap(),
            range: bits[2].parse::<u64>().unwrap(),
        })
    }
}

struct SeedRange {
    start: u64,
    count: u64,
}

impl SeedRange {
    fn contains(&self, val: u64) -> bool {
        val >= self.start && val < self.start + self.count
    }

    fn from(bits: &[&str]) -> Self {
        let start = bits[0].parse::<u64>().unwrap();
        let count = bits[1].parse::<u64>().unwrap();
        Self { start, count }
    }
}

fn part_two(input: &str) {
    let bits = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = bits.iter().take(1).collect::<Vec<&&str>>()[0];
    let seeds = seeds.split(' ').skip(1).collect::<Vec<_>>();
    let seeds = seeds.chunks(2).map(SeedRange::from).collect::<Vec<_>>();

    let maps = bits
        .into_iter()
        .skip(1)
        .map(|s| {
            s.trim()
                .split('\n')
                .skip(1)
                .map(|s| s.parse::<Map>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<Map>>>();

    let mut min = 0;
    let lowest_location = loop {
        let mut value = min;
        for map in maps.iter().rev() {
            for m in map.iter() {
                if let Some(v) = m.rev_lookup(value) {
                    value = v;
                    break;
                }
            }
        }
        // If, after all the reverse lookups are done, we have a valid seed, then we're done
        if seeds.iter().any(|s| s.contains(value)) {
            break min;
        }

        min += 1;
    };
    println!("Part two: {}", lowest_location);
}

fn part_one(input: &str) {
    let bits = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = bits.iter().take(1).collect::<Vec<&&str>>()[0];
    let mut values = seeds
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = bits
        .into_iter()
        .skip(1)
        .map(|s| {
            s.trim()
                .split('\n')
                .skip(1)
                .map(|s| s.parse::<Map>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<Map>>>();

    let mut min = None;
    for value in values.iter_mut() {
        for map in maps.iter() {
            for m in map.iter() {
                if let Some(v) = m.try_convert(*value) {
                    *value = v;
                    break;
                }
            }
        }
        if min.is_none() || *value < min.unwrap() {
            min = Some(*value);
        }
    }
    println!("Part one: {}", min.unwrap());
}
