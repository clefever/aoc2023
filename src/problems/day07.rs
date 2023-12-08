use fxhash::{FxHashSet, FxHashMap};
use std::cmp::Ordering;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(7);
    advent_of_code::answer(1, Some(252052080), part1(&input));
    advent_of_code::answer(2, Some(252898370), part2(&input));
}

fn part1(input: &[String]) -> i32 {
    let mut hands = parse_hands(&input, false);
    hands.sort();
    (0..hands.len()).map(|i| hands[i].bid * (i as i32 + 1)).sum()
}

fn part2(input: &[String]) -> i32 {
    let mut hands = parse_hands(&input, true);
    hands.sort();
    (0..hands.len()).map(|i| hands[i].bid * (i as i32 + 1)).sum()
}

fn parse_hands(input: &[String], use_joker: bool) -> Vec<Hand> {
    let f = if use_joker {
        calculate_hand_type2
    } else {
        calculate_hand_type
    };

    input.iter().map(|line| {
        let split = line.split(' ').collect::<Vec<_>>();
        let cards = split[0].chars().collect::<Vec<_>>();
        Hand { cards: cards.clone(), bid: split[1].parse::<i32>().unwrap(), hand_type: f(&cards), use_joker }
    }).collect::<Vec<_>>()
}

fn calculate_hand_type(cards: &Vec<char>) -> HandType {
    let set = cards.into_iter().collect::<FxHashSet<_>>();
    let counts = set.iter().map(|&c| (c, cards.iter().filter(|&ch| ch == c).count())).collect::<FxHashMap<_, _>>();
    match counts.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 => match counts.values().any(|&v| v == 3) {
            true => HandType::ThreeOfAKind,
            false => HandType::TwoPair
        }
        2 => match counts.values().next() {
            Some(1) | Some(4) => HandType::FourOfAKind,
            Some(2) | Some(3) => HandType::FullHouse,
            _ => panic!("invalid hand")
        }
        1 => HandType::FiveOfAKind,
        _ => panic!("invalid hand")
    }
}

fn calculate_hand_type2(cards: &Vec<char>) -> HandType {
    let joker_count = cards.iter().filter(|&c| c == &'J').count();
    let set = cards.into_iter().filter(|&c| c != &'J').collect::<FxHashSet<_>>();
    let counts = set.iter().map(|&c| (c, cards.iter().filter(|&ch| ch == c).count())).collect::<FxHashMap<_, _>>();
    match joker_count {
        5 | 4 => HandType::FiveOfAKind,
        3 => match counts.len() {
            2 => HandType::FourOfAKind,
            1 => HandType::FiveOfAKind,
            _ => panic!("invalid hand")
        }
        2 => match counts.len() {
            3 => HandType::ThreeOfAKind,
            2 => HandType::FourOfAKind,
            1 => HandType::FiveOfAKind,
            _ => panic!("invalid hand")
        }
        1 => match counts.len() {
            4 => HandType::OnePair,
            3 => HandType::ThreeOfAKind,
            2 => match counts.values().next() {
                Some(1) | Some(3) => HandType::FourOfAKind,
                Some(2) => HandType::FullHouse,
                _ => panic!("invalid hand")
            }
            1 => HandType::FiveOfAKind,
            _ => panic!("invalid hand")
        }
        0 => calculate_hand_type(cards),
        _ => panic!("invalid hand")
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7
}

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    hand_type: HandType,
    use_joker: bool
}

const CARD_VALUES: &str = "23456789TJQKA";
const CARD_VALUES_JOKER: &str = "J23456789TQKA";

fn cmp_card_value(a: &Vec<char>, b: &Vec<char>, use_joker: bool) -> Ordering {
    let card_values = if use_joker {
        CARD_VALUES_JOKER
    } else {
        CARD_VALUES
    };

    for i in 0..a.len() {
        let idx_a = card_values.chars().position(|c| c == a[i]).unwrap();
        let idx_b = card_values.chars().position(|c| c == b[i]).unwrap();
        if idx_a < idx_b {
            return Ordering::Less;
        } else if idx_a > idx_b {
            return Ordering::Greater;
        }
    }

    return Ordering::Equal;
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type == other.hand_type {
            return Some(cmp_card_value(&self.cards, &other.cards, self.use_joker));
        }
        
        Some(self.hand_type.cmp(&other.hand_type))
    }
}
