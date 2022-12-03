use advent_of_code_2022::day03;

#[test]
fn part1_example() {
    assert_eq!(
        day03::part1(std::fs::read_to_string("tests/day03/example.txt").unwrap()),
        157
    );
}

#[test]
fn part1() {
    assert_eq!(
        day03::part1(std::fs::read_to_string("tests/day03/input.txt").unwrap()),
        8039
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        day03::part2(std::fs::read_to_string("tests/day03/example.txt").unwrap()),
        70
    );
}

#[test]
fn part2() {
    assert_eq!(
        day03::part2(std::fs::read_to_string("tests/day03/input.txt").unwrap()),
        2510
    );
}
