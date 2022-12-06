use std::collections::HashSet;

pub fn part1(input: String) -> usize {
    find_start_marker(&input, 4)
}

pub fn part2(input: String) -> usize {
    find_start_marker(&input, 14)
}

fn find_start_marker(input: &str, window_size: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    chars
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| is_unique(window))
        .unwrap()
        .0
        + window_size
}

fn is_unique(input: &[char]) -> bool {
    HashSet::<&char>::from_iter(input.iter()).len() == input.len()
}
