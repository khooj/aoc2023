use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline, u64},
    combinator::opt,
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
    IResult,
};
use utils::get_file_string;

struct Ranges {
    dst: Range<u64>,
    src: Range<u64>,
}

impl Ranges {
    fn map(&self, src: u64) -> Option<u64> {
        let s = src.checked_sub(self.src.start);
        if s.is_none() {
            return None;
        }
        let s = self.dst.start + s.unwrap();
        if s > self.dst.end {
            return None;
        }
        Some(s)
    }
}

struct MapRanges(Vec<Ranges>);

impl MapRanges {
    fn map(&self, src: u64) -> u64 {
        for range in &self.0 {
            if let Some(k) = range.map(src) {
                return k;
            }
        }
        return src;
    }
}

fn parse_range(s: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (s, dst) = terminated(u64, char(' '))(s)?;
    let (s, src) = terminated(u64, char(' '))(s)?;
    let (s, size) = u64(s)?;
    Ok((
        s,
        (
            Range {
                start: dst,
                end: dst + size,
            },
            Range {
                start: src,
                end: src + size,
            },
        ),
    ))
}

fn parse_map(s: &str) -> IResult<&str, MapRanges> {
    let (s, ranges) = many0(terminated(parse_range, opt(newline)))(s)?;
    Ok((
        s,
        MapRanges(
            ranges
                .into_iter()
                .map(|(dst, src)| Ranges { dst, src })
                .collect(),
        ),
    ))
}

fn parse_map_with_tag<'a>(t: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, MapRanges> {
    delimited(tag(t), parse_map, many0(newline))
}

fn parse_input(s: &str) -> IResult<&str, (Vec<u64>, Vec<MapRanges>)> {
    let (s, seeds) = delimited(tag("seeds:"), many0(preceded(char(' '), u64)), many1(newline))(s)?;
    let (s, seed_to_soil) = parse_map_with_tag("seed-to-soil map:\n")(s)?;
    let (s, soild_to_fertilizer) = parse_map_with_tag("soil-to-fertilizer map:\n")(s)?;
    let (s, fertilizer_to_water) = parse_map_with_tag("fertilizer-to-water map:\n")(s)?;
    let (s, water_to_light) = parse_map_with_tag("water-to-light map:\n")(s)?;
    let (s, light_to_temperature) = parse_map_with_tag("light-to-temperature map:\n")(s)?;
    let (s, temperature_to_humidity) = parse_map_with_tag("temperature-to-humidity map:\n")(s)?;
    let (s, humidity_to_location) = parse_map_with_tag("humidity-to-location map:\n")(s)?;

    Ok((
        s,
        (
            seeds,
            vec![
                seed_to_soil,
                soild_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            ],
        ),
    ))
}

fn lowest_location_part1(s: &str) -> u64 {
    let (_, (seeds, maps)) = parse_input(s).unwrap();
    let mut res = u64::MAX;
    for seed in seeds {
        let mut loc = seed;
        for map in &maps {
            loc = map.map(loc);
        }
        if loc < res {
            res = loc;
        }
    }
    res
}

fn main() {
    env_logger::init();
    let s = get_file_string();
    println!("part1 {}", lowest_location_part1(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let (_, (dst, src)) = parse_range("50 98 2").unwrap();
        assert_eq!(dst, 50..52);
        assert_eq!(src, 98..100);

        let (_, ranges) = parse_map("50 98 2\n50 98 2\n").unwrap();
        assert_eq!(ranges.0.len(), 2);

        let (_, ranges) = parse_map("50 98 2\n50 98 2").unwrap();
        assert_eq!(ranges.0.len(), 2);
    }
}
