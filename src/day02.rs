pub fn part1(input: String) -> u32 {
    input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(score)
        .sum()
}

pub fn part2(input: String) -> u32 {
    input.split('\n')
        .map(parse)
        .map(score)
        .sum()
}

fn parse(line: &str) -> (&str, &str) {
    match line.split_once(' ').unwrap() {
        ("A", "X") => ("A", "Z"),
        ("A", "Y") => ("A", "X"),
        ("A", "Z") => ("A", "Y"),
        ("B", "X") => ("B", "X"),
        ("B", "Y") => ("B", "Y"),
        ("B", "Z") => ("B", "Z"),
        ("C", "X") => ("C", "Y"),
        ("C", "Y") => ("C", "Z"),
        ("C", "Z") => ("C", "X"),
        (_,_) => ("invalid", "invalid")
    }
}

fn score(shapes: (&str, &str)) -> u32 {
    match (shapes.0, shapes.1) {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => 0,
    }
}
