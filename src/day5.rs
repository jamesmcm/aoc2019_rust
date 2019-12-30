use crate::intcode_vm;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    // 1202 change
    let ram: Vec<i32> = input.to_vec();
    let input_vec: Vec<i32> = vec![1];

    let mut output: Vec<i32> = intcode_vm(&ram, input_vec).1;
    println!("{:?}", output);
    output.pop().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    // 1202 change
    let ram: Vec<i32> = input.to_vec();
    let input_vec: Vec<i32> = vec![5];

    let mut output: Vec<i32> = intcode_vm(&ram, input_vec).1;
    println!("{:?}", output);
    output.pop().unwrap()
}
