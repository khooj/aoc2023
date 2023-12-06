use log::debug;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, newline, u32},
    combinator::opt,
    multi::many1,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use utils::get_file_string;

#[derive(Debug)]
struct Race {
    time: u32,
    dist: u32,
}

fn parse_line<'a>(s: &'a str, t: &str) -> IResult<&'a str, Vec<u32>> {
    let (s, _) = tag(t)(s)?;
    let (s, _) = multispace0(s)?;
    many1(delimited(multispace0, u32, opt(newline)))(s)
}

fn parse_input(s: &str) -> Vec<Race> {
    let (s, times) = parse_line(s, "Time:").unwrap();
    let (_, dists) = parse_line(s, "Distance:").unwrap();

    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(t, d)| Race { time: t, dist: d })
        .collect()
}

fn is_win_possible(time: u32, max_dist: u32, press_time: u32) -> bool {
    debug!("time {} max_dist {} press_time {}", time, max_dist, press_time);
    (time - press_time) * press_time > max_dist
}

fn every_possible_race_solution_part1(s: &str) -> u32 {
    let races = parse_input(s);
    let mut res = 1;
    for race in races {
        let mut wins = 0;
        for press_time in 0..=race.time {
            wins += if is_win_possible(race.time, race.dist, press_time) {
                1
            } else {
                0
            };
        }
        res *= if wins > 0 { wins } else { 1 };
    }
    res
}

fn main() {
    env_logger::init();

    let s = get_file_string();
    println!("part1 {}", every_possible_race_solution_part1(&s));
}
