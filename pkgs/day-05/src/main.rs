use std::{ops::Range, str::FromStr};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use rayon::prelude::*;

fn main() {
    let input = include_str!("in").trim();
    let (_, (seeds, maps)) = parse(input).unwrap();
    dbg!(part1(&seeds, &maps));
    dbg!(part2(&seeds, &maps));
}

#[derive(Debug)]
struct Mapping {
    range: Range<u64>,
    offset: i64,
}
type Map = Vec<Mapping>;

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (input, seeds) = seeds(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, maps) = separated_list1(tag("\n\n"), map)(input)?;
    Ok((input, (seeds, maps)))
}

fn number<F: FromStr>(input: &str) -> IResult<&str, F> {
    map_res(digit1, FromStr::from_str)(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), number))(input)
}

fn map(input: &str) -> IResult<&str, Map> {
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, ranges) = separated_list1(tag("\n"), mapping)(input)?;
    Ok((input, ranges))
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, destination) = number::<u64>(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, source) = number::<u64>(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, len) = number::<u64>(input)?;
    Ok((
        input,
        Mapping {
            range: source..source + len,
            offset: destination as i64 - source as i64,
        },
    ))
}

fn part1(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .iter()
        .map(|&seed| proxy_all(seed, maps))
        .min()
        .unwrap()
}

fn proxy_all(seed: u64, maps: &[Map]) -> u64 {
    maps.iter().fold(seed, |value, map| proxy_one(value, map))
}

fn proxy_one(value: u64, map: &[Mapping]) -> u64 {
    map.iter()
        .find(|mapping| mapping.range.contains(&value))
        .map_or(value, |mapping| (value as i64 + mapping.offset) as u64)
}

fn part2(seeds: &[u64], maps: &[Map]) -> u64 {
    seeds
        .par_chunks(2)
        .flat_map(|pair| {
            let s = pair[0];
            let len = pair[1];
            s..s + len
        })
        .map(|seed| proxy_all(seed, maps))
        .min()
        .unwrap()
}
