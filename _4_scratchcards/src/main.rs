use std::collections::{hash_map::RandomState, HashSet, HashMap};

use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, space0, u64},
    combinator::opt,
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use utils::get_file_string;

#[derive(Debug)]
struct Card {
    id: u64,
    winning: Vec<u64>,
    have: Vec<u64>,
}

fn parse_single_card(s: &str) -> IResult<&str, Card> {
    let (s, id) = delimited(tag("Card "), preceded(space0, u64), char(':'))(s)?;
    let (s, winning) = many0(preceded(alt((tag("  "), tag(" "))), u64))(s)?;
    let (s, _) = tag(" |")(s)?;
    let (s, have) = many0(preceded(alt((tag("  "), tag(" "))), u64))(s)?;
    Ok((s, Card { id, winning, have }))
}

fn parse_cards(s: &str) -> Vec<Card> {
    let (_, cards) = many0(terminated(parse_single_card, opt(newline)))(s).unwrap();
    cards
}

fn cards_points_part1(s: &str) -> u64 {
    let cards = parse_cards(s);
    let mut res = 0;
    for card in cards {
        // explicit type needed?
        let win: HashSet<u64, RandomState> = HashSet::from_iter(card.winning);
        let have = HashSet::from_iter(card.have);
        let have_winning_numbers: Vec<&u64> = have.intersection(&win).collect();
        if have_winning_numbers.len() == 0 {
            continue;
        }
        res += 2u64.pow((have_winning_numbers.len() - 1) as u32);
    }
    res
}

fn total_cards(s: &str) -> u64 {
    let cards = parse_cards(s);
    let mut nums = HashMap::new();
    let last_id = cards.last().unwrap().id + 1;
    for card in cards {
        *nums.entry(card.id).or_default() += 1;
        let win: HashSet<u64, RandomState> = HashSet::from_iter(card.winning);
        let have = HashSet::from_iter(card.have);
        let winning_numbers_count = have.intersection(&win).count();
        debug!("have {} wins for card {}", winning_numbers_count, card.id);
        for i in (card.id+1)..=(card.id+winning_numbers_count as u64) {
            *nums.entry(i).or_default() += *nums.entry(card.id).or_default();
        }
    }
    debug!("result card instances {:?}", nums);
    assert!(!nums.contains_key(&last_id));
    nums.values().sum()
}

fn main() {
    env_logger::init();
    let s = get_file_string();
    println!("part1 {}", cards_points_part1(&s));
    println!("part2 {}", total_cards(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let (_, card) = parse_single_card("Card 1: 11 | 12").unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(&card.winning[..], &[11]);
        assert_eq!(&card.have[..], &[12]);

        let (_, card) = parse_single_card("Card 1:  1 | 12").unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(&card.winning[..], &[1]);
        assert_eq!(&card.have[..], &[12]);

        let (_, card) = parse_single_card("Card  18: 61 30 69 53 76 32 29 45 26 79 | 60 69 82 78 31 72 91 13 92 40 24 54 25 14 22 87 70 35 18  7 96 86 49 15 66").unwrap();
        assert_eq!(card.id, 18);
        assert_eq!(card.winning.len(), 10);
        assert_eq!(card.have.len(), 25);

        let cards = parse_cards("Card 1: 11 | 12\nCard 2:  3 | 14");
        assert_eq!(cards[0].id, 1);
        assert_eq!(&cards[0].winning[..], &[11]);
        assert_eq!(&cards[0].have[..], &[12]);
        assert_eq!(cards[1].id, 2);
        assert_eq!(&cards[1].winning[..], &[3]);
        assert_eq!(&cards[1].have[..], &[14]);
    }
}
