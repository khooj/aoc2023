mod priority_queue;

use itertools::Itertools;
use log::debug;
use priority_queue::Pq;
use std::cmp::Ordering;
use utils::get_file_string;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
enum HandStrength {
    High = 0, // 23456
    One,      // A23A4
    Two,      // 23432
    Three,    // TTT98
    Full,     // 23332
    Four,     // AA8AA
    Five,     // AAAAA
}

#[derive(Clone, Debug)]
struct Hand(String, HandStrength);

impl Hand {
    fn old_hand(s: &str) -> Hand {
        let hand = s.to_string();
        let freq = hand.chars().counts();
        let strength = Self::strength(&freq);
        Hand(hand, strength)
    }

    fn new(s: &str) -> Hand {
        let hand = s.to_string();
        let b = hand.chars().filter(|c| *c != 'J').counts();
        let freq_no_joker: Vec<_> = b.values().sorted().rev().collect();
        let joker_count = hand.chars().filter(|c| *c == 'J').count();
        let strength = Self::joker_strength(&freq_no_joker, joker_count);
        let strength = strength.unwrap_or(Self::strength(&b));
        Hand(hand, strength)
    }

    fn joker_strength(symb: &Vec<&usize>, jokers: usize) -> Option<HandStrength> {
        Some(match (&symb[..], jokers) {
            (&[], 5) | (&[1], 4) | (&[2], 3) | (&[3], 2) | (&[4], 1) => HandStrength::Five,
            (&[1, 1], 3) | (&[3, 1], 1) | (&[2, 1], 2)=> HandStrength::Four,
            (&[1, 1, 1], 2) | (&[2, 1, 1], 1) => HandStrength::Three,
            (&[1, 1, 1, 1], 1) => HandStrength::One,
            (&[2, 2], 1) => HandStrength::Full,
            _ => return None,
        })
    }

    fn strength(freq: &HashMap<char, usize>) -> HandStrength {
        let mut strength = HandStrength::High;
        if freq.len() == 5 {
            strength = HandStrength::High;
        } else if freq.len() == 1 {
            strength = HandStrength::Five;
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
        strength
    }

    fn as_u32(c: &char) -> u32 {
        match c {
            l @ '2'..='9' => l.to_digit(10).unwrap(),
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("not supported card"),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.1 != other.1 {
            return false;
        }

        let lhs: Vec<_> = self.0.chars().collect();
        let rhs: Vec<_> = other.0.chars().collect();
        for i in 0..lhs.len() {
            debug!("compare chars {} {}", &lhs[i], &rhs[i]);
            if let Some(k) = Hand::as_u32(&lhs[i]).partial_cmp(&Hand::as_u32(&rhs[i])) {
                if k == Ordering::Equal {
                    continue;
                }
                return false;
            }
        }
        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(o) = self.1.partial_cmp(&other.1) {
            if o == Ordering::Greater || o == Ordering::Less {
                return o;
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
                return k;
            }
        }
        Ordering::Equal
    }
}

#[derive(Clone, Debug)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl Eq for HandBid {}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
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
    let bids = parse_input(s);
    let mut pq = Pq::new(bids.len());
    for b in bids {
        pq.insert(b);
    }

    let mut rank = 1u64;
    let mut res = 0u64;
    while let Some(ll) = pq.get() {
        debug!("ll {:?} with rank {}", ll, rank);
        res += rank.saturating_mul(ll.into_iter().map(|el| el.bid).sum());
        rank += 1;
    }
    res
}

fn unique_elems(s: &str) -> bool {
    let bids = parse_input(s);
    let l = bids.len();
    let mut h = HashSet::new();
    for b in bids {
        h.insert(b.hand.0);
    }
    return h.len() == l;
}

fn total_winnings_variant2(s: &str) -> u64 {
    let bids = parse_input(s);
    let mut pq = BinaryHeap::new();
    for b in bids {
        pq.push(Reverse(b));
    }

    let mut rank = 1u64;
    let mut res = 0u64;
    while let Some(hb) = pq.pop() {
        debug!("hb {:?} with rank {}", hb, rank);
        res += rank.saturating_mul(hb.0.bid);
        rank += 1;
    }
    res
}

fn main() {
    env_logger::init();

    let s = get_file_string();
    // println!("unique_elems {}", unique_elems(&s));
    println!("part1 {}", total_winnings_variant2(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        env_logger::init();

        let hand = Hand::new("32T3K");
        assert_eq!(hand.1, HandStrength::One);

        let hand1 = Hand::new("KK677");
        let hand2 = Hand::new("KTJJT");
        let hand3 = Hand::new("QQQJA");
        assert_eq!(hand1.1, HandStrength::Two);
        assert_eq!(hand2.1, HandStrength::Four);
        assert_eq!(hand3.1, HandStrength::Four);
        assert!(hand2 > hand3 && hand2 > hand1);
        assert!(hand3 > hand1);

        assert_eq!(Hand::as_u32(&'2'), 2);
        assert_eq!(Hand::as_u32(&'J'), 1);
        assert!(Hand::as_u32(&'2') > Hand::as_u32(&'J'));

        let hand1 = Hand::new("JAAAA");
        let hand2 = Hand::new("22222");
        assert!(hand2 > hand1);
    }
}
