use regex::Regex;

pub fn part1(input: String) -> String {
    let mut split = input.split("\n\n");
    let mut stacks = parse_stacks(split.next().unwrap());

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    apply_instructions(
        split.next().unwrap(),
        &regex,
        &mut stacks,
        apply_instruction,
    );

    format_output(&stacks)
}

pub fn part2(input: String) -> String {
    let mut split = input.split("\n\n");
    let mut stacks = parse_stacks(split.next().unwrap());

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    apply_instructions(
        split.next().unwrap(),
        &regex,
        &mut stacks,
        apply_instruction_v2,
    );
    format_output(&stacks)
}

fn apply_instructions<F>(input: &str, regex: &Regex, stacks: &mut [Vec<char>], mut func: F)
where
    F: FnMut((usize, usize, usize), &mut [Vec<char>]),
{
    input
        .lines()
        .map(|line| parse_instruction(line, regex))
        .for_each(|(count, from, to)| {
            func((count, from, to), stacks);
        });
}

fn apply_instruction((count, from, to): (usize, usize, usize), stacks: &mut [Vec<char>]) {
    (0..count).for_each(|_| {
        let val = stacks[from].pop().unwrap();
        stacks[to].push(val);
    })
}

fn apply_instruction_v2((count, from, to): (usize, usize, usize), stacks: &mut [Vec<char>]) {
    let mut tmp = Vec::new();
    (0..count).for_each(|_| {
        tmp.push(stacks[from].pop().unwrap());
    });
    tmp.iter().rev().for_each(|val| stacks[to].push(*val));
}

fn format_output(stacks: &[Vec<char>]) -> String {
    stacks
    .iter()
    .map(|stack| stack.last().unwrap())
    .collect::<String>()
}

fn parse_instruction(line: &str, regex: &Regex) -> (usize, usize, usize) {
    let vec = regex
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|item| item.unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (vec[0], vec[1] - 1, vec[2] - 1)
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut crate_iter = input.split('\n').rev();
    crate_iter
        .next()
        .unwrap()
        .trim()
        .split("   ")
        .for_each(|_| stacks.push(Vec::new()));
    crate_iter.for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, val)| *val != ' ')
            .for_each(|(i, val)| stacks[i].push(val))
    });
    stacks
}
