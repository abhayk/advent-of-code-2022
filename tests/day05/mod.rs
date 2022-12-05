use advent_of_code_2022::day05;

#[test]
fn part1_example() {
    assert_eq!(
        day05::part1(std::fs::read_to_string("tests/day05/example.txt").unwrap()),
        "CMZ"
    );
}

#[test]
fn part1() {
    assert_eq!(
        day05::part1(std::fs::read_to_string("tests/day05/input.txt").unwrap()),
        "NTWZZWHFV"
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        day05::part2(std::fs::read_to_string("tests/day05/example.txt").unwrap()),
        "MCD"
    );
}

#[test]
fn part2() {
    assert_eq!(
        day05::part2(std::fs::read_to_string("tests/day05/input.txt").unwrap()),
        "BRZGFVBTJ"
    );
}
