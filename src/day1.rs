#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().map(|l| (l / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input.iter().map(recurse_add).sum()
}

pub fn recurse_add(x: &i32) -> i32 {
    let m = (x / 3) - 2;
    if m < 0 {
        0
    } else {
        m + recurse_add(&m)
    }
}
