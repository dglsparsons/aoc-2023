use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl std::cmp::PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn to_value(ht: &HandType) -> usize {
            match ht {
                HandType::FiveOfAKind => 0,
                HandType::FourOfAKind => 1,
                HandType::FullHouse => 2,
                HandType::ThreeOfAKind => 3,
                HandType::TwoPair => 4,
                HandType::OnePair => 5,
                HandType::HighCard => 6,
            }
        }
        to_value(self).cmp(&to_value(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    original_cards: String,
    bid: usize,
    hand_type: HandType,
    hand_type_excluding_jokers: HandType,
    joker_count: usize,
}

impl Hand {
    fn apply_jokers(&mut self) {
        self.cards = self
            .cards
            .chars()
            .map(|c| match c {
                'D' => 'Z',
                _ => c,
            })
            .collect::<String>();

        self.hand_type = match (&self.hand_type_excluding_jokers, self.joker_count) {
            (h, 0) => *h,
            (_, 5) => HandType::FiveOfAKind,
            (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind,
            (HandType::TwoPair, 1) => HandType::FullHouse,
            (HandType::OnePair, 3) => HandType::FiveOfAKind,
            (HandType::OnePair, 2) => HandType::FourOfAKind,
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,
            (HandType::HighCard, 4) => HandType::FiveOfAKind,
            (HandType::HighCard, 3) => HandType::FourOfAKind,
            (HandType::HighCard, 2) => HandType::ThreeOfAKind,
            (HandType::HighCard, 1) => HandType::OnePair,
            (h, _) => *h,
        }
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
    }
}

fn get_hand_type(cards: &str) -> (HandType, HandType, usize) {
    let mut countmap: HashMap<char, usize> = HashMap::new();
    for c in cards.chars() {
        let count = countmap.entry(c).or_insert(0);
        *count += 1;
    }
    let mut counts = countmap.values().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    let mut jokerless_counts = countmap
        .iter()
        .filter(|(k, _)| **k != 'J')
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    jokerless_counts.sort();
    jokerless_counts.reverse();
    (
        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        },
        match jokerless_counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        },
        *countmap.get(&'J').unwrap_or(&0),
    )
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let original_cards = parts.next().unwrap().to_owned();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        let (hand_type, hand_type_excluding_jokers, joker_count) = get_hand_type(&original_cards);
        let cards = original_cards
            .clone()
            .chars()
            .map(|c| match c {
                'A' => 'A',
                'K' => 'B',
                'Q' => 'C',
                'J' => 'D',
                'T' => 'E',
                '9' => 'F',
                '8' => 'G',
                '7' => 'H',
                '6' => 'I',
                '5' => 'J',
                '4' => 'K',
                '3' => 'L',
                '2' => 'M',
                _ => c,
            })
            .collect::<String>();

        Ok(Hand {
            joker_count,
            original_cards,
            bid,
            cards,
            hand_type,
            hand_type_excluding_jokers,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();

    let mut hands = input
        .lines()
        .map(|l| l.parse::<Hand>().unwrap())
        .collect::<Vec<_>>();

    hands.sort();
    hands.reverse();

    let mut total = 0;
    for (idx, h) in hands.iter().enumerate() {
        total += (idx + 1) * h.bid;
    }

    println!("part one: {}", total);

    let mut hands = hands
        .into_iter()
        .map(|mut h| {
            h.apply_jokers();
            h
        })
        .collect::<Vec<_>>();
    hands.sort();
    hands.reverse();

    let mut total = 0;
    for (idx, h) in hands.iter().enumerate() {
        total += (idx + 1) * h.bid;
    }

    println!("part two: {}", total);
}
