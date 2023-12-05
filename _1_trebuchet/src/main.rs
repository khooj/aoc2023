use aho_corasick::{AhoCorasick, PatternID};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::{collections::HashMap, env};

fn part1(s: String) -> u64 {
    let mut res = 0;
    for l in s.lines() {
        let c = l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
        res += u64::from_str(&format!("{}{}", c.first().unwrap(), c.last().unwrap())).unwrap();
    }
    res
}

fn part2(s: String) -> u64 {
    let mut res = 0;
    let pt = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let keys = pt.keys().collect::<Vec<&&str>>();
    let ac = AhoCorasick::new(&keys).unwrap();
    for l in s.lines() {
        let mut mtch = Vec::with_capacity(10);
        for mat in ac.find_overlapping_iter(l) {
            mtch.push(mat.pattern());
        }

        let first = mtch.first().unwrap();
        let last = mtch.last().unwrap();
        let first = pt[keys[first.as_usize()]];
        let last = pt[keys[last.as_usize()]];

        res += first * 10 + last;
    }
    res
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = args.get(1).unwrap();
    let mut f = File::open(fp).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{}", part2(s));
}
