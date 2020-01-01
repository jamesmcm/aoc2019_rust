use crate::intcode_vm;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    // 1202 change
    let ram: Vec<i64> = input.to_vec();
    let input_vec: Vec<i64> = vec![1];

    let mut output: Vec<i64> = intcode_vm(&ram, input_vec).1;
    println!("{:?}", output);
    output.pop().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    // 1202 change
    let ram: Vec<i64> = input.to_vec();
    let input_vec: Vec<i64> = vec![5];

    let mut output: Vec<i64> = intcode_vm(&ram, input_vec).1;
    println!("{:?}", output);
    output.pop().unwrap()
}
