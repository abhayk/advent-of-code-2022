use std::collections::HashSet;

pub fn part1(input: String) -> usize {
    find_start_marker(&input, 4)
}

pub fn part2(input: String) -> usize {
    find_start_marker(&input, 14)
}

fn find_start_marker(input: &str, window_size: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    chars.windows(window_size).position(all_unique).unwrap() + window_size
}

fn all_unique(input: &[char]) -> bool {
    HashSet::<&char>::from_iter(input.iter()).len() == input.len()
}
