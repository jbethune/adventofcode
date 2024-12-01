use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use camino::Utf8Path as Path;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7,
    N8 = 8,
    N9 = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

#[derive(Debug)]
struct BadCardError {}

impl TryFrom<char> for Card {
    type Error = BadCardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(n) = value.to_digit(10) {
            Ok(match n {
                0 | 1 => panic!("bad input"),
                2 => Card::N2,
                3 => Card::N3,
                4 => Card::N4,
                5 => Card::N5,
                6 => Card::N6,
                7 => Card::N7,
                8 => Card::N8,
                9 => Card::N9,
                _ => panic!("cannot happen"),
            })
        } else {
            match value {
                'T' => Ok(Card::T),
                'J' => Ok(Card::J),
                'Q' => Ok(Card::Q),
                'K' => Ok(Card::K),
                'A' => Ok(Card::A),
                _ => Err(Self::Error {}),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn new(cards_list: Vec<Card>, bid: usize) -> Self {
        let mut cards = [Card::A; 5];
        let counter = {
            let mut counter = HashMap::<Card, usize>::new();
            for (i, c) in cards_list.iter().enumerate() {
                cards[i] = *c;
                counter.entry(*c).and_modify(|c| *c += 1).or_insert(1);
            }
            counter
        };
        let mut counts: Vec<usize> = counter.values().cloned().collect();
        counts.sort_unstable();
        let hand_type = match counts.pop().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts.pop().unwrap() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts.pop().unwrap() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("should not happen"),
        };
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand {
    cards: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

impl JokerHand {
    fn new(cards_list: Vec<Card>, bid: usize) -> Self {
        let mut cards = [Card::A; 5];
        let mut jokers = 0;
        let counter = {
            let mut counter = HashMap::<Card, usize>::new();
            for (i, c) in cards_list.iter().enumerate() {
                cards[i] = *c;
                if *c == Card::J {
                    jokers += 1;
                } else {
                    counter.entry(*c).and_modify(|c| *c += 1).or_insert(1);
                }
            }
            counter
        };
        let mut counts: Vec<usize> = counter.values().cloned().collect();
        counts.sort_unstable();

        let hand_type = {
            if let Some(highest_count) = counts.pop() {
                match highest_count {
                    5 => HandType::FiveOfAKind,
                    4 => {
                        if jokers == 1 {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FourOfAKind
                        }
                    }
                    3 => {
                        if let Some(next_highest) = counts.pop() {
                            match next_highest {
                                2 => HandType::FullHouse,
                                1 => match jokers {
                                    1 => HandType::FourOfAKind,
                                    0 => HandType::ThreeOfAKind,
                                    _ => panic!("cannot happen"),
                                },
                                _ => panic!("cannot happen"),
                            }
                        } else {
                            assert_eq!(jokers, 2);
                            HandType::FiveOfAKind
                        }
                    }
                    2 => {
                        if let Some(next_highest) = counts.pop() {
                            match next_highest {
                                2 => {
                                    if jokers == 1 {
                                        HandType::FullHouse
                                    } else {
                                        HandType::TwoPair
                                    }
                                }
                                1 => match jokers {
                                    2 => HandType::FourOfAKind,
                                    1 => HandType::ThreeOfAKind,
                                    0 => HandType::OnePair,
                                    _ => panic!("cannot happen"),
                                },
                                _ => panic!("cannot happen"),
                            }
                        } else {
                            assert_eq!(jokers, 3);
                            HandType::FiveOfAKind
                        }
                    }
                    1 => match jokers {
                        4 => HandType::FiveOfAKind,
                        3 => HandType::FourOfAKind,
                        2 => HandType::ThreeOfAKind,
                        1 => HandType::OnePair,
                        0 => HandType::HighCard,
                        _ => panic!("impossible"),
                    },
                    _ => panic!("should not happen"),
                }
            } else {
                HandType::FiveOfAKind // 5 jokers
            }
        };
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.hand_type.cmp(&other.hand_type);
        if result == Ordering::Equal {
            for (card_a, card_b) in self.cards.iter().zip(other.cards.iter()) {
                let result = card_a.cmp(card_b);
                if result != Ordering::Equal {
                    return result;
                }
            }
        }
        result
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.hand_type.cmp(&other.hand_type);
        if result == Ordering::Equal {
            for (card_a, card_b) in self.cards.iter().zip(other.cards.iter()) {
                if *card_a == Card::J {
                    if *card_b != Card::J {
                        return Ordering::Less;
                    }
                } else if *card_b == Card::J {
                    return Ordering::Greater;
                }
                let result = card_a.cmp(card_b);
                if result != Ordering::Equal {
                    return result;
                }
            }
        }
        result
    }
}

fn part1(input: &Path) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    let mut hands: Vec<Hand> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (cards, bid) = line.trim().split_once(' ').unwrap();
        let cards: Vec<Card> = cards.chars().map(|c| c.try_into().unwrap()).collect();
        let bid = bid.parse().unwrap();
        let hand = Hand::new(cards, bid);
        hands.push(hand);
    }
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn part2(input: &Path) -> usize {
    let reader = BufReader::new(File::open(input).unwrap());
    let mut hands: Vec<JokerHand> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (cards, bid) = line.trim().split_once(' ').unwrap();
        let cards: Vec<Card> = cards.chars().map(|c| c.try_into().unwrap()).collect();
        let bid = bid.parse().unwrap();
        let hand = JokerHand::new(cards, bid);
        hands.push(hand);
    }
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

pub fn main(_input: &Path) -> (usize, usize) {
    (part1(_input), part2(_input))
}
