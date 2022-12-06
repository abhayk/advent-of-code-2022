use once_cell::sync::OnceCell;
use regex::Regex;

fn regex() -> &'static Regex {
    static INSTANCE: OnceCell<Regex> = OnceCell::new();
    INSTANCE.get_or_init(|| Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap())
}

pub fn part1(input: String) -> u32 {
    let regex = regex();
    input
        .lines()
        .map(|line| parse(line, regex))
        .filter(|ranges| full_overlap(ranges))
        .count() as u32
}

pub fn part2(input: String) -> u32 {
    let regex = regex();
    input
        .lines()
        .map(|line| parse(line, regex))
        .filter(|ranges| partial_overlap(ranges))
        .count() as u32
}

fn full_overlap(v: &[u32]) -> bool {
    (v[0] <= v[2] && v[1] >= v[3]) || (v[2] <= v[0] && v[3] >= v[1])
}

fn partial_overlap(v: &[u32]) -> bool {
    (v[0] <= v[2] && v[2] <= v[1]) || (v[2] <= v[0] && v[0] <= v[3])
}

fn parse(line: &str, regex: &Regex) -> Vec<u32> {
    regex
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|item| item.unwrap().as_str().parse::<u32>().unwrap())
        .collect()
}
