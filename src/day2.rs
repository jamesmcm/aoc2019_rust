use itertools::Itertools;
use rayon::prelude::*;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    // 1202 change
    let mut ram: Vec<i32> = input.to_vec();
    ram[1] = 12;
    ram[2] = 2;
    solve_part1_inner(&ram)
}

fn solve_part1_inner(input: &[i32]) -> i32 {
    let mut ram: Vec<i32> = input.to_vec();
    let mut i: usize = 0;

    while ram[i] != 99 {
        match ram[i] {
            1 => {
                let dest: usize = ram[i + 3] as usize;
                let src1: usize = ram[i + 1] as usize;
                let src2: usize = ram[i + 2] as usize;
                ram[dest] = ram[src1] + ram[src2];
            }
            2 => {
                let dest: usize = ram[i + 3] as usize;
                let src1: usize = ram[i + 1] as usize;
                let src2: usize = ram[i + 2] as usize;
                ram[dest] = ram[src1] * ram[src2];
            }
            99 => {
                break;
            }
            _ => {
                println! {"Error"};
            }
        }
        i += 4;
    }
    ram[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    const CHECK: i32 = 19690720;
    let vals: Vec<(i32, i32)> = (0..100)
        .cartesian_product(0..100)
        .collect::<Vec<(i32, i32)>>();
    let par_iter = vals.into_par_iter().map(|x| solve_part2_inner(input, x));
    let output: Vec<((i32, i32), i32)> = par_iter.filter(|x| x.1 == CHECK).collect();
    100 * (output[0].0).0 + (output[0].0).1
}

fn solve_part2_inner(input: &[i32], vals: (i32, i32)) -> ((i32, i32), i32) {
    let mut ram: Vec<i32> = input.to_vec();
    ram[1] = vals.0;
    ram[2] = vals.1;
    (vals, solve_part1_inner(&ram))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(solve_part1_inner(&input_generator("1,0,0,0,99")), 2);
    }
    #[test]
    fn sample2() {
        assert_eq!(solve_part1_inner(&input_generator("2,3,0,3,99")), 2);
    }
    #[test]
    fn sample3() {
        assert_eq!(solve_part1_inner(&input_generator("2,4,4,5,99,0")), 2);
    }
    #[test]
    fn sample4() {
        assert_eq!(
            solve_part1_inner(&input_generator("1,1,1,4,99,5,6,0,99")),
            30
        );
    }
    #[test]
    fn sample5() {
        assert_eq!(
            solve_part1_inner(&input_generator("1,9,10,3,2,3,11,0,99,30,40,50")),
            3500
        );
    }
}
