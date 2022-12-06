use advent_of_code_2022::day06;

#[test]
fn part1_example() {
    assert_eq!(
        day06::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
        7
    );
    assert_eq!(day06::part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
    assert_eq!(day06::part1("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
    assert_eq!(
        day06::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
        10
    );
    assert_eq!(
        day06::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
        11
    );
}

#[test]
fn part1() {
    assert_eq!(
        day06::part1(std::fs::read_to_string("tests/day06/input.txt").unwrap()),
        1155
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        day06::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
        19
    );
    assert_eq!(day06::part2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
    assert_eq!(day06::part2("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
    assert_eq!(
        day06::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
        29
    );
    assert_eq!(
        day06::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
        26
    );
}

#[test]
fn part2() {
    assert_eq!(
        day06::part2(std::fs::read_to_string("tests/day06/input.txt").unwrap()),
        2789
    );
}
