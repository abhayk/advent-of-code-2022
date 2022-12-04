use advent_of_code_2022::day04;

#[test]
fn part1_example() {
    assert_eq!(
        day04::part1(std::fs::read_to_string("tests/day04/example.txt").unwrap()),
        2
    );
}

#[test]
fn part1() {
    assert_eq!(
        day04::part1(std::fs::read_to_string("tests/day04/input.txt").unwrap()),
        503
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        day04::part2(std::fs::read_to_string("tests/day04/example.txt").unwrap()),
        4
    );
}

#[test]
fn part2() {
    assert_eq!(
        day04::part2(std::fs::read_to_string("tests/day04/input.txt").unwrap()),
        827
    );
}
