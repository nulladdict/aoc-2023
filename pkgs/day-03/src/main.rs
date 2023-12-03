use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("in").trim();
    let field = parse(input);
    dbg!(part1(&field));
    dbg!(part2(&field));
}

#[derive(Debug)]
struct Part {
    num: u32,
    coords: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Field {
    parts: Vec<Part>,
    symbols: HashMap<(i32, i32), char>,
}

fn parse(input: &str) -> Field {
    let mut parts = vec![];
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut coords = vec![];
        for (x, char) in line.chars().enumerate() {
            match char {
                '0'..='9' => {
                    num = num * 10 + char.to_digit(10).unwrap();
                    coords.push((x as i32, y as i32));
                }
                symbol => {
                    if symbol != '.' {
                        symbols.insert((x as i32, y as i32), symbol);
                    }
                    if num != 0 {
                        parts.push(Part { num, coords });
                        num = 0;
                        coords = vec![];
                    }
                }
            }
        }
        if num != 0 {
            parts.push(Part { num, coords });
        }
    }
    Field { parts, symbols }
}

fn part1(field: &Field) -> u32 {
    field
        .parts
        .iter()
        .filter_map(|part| {
            if part
                .coords
                .iter()
                .flat_map(neighbours8)
                .any(|coord| field.symbols.contains_key(&coord))
            {
                Some(part.num)
            } else {
                None
            }
        })
        .sum()
}

fn neighbours8(&p: &(i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = p;
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        // (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn part2(field: &Field) -> u32 {
    let mut counts: HashMap<(i32, i32), HashSet<u32>> = HashMap::new();
    for part in &field.parts {
        for coord in part.coords.iter().flat_map(neighbours8) {
            counts.entry(coord).or_default().insert(part.num);
        }
    }
    field
        .symbols
        .iter()
        .filter_map(|symbol| {
            let (&coord, &char) = symbol;
            let parts = counts.get(&coord);
            match (char, parts) {
                ('*', Some(parts)) if parts.len() == 2 => Some(parts.iter().product::<u32>()),
                _ => None,
            }
        })
        .sum()
}
