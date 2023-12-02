use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("in").trim();
    let (_, games) = parse(input).unwrap();
    dbg!(part1(&games));
    dbg!(part2(&games));
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|g| {
            if g.moves.iter().all(|&(num, color)| match color {
                Color::Red => num <= 12,
                Color::Green => num <= 13,
                Color::Blue => num <= 14,
            }) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| {
            let max_of = |color| {
                g.moves
                    .iter()
                    .filter_map(|&(num, c)| if c == color { Some(num) } else { None })
                    .max()
                    .unwrap_or(0)
            };
            let red = max_of(Color::Red);
            let green = max_of(Color::Green);
            let blue = max_of(Color::Blue);
            red * green * blue
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("invalid color"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    moves: Vec<(u32, Color)>,
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    all_consuming(separated_list1(line_ending, parse_game))(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, |id: &str| id.parse())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, moves) = separated_list1(alt((tag("; "), tag(", "))), parse_move)(input)?;
    Ok((input, Game { id, moves }))
}

fn parse_move(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, num) = map_res(digit1, |num: &str| num.parse())(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = map_res(
        alt((tag("red"), tag("green"), tag("blue"))),
        |color: &str| color.parse(),
    )(input)?;
    Ok((input, (num, color)))
}
