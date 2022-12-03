use std::collections::BinaryHeap;

pub fn part1(input: String) -> u32 {
    *calories_per_elf(input).iter().max().unwrap()
}

pub fn part2(input: String) -> u32 {
    let mut heap = BinaryHeap::from(calories_per_elf(input));
    (0..3).map(|_| heap.pop().unwrap()).sum()
}

fn calories_per_elf(input: String) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|food| {
            food.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}
