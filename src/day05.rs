use once_cell::sync::OnceCell;
use regex::Regex;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn regex() -> &'static Regex {
    static INSTANCE: OnceCell<Regex> = OnceCell::new();
    INSTANCE.get_or_init(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap())
}

pub fn part1(input: String) -> String {
    let mut split = input.split("\n\n");
    let mut stacks = parse_stacks(split.next().unwrap());
    split
        .next()
        .unwrap()
        .lines()
        .map(parse_instruction)
        .for_each(|instruction| apply_instruction(instruction, &mut stacks));
    format_output(&stacks)
}

pub fn part2(input: String) -> String {
    let mut split = input.split("\n\n");
    let mut stacks = parse_stacks(split.next().unwrap());
    split
        .next()
        .unwrap()
        .lines()
        .map(parse_instruction)
        .for_each(|instruction| apply_instruction_v2(instruction, &mut stacks));
    format_output(&stacks)
}

fn apply_instruction(instruction: Instruction, stacks: &mut [Vec<char>]) {
    (0..instruction.count).for_each(|_| {
        let val = stacks[instruction.from].pop().unwrap();
        stacks[instruction.to].push(val);
    })
}

fn apply_instruction_v2(instruction: Instruction, stacks: &mut [Vec<char>]) {
    (0..instruction.count)
        .map(|_| stacks[instruction.from].pop().unwrap())
        .collect::<Vec<char>>()
        .iter()
        .rev()
        .for_each(|val| stacks[instruction.to].push(*val));
}

fn format_output(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn parse_instruction(line: &str) -> Instruction {
    let vec = regex()
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|item| item.unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Instruction {
        count: vec[0],
        from: vec[1] - 1,
        to: vec[2] - 1,
    }
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
