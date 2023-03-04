use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
enum Color {
    Heart,
    Spade,
    Diamond,
    Club,
}

const VALUE: [char; 13] = [
    'A', 'K', 'Q', 'J', '1', '9', '8', '7', '6', '5', '4', '3', '2',
];
const ACEINDEX: usize = 0;

fn get_value_index(chr: char) -> usize {
    VALUE.iter().position(|r| *r == chr).unwrap()
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Debug)]
enum Rank {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand<'a> {
    raw_hand: &'a str,
    rank: Rank,
    tiebreak: Vec<usize>,
}

fn get_color<'a>(cards: &Vec<&'a str>) -> Option<Color> {
    let first_color = cards[0].chars().last().unwrap();

    let same_color = cards
        .iter()
        .all(|card| first_color == card.chars().last().unwrap());

    if !same_color {
        return None;
    }

    match first_color {
        'H' => Some(Color::Heart),
        'S' => Some(Color::Spade),
        'D' => Some(Color::Diamond),
        'C' => Some(Color::Club),
        _ => None,
    }
}

fn get_kinds_and_tiebreak<'a>(cards: &Vec<&'a str>) -> (Option<Rank>, Vec<usize>) {
    let mut counter = HashMap::new();
    for card in cards {
        let chr = card.chars().next().unwrap();

        match counter.get(&chr) {
            Some(count) => counter.insert(chr, count + 1),
            None => counter.insert(chr, 1),
        };
    }

    let mut four_of_a_kind = None;
    let mut full_house = (None, None);
    let mut three_of_a_kind = None;
    let mut two_pairs = (None, None);
    let mut one_pair = None;
    let mut singles = vec![];

    for (key, value) in counter {
        if value == 4 {
            four_of_a_kind = Some(key);
        }
        if value == 3 {
            full_house = (Some(key), full_house.1);
            three_of_a_kind = Some(key)
        }
        if value == 2 {
            full_house = (full_house.0, Some(key));
            one_pair = Some(key);
            if two_pairs.0 == None {
                two_pairs.0 = Some(key);
            } else {
                two_pairs.1 = Some(key);
            }
        }
        if value == 1 {
            singles.push(get_value_index(key));
        }
    }
    singles.sort();

    match four_of_a_kind {
        Some(chr) => {
            return (
                Some(Rank::FourOfAKind),
                vec![get_value_index(chr), singles[0]],
            )
        }
        _ => {}
    }

    match full_house {
        (Some(primary), Some(secondary)) => {
            return (
                Some(Rank::FullHouse),
                vec![get_value_index(primary), get_value_index(secondary)],
            )
        }
        _ => {}
    }

    match three_of_a_kind {
        Some(chr) => {
            return (
                Some(Rank::ThreeOfAKind),
                vec![get_value_index(chr), singles[0], singles[1]],
            )
        }
        _ => {}
    }

    match two_pairs {
        (Some(primary), Some(secondary)) => {
            if primary > secondary {
                return (
                    Some(Rank::TwoPairs),
                    vec![
                        get_value_index(primary),
                        get_value_index(secondary),
                        singles[0],
                    ],
                );
            } else {
                return (
                    Some(Rank::TwoPairs),
                    vec![
                        get_value_index(secondary),
                        get_value_index(primary),
                        singles[0],
                    ],
                );
            }
        }
        _ => {}
    }

    match one_pair {
        Some(chr) => {
            return (
                Some(Rank::OnePair),
                vec![get_value_index(chr), singles[0], singles[1], singles[2]],
            )
        }
        _ => {}
    }

    (Some(Rank::HighCard), singles)
}

fn ace_is_lowest_in_straight<'a>(cards: &Vec<&'a str>, highest_card: usize) -> bool {
    highest_card == 0 && cards.iter().any(|card| card.starts_with('5'))
}

fn set_ace_to_lowest(tiebreak: Vec<usize>) -> Vec<usize> {
    let mut new_tiebreak = Vec::from_iter(tiebreak[1..tiebreak.len()].iter().cloned());
    new_tiebreak.push(13);
    new_tiebreak
}

