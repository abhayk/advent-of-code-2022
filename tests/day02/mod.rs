use advent_of_code_2022::day02;

#[test]
fn part1_example() {
    assert_eq!(
        day02::part1(std::fs::read_to_string("tests/day02/example.txt").unwrap()),
        15
    );
}

#[test]
fn part1() {
    assert_eq!(
        day02::part1(std::fs::read_to_string("tests/day02/input.txt").unwrap()),
        13565
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        day02::part2(std::fs::read_to_string("tests/day02/example.txt").unwrap()),
        12
    );
}

#[test]
fn part2() {
    assert_eq!(
        day02::part2(std::fs::read_to_string("tests/day02/input.txt").unwrap()),
        12424
    );
}
