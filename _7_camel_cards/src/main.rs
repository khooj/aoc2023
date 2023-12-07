use itertools::Itertools;
use log::debug;
use std::cmp::Ordering;
use utils::get_file_string;

#[derive(PartialEq, PartialOrd, Debug)]
enum HandStrength {
    High = 0,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

struct Hand(String, HandStrength);

impl Hand {
    fn new(s: &str) -> Hand {
        let hand = s.to_string();
        let freq = hand.chars().counts();
        let mut strength = HandStrength::High;
        if freq.len() == 5 {
            strength = HandStrength::Five;
        } else if freq.len() == 1 {
            // strength = HandStrength::High;
        } else if freq.len() == 2 {
            let v: Vec<&usize> = freq.values().collect();
            if *v[0] == 4 || *v[1] == 4 {
                strength = HandStrength::Four;
            }
            if *v[0] == 3 || *v[1] == 3 {
                strength = HandStrength::Full;
            }
        } else if freq.len() == 3 {
            let v: Vec<&usize> = freq.values().collect();
            if *v[0] == 3 || *v[1] == 3 || *v[2] == 3 {
                strength = HandStrength::Three;
            }
            if *v[0] == 2 && *v[1] == 2 || *v[1] == 2 && *v[2] == 2 || *v[0] == 2 && *v[2] == 2 {
                strength = HandStrength::Two;
            }
        } else if freq.len() == 4 {
            strength = HandStrength::One;
        }
        Hand(hand, strength)
    }

    fn as_u32(c: &char) -> u32 {
        match c {
            l @ '2'..='9' => l.to_digit(10).unwrap() - 2,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("not supported card"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(o) = self.1.partial_cmp(&other.1) {
            if o == Ordering::Greater || o == Ordering::Less {
                return Some(o);
            }
        }

        let lhs: Vec<_> = self.0.chars().collect();
        let rhs: Vec<_> = other.0.chars().collect();
        for i in 0..lhs.len() {
            debug!("compare chars {} {}", &lhs[i], &rhs[i]);
            if let Some(k) = Hand::as_u32(&lhs[i]).partial_cmp(&Hand::as_u32(&rhs[i])) {
                if k == Ordering::Equal {
                    continue;
                }
                return Some(k);
            }
        }
        Some(Ordering::Equal)
    }
}

struct HandBid {
    hand: Hand,
    bid: u64,
}

fn parse_input(s: &str) -> Vec<HandBid> {
    let mut res = vec![];
    for l in s.lines() {
        let k: Vec<_> = l.split(" ").collect();
        let hand = Hand::new(k[0]);
        let bid = k[1].parse().unwrap();
        res.push(HandBid { hand, bid })
    }
    res
}

fn total_winnings(s: &str) -> u64 {
    let mut bids = parse_input(s);
    bids.sort_by(|lhs, rhs| lhs.hand.partial_cmp(&rhs.hand).unwrap());
    bids.into_iter()
        .enumerate()
        .fold(0, |acc, (idx, hb)| acc + (idx as u64 + 1) * hb.bid)
}

fn main() {
    let s = get_file_string();
    println!("part1 {}", total_winnings(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        env_logger::init();

        let hand = Hand::new("32T3K");
        assert_eq!(hand.1, HandStrength::One);
        assert_eq!(Hand::as_u32(&'2'), 0);
        assert_eq!(Hand::as_u32(&'9'), 7);

        let hand1 = Hand::new("KK677");
        let hand2 = Hand::new("KTJJT");
        assert!(hand1 > hand2);
        assert!(hand1 > hand);
        assert!(hand2 > hand);
    }
}
