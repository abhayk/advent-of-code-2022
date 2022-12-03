use std::collections::HashSet;

pub fn part1(input: String) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(r1, r2)| (char_set(r1), char_set(r2)))
        .map(|(s1, s2)| common_element(&[s1, s2]))
        .map(priority)
        .sum()
}

pub fn part2(input: String) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|arr| {
            arr.iter()
                .map(|line| char_set(line))
                .collect::<Vec<HashSet<char>>>()
        })
        .map(|char_sets| common_element(&char_sets))
        .map(priority)
        .sum()
}

fn char_set(line: &str) -> HashSet<char> {
    HashSet::from_iter(line.chars())
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn common_element(sets: &[HashSet<char>]) -> char {
    let mut result = sets[0].clone();
    result.retain(|c| sets.iter().all(|set| set.contains(c)));
    *result.iter().next().unwrap()
}
