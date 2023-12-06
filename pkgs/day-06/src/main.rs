fn main() {
    let input = include_str!("in").trim();
    let races = parse(input);
    dbg!(part1(&races));
    dbg!(part2(&races));
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let (time, distance) = input.split_once("\n").unwrap();
    let (_, times) = time.split_once(":").unwrap();
    let times = times
        .split_ascii_whitespace()
        .map(|time| time.parse::<u32>().unwrap());
    let (_, distances) = distance.split_once(":").unwrap();
    let distances = distances
        .split_ascii_whitespace()
        .map(|distance| distance.parse::<u32>().unwrap());
    times.zip(distances).collect()
}

fn part1(races: &[(u32, u32)]) -> usize {
    races
        .iter()
        .map(|&(time, distance)| {
            (0..=time)
                .filter(|speed| speed * (time - speed) > distance)
                .count()
        })
        .product()
}

fn part2(races: &[(u32, u32)]) -> usize {
    let mut time = String::with_capacity(20);
    let mut distance = String::with_capacity(20);
    for &(t, d) in races {
        time += &t.to_string();
        distance += &d.to_string();
    }
    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();
    (0..=time)
        .filter(|speed| speed * (time - speed) > distance)
        .count()
}
