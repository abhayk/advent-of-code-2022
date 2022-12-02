use advent_of_code_2022::day01;

#[test]
fn part1_example() {
    assert_eq!(
        day01::part1(std::fs::read_to_string("tests/day01/example.txt").unwrap()),
        24000
    );
}

#[test]
fn part1() {
    assert_eq!(
        day01::part1(std::fs::read_to_string("tests/day01/input.txt").unwrap()),
        66306
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        day01::part2(std::fs::read_to_string("tests/day01/example.txt").unwrap()),
        45000
    );
}

#[test]
fn part2() {
    assert_eq!(
        day01::part2(std::fs::read_to_string("tests/day01/input.txt").unwrap()),
        195292
    );
}
