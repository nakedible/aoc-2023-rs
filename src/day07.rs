use anyhow::Result;
use test_case::test_case;

type Card = u8;

fn parse_card(c: char) -> Option<Card> {
    match c {
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

type Hand = [Card; 5];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input(filename: &str) -> Result<Vec<(Hand, i64)>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input
        .lines()
        .map(|line| {
            let (handstr, betstr) = line.split_once(" ").unwrap();
            let cards: [Card; 5] = handstr
                .chars()
                .filter_map(|c| parse_card(c))
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let bet = betstr.parse::<i64>().unwrap();
            (cards, bet)
        })
        .collect();
    return Ok(ret);
}

fn rank_hand(hand: &Hand, j: bool) -> Rank {
    let mut counts = [0u8; 15];
    let mut rank = Rank::HighCard;
    let mut jokers = 0;
    for &card in hand {
        if j && card == 11 {
            jokers += 1;
            continue;
        }
        counts[card as usize] += 1;
        match (counts[card as usize], rank) {
            (2, Rank::HighCard) => rank = Rank::OnePair,
            (2, Rank::OnePair) => rank = Rank::TwoPairs,
            (2, Rank::ThreeOfAKind) => rank = Rank::FullHouse,
            (3, Rank::OnePair) => rank = Rank::ThreeOfAKind,
            (3, Rank::TwoPairs) => rank = Rank::FullHouse,
            (4, _) => rank = Rank::FourOfAKind,
            (5, _) => rank = Rank::FiveOfAKind,
            _ => (),
        }
    }
    match (jokers, rank) {
        (5, _) => rank = Rank::FiveOfAKind,
        (4, _) => rank = Rank::FiveOfAKind,
        (3, Rank::OnePair) => rank = Rank::FiveOfAKind,
        (3, Rank::HighCard) => rank = Rank::FourOfAKind,
        (2, Rank::ThreeOfAKind) => rank = Rank::FiveOfAKind,
        (2, Rank::OnePair) => rank = Rank::FourOfAKind,
        (2, Rank::HighCard) => rank = Rank::ThreeOfAKind,
        (1, Rank::FourOfAKind) => rank = Rank::FiveOfAKind,
        (1, Rank::ThreeOfAKind) => rank = Rank::FourOfAKind,
        (1, Rank::TwoPairs) => rank = Rank::FullHouse,
        (1, Rank::OnePair) => rank = Rank::ThreeOfAKind,
        (1, Rank::HighCard) => rank = Rank::OnePair,
        (_, _) => (),
    }
    rank
}

#[test_case("inputs/example-07-1.txt" => matches Ok(6440))]
#[test_case("inputs/input-07.txt" => matches Ok(249726565))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ranked: Vec<(Rank, Hand, i64)> = input
        .into_iter()
        .map(|(hand, bet)| (rank_hand(&hand, false), hand, bet))
        .collect();
    ranked.sort();
    let mut total = 0;
    for (i, (_, _, bet)) in ranked.iter().enumerate() {
        total += (i as i64 + 1) * bet;
    }
    Ok(total)
}

fn downgrade(card: Card) -> Card {
    match card {
        11 => 1,
        _ => card,
    }
}

#[test_case("inputs/example-07-1.txt" => matches Ok(5905))]
#[test_case("inputs/input-07.txt" => matches Ok(251135960))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut ranked: Vec<(Rank, Hand, i64)> = input
        .into_iter()
        .map(|(hand, bet)| (rank_hand(&hand, true), hand.map(downgrade), bet))
        .collect();
    ranked.sort();
    let mut total = 0;
    for (i, (_, _, bet)) in ranked.iter().enumerate() {
        total += (i as i64 + 1) * bet;
    }
    Ok(total)
}
