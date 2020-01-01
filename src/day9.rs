use crate::intcode_vm;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut output = intcode_vm(&input, vec![1]).1;
    println!("{:?}", output);
    output.pop().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut output = intcode_vm(&input, vec![2]).1;
    println!("{:?}", output);
    output.pop().unwrap()
}
