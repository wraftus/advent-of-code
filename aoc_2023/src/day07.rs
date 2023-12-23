use std::fs;
use std::cmp::Ordering;

const NUM_CARDS: usize = 14;
const JACK_NUM: usize  = 11;

#[derive(Clone, Copy, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPairs,
    OnePair,
    HighCard
}
impl HandType {
    fn from_cards(cards: &[usize; 5], use_jacks: bool) -> HandType {
        let mut card_counts: [usize; NUM_CARDS] = [0; NUM_CARDS];
        for &card_val in cards.iter() {
            if use_jacks && card_val == JACK_NUM { continue; }
            card_counts[card_val - 1] += 1;
        }

        let mut sorted_idxs: [usize; NUM_CARDS] =
            (0..NUM_CARDS).collect::<Vec<usize>>().try_into().unwrap();
        sorted_idxs.sort_by(|&idx, &jdx| card_counts[jdx].cmp(&card_counts[idx]));

        if use_jacks {
            for &card_val in cards.iter() {
                if card_val != JACK_NUM { continue; }
                card_counts[sorted_idxs[0]] += 1;
            }
        }

        // we have a five of a quintiplet of cards
        if card_counts[sorted_idxs[0]] == 5 {
            return HandType::FiveOfKind;
        }
        // we have a quadruplet of cards
        if card_counts[sorted_idxs[0]] == 4 {
            return HandType::FourOfKind;
        }
        // we have a triplet of cards
        if card_counts[sorted_idxs[0]] == 3 {
            // we also also a pair of cards
            if card_counts[sorted_idxs[1]] == 2 {
                return HandType::FullHouse;
            }
            // we have two other single cards
            return HandType::ThreeOfKind;
        }
        // we have a pair of cards
        if card_counts[sorted_idxs[0]] == 2 {
            // we also have another pair of cards
            if card_counts[sorted_idxs[1]] == 2 {
                return HandType::TwoPairs;
            }
            return HandType::OnePair
        }
        // we have five loose cards
        return HandType::HighCard
    }

    fn compare_types(type1: &HandType, type2: &HandType) -> Ordering {
        match type1 {
            HandType::FiveOfKind => match type2 {
                HandType::FiveOfKind => Ordering::Equal,
                _                    => Ordering::Greater
            },
            HandType::FourOfKind => match type2 {
                HandType::FiveOfKind => Ordering::Less,
                HandType::FourOfKind => Ordering::Equal,
                _                    => Ordering::Greater
            },
            HandType::FullHouse => match type2 {
                HandType::FiveOfKind => Ordering::Less,
                HandType::FourOfKind => Ordering::Less,
                HandType::FullHouse  => Ordering::Equal,
                _                    => Ordering::Greater
            },
            HandType::ThreeOfKind => match type2 {
                HandType::FiveOfKind  => Ordering::Less,
                HandType::FourOfKind  => Ordering::Less,
                HandType::FullHouse   => Ordering::Less,
                HandType::ThreeOfKind => Ordering::Equal,
                _                     => Ordering::Greater
            },
            HandType::TwoPairs => match type2 {
                HandType::FiveOfKind  => Ordering::Less,
                HandType::FourOfKind  => Ordering::Less,
                HandType::FullHouse   => Ordering::Less,
                HandType::ThreeOfKind => Ordering::Less,
                HandType::TwoPairs    => Ordering::Equal,
                _                     => Ordering::Greater
            },
            HandType::OnePair => match type2 {
                HandType::FiveOfKind  => Ordering::Less,
                HandType::FourOfKind  => Ordering::Less,
                HandType::FullHouse   => Ordering::Less,
                HandType::ThreeOfKind => Ordering::Less,
                HandType::TwoPairs    => Ordering::Less,
                HandType::OnePair     => Ordering::Equal,
                _                     => Ordering::Greater
            },
            HandType::HighCard => match type2 {
                HandType::FiveOfKind  => Ordering::Less,
                HandType::FourOfKind  => Ordering::Less,
                HandType::FullHouse   => Ordering::Less,
                HandType::ThreeOfKind => Ordering::Less,
                HandType::TwoPairs    => Ordering::Less,
                HandType::OnePair     => Ordering::Less,
                HandType::HighCard    => Ordering::Equal,
            }
        }
    }
}

struct Hand {
    cards: [usize; 5],
    bid:   usize
}
#[derive(Debug)]
struct TypedHand {
    type_: HandType,
    cards: [usize; 5],
    bid:   usize
}

fn read_input() -> Vec<Hand> {
    let file_contents =
        fs::read_to_string("input/day07.txt")
        .expect("Failed to read input file!");

    file_contents.lines().map(|line_str| {
        let line_splits: Vec<&str> = line_str.split_whitespace().collect();
        assert!(line_splits.len() == 2, "Malformed hand line!");

        let cards: [usize; 5] =
            line_splits[0].chars().map(|c| match c {
                '1' => 1, '2' => 2,  '3' => 3,  '4' => 4,  '5' => 5,  '6' => 6, '7' => 7, '8' => 8,
                '9' => 9, 'T' => 10, 'J' => 11, 'Q' => 12, 'K' => 13, 'A' => 14,
                _   => unreachable!("Invalid card in hand!")
            }).collect::<Vec<usize>>().try_into().expect("Invalid number of cards in hand!");
        let bid: usize = line_splits[1].parse().expect("Failed to parse hand's bid!");
        Hand { cards, bid }
    }).collect()
}

fn hand_sorting_key(hand1: &TypedHand, hand2: &TypedHand) -> Ordering {
    match HandType::compare_types(&hand1.type_, &hand2.type_) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less    => Ordering::Less,
        Ordering::Equal   => {
            for (&card1, &card2) in hand1.cards.iter().zip(hand2.cards.iter()) {
                if card1 != card2 { return card1.cmp(&card2) }
            }
            unreachable!("All cards in the hand are equal!");
        }
    }
}

fn puzzle_one(hands: &Vec<Hand>) -> usize {
    let mut typed_hands: Vec<TypedHand> = hands.iter().map(|hand| {
        TypedHand {
            type_: HandType::from_cards(&hand.cards, false),
            cards: hand.cards.clone(),
            bid:   hand.bid
        }
    }).collect();
    typed_hands.sort_by(|hand1, hand2| hand_sorting_key(hand1, hand2));
    typed_hands.iter().enumerate().map(|(idx, hand)| (idx + 1)*hand.bid).sum()
}

fn shift_card_for_jack(card: usize) -> usize {
    if      card == JACK_NUM { return 1; }
    else if card <  JACK_NUM { return card + 1}
    return card;
}

fn puzzle_two(hands: &Vec<Hand>) -> usize {
    let mut typed_hands: Vec<TypedHand> = hands.iter().map(|hand| {
        let shifted_cards: [usize; 5] =
            hand.cards.iter().map(|card| shift_card_for_jack(*card))
            .collect::<Vec<usize>>().try_into().unwrap();

        TypedHand {
            type_: HandType::from_cards(&hand.cards, true),
            cards: shifted_cards,
            bid:   hand.bid
        }
    }).collect();
    typed_hands.sort_by(|hand1, hand2| hand_sorting_key(hand1, hand2));
    typed_hands.iter().enumerate().map(|(idx, hand)| (idx + 1)*hand.bid).sum()
}

fn main() {
    let hands = read_input();

    println!("Puzzle 1: {}", puzzle_one(&hands));
    println!("Puzzle 2: {}", puzzle_two(&hands));
}