fn get_rank<'a>(
    cards: &Vec<&'a str>,
    highest_card: usize,
    color: &Option<Color>,
) -> (Rank, Vec<usize>) {
    let (kind, tiebreak) = get_kinds_and_tiebreak(cards);

    let straight = is_straigh(&cards, highest_card);

    if color.is_some() && straight {
        if ace_is_lowest_in_straight(cards, highest_card) {
            return (Rank::StraightFlush, set_ace_to_lowest(tiebreak));
        }
        return (Rank::StraightFlush, tiebreak);
    }

    if color.is_some() {
        return (Rank::Flush, tiebreak);
    }

    if straight {
        if ace_is_lowest_in_straight(cards, highest_card) {
            return (Rank::Straight, set_ace_to_lowest(tiebreak));
        }
        return (Rank::Straight, tiebreak);
    }

    let rank = match kind {
        Some(Rank::FourOfAKind) => Rank::FourOfAKind,
        Some(Rank::FullHouse) => Rank::FullHouse,
        Some(rank) => rank,
        None => Rank::HighCard,
    };
    return (rank, tiebreak);
}

fn get_highest_card<'a>(cards: &mut Vec<&str>) -> usize {
    cards.sort_by(|a, b| {
        VALUE
            .iter()
            .position(|r| *r == a.chars().next().unwrap())
            .partial_cmp(&VALUE.iter().position(|r| *r == b.chars().next().unwrap()))
            .unwrap()
    });
    get_value_index(cards[0].chars().next().unwrap())
}

fn is_in_order(card: &str, idx: usize) -> i32 {
    let pos = (*(&VALUE
        .iter()
        .position(|r| *r == card.chars().next().unwrap())
        .unwrap())) as i32;
    pos - (idx as i32)
}

fn is_straigh<'a>(cards: &Vec<&'a str>, highest_card: usize) -> bool {
    if highest_card == ACEINDEX {
        cards[1..5]
            .iter()
            .enumerate()
            .all(|(idx, card)| is_in_order(cards[1], 1) == is_in_order(card, idx + 1))
            && ((is_in_order(cards[0], 0) == is_in_order(cards[1], 1))
                || cards.iter().any(|card| card.starts_with('5'))) //special case when A is the lowest card in the straight
    } else {
        cards
            .iter()
            .enumerate()
            .all(|(idx, card)| is_in_order(cards[0], 0) == is_in_order(card, idx))
    }
}

fn generate_hands<'a>(raw_hands: &[&'a str]) -> Vec<Hand<'a>> {
    let mut hands = vec![];

    for raw_hand in raw_hands.iter() {
        let mut cards = raw_hand.split(" ").collect::<Vec<&str>>();
        let color = get_color(&cards);
        let highest_card = get_highest_card(&mut cards);
        let (rank, tiebreak) = get_rank(&cards, highest_card, &color);
        hands.push(Hand {
            raw_hand,
            rank,
            tiebreak,
        })
    }
    hands
}

fn generate_winning_hands<'a>(mut hands: Vec<Hand<'a>>) -> Vec<&'a str> {
    hands.sort_by(|a, b| {
        if a.rank == b.rank {
            return a.tiebreak.cmp(&b.tiebreak);
        }
        return a.rank.cmp(&b.rank);
    });

    let mut answer: Vec<&'a str> = vec![hands[0].raw_hand];
    for idx in 1..hands.len() {
        if hands[idx].rank == hands[0].rank && hands[idx].tiebreak == hands[0].tiebreak {
            answer.push(hands[idx].raw_hand);
        }
    }

    answer
}

pub fn winning_hands<'a>(raw_hands: &[&'a str]) -> Vec<&'a str> {
    if raw_hands.len() == 1 {
        return raw_hands.to_vec();
    }
    let hands = generate_hands(raw_hands);
    generate_winning_hands(hands)
}
