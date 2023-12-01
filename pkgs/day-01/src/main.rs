fn main() {
    let input = include_str!("in").trim();
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            return digits.first().unwrap() * 10 + digits.last().unwrap();
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            return digits.first().unwrap() * 10 + digits.last().unwrap();
        })
        .sum()
}
