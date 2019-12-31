use crate::intcode_vm;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    let mut phases: Vec<Vec<i32>> = Vec::new();
    let mut max_signal: i32 = 0;

    for i in 0..5 {
        for j in 0..5 {
            if j == i {
                continue;
            }
            for k in 0..5 {
                if k == i || k == j {
                    continue;
                }
                for l in 0..5 {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    for m in 0..5 {
                        if m == i || m == j || m == k || m == l {
                            continue;
                        }
                        phases.push(vec![i, j, k, l, m]);
                    }
                }
            }
        }
    }

    for p in phases {
        let ram: Vec<i32> = input.to_vec();
        let input_vec: Vec<i32> = vec![0, p[0]];
        let output: Vec<i32> = intcode_vm(&ram, input_vec).1;

        let input_vec: Vec<i32> = vec![output[0], p[1]];
        let output: Vec<i32> = intcode_vm(&ram, input_vec).1;
        let input_vec: Vec<i32> = vec![output[0], p[2]];
        let output: Vec<i32> = intcode_vm(&ram, input_vec).1;
        let input_vec: Vec<i32> = vec![output[0], p[3]];
        let output: Vec<i32> = intcode_vm(&ram, input_vec).1;
        let input_vec: Vec<i32> = vec![output[0], p[4]];
        let output: Vec<i32> = intcode_vm(&ram, input_vec).1;
        if output[0] > max_signal {
            max_signal = output[0];
        }
    }

    println!("{:?}", max_signal);
    max_signal
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_thrust1() {
        assert_eq!(
            solve_part1(&input_generator(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            )),
            43210
        );
    }
    #[test]
    fn test_thrust2() {
        assert_eq!(
            solve_part1(&input_generator(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            )),
            54321
        );
    }
    #[test]
    fn test_thrust3() {
        assert_eq!(
            solve_part1(&input_generator(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )),
            65210
        );
    }
}
