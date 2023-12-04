use std::{collections::HashMap, str::FromStr};

use nom::{
    bytes::complete::take_till,
    character::complete::{digit1, line_ending, space1},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("in").trim();
    let (_, cards) = parse(input).unwrap();
    dbg!(part1(&cards));
    dbg!(part2(&cards));
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    yours: Vec<u32>,
}

fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    all_consuming(separated_list1(line_ending, card))(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = take_till_digit(input)?;
    let (input, id) = number(input)?;
    let (input, _) = take_till_digit(input)?;
    let (input, winning) = separated_list1(space1, number)(input)?;
    let (input, _) = take_till_digit(input)?;
    let (input, yours) = separated_list1(space1, number)(input)?;
    Ok((input, Card { id, winning, yours }))
}

fn take_till_digit(input: &str) -> IResult<&str, &str> {
    take_till(|c: char| c.is_ascii_digit())(input)
}

fn number<F: FromStr>(input: &str) -> IResult<&str, F> {
    map_res(digit1, FromStr::from_str)(input)
}

fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .filter_map(|card| {
            let count = card
                .yours
                .iter()
                .filter(|num| card.winning.contains(num))
                .count();
            if count > 0 {
                Some(1 << (count - 1))
            } else {
                None
            }
        })
        .sum()
}

fn part2(cards: &[Card]) -> u32 {
    let mut copies = cards
        .iter()
        .map(|card| (card.id, 1))
        .collect::<HashMap<u32, u32>>();
    for card in cards {
        let count = card
            .yours
            .iter()
            .filter(|num| card.winning.contains(num))
            .count() as u32;
        let multi = *copies.get(&card.id).unwrap_or(&1);
        for id in 1..=count {
            copies.entry(card.id + id).and_modify(|c| *c += multi);
        }
    }
    copies.values().sum()
}
