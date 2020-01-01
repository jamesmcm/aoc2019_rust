use crate::intcode_vm;
use crate::VM;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut phases: Vec<Vec<i64>> = Vec::new();
    let mut max_signal: i64 = 0;

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
        let ram: Vec<i64> = input.to_vec();
        let input_vec: Vec<i64> = vec![0, p[0]];
        let output: Vec<i64> = intcode_vm(&ram, input_vec).1;

        let input_vec: Vec<i64> = vec![output[0], p[1]];
        let output: Vec<i64> = intcode_vm(&ram, input_vec).1;
        let input_vec: Vec<i64> = vec![output[0], p[2]];
        let output: Vec<i64> = intcode_vm(&ram, input_vec).1;
        let input_vec: Vec<i64> = vec![output[0], p[3]];
        let output: Vec<i64> = intcode_vm(&ram, input_vec).1;
        let input_vec: Vec<i64> = vec![output[0], p[4]];
        let output: Vec<i64> = intcode_vm(&ram, input_vec).1;
        if output[0] > max_signal {
            max_signal = output[0];
        }
    }

    println!("{:?}", max_signal);
    max_signal
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut phases: Vec<Vec<i64>> = Vec::new();
    let mut max_phases: Vec<i64> = Vec::new();
    let mut max_signal: i64 = 0;

    for i in 5..10 {
        for j in 5..10 {
            if j == i {
                continue;
            }
            for k in 5..10 {
                if k == i || k == j {
                    continue;
                }
                for l in 5..10 {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    for m in 5..10 {
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
        let mut vms: Vec<VM> = Vec::new();
        let mut output: i64;
        // Initialise with phases
        for x in 1..6 {
            vms.push(VM {
                ram: input.to_vec(),
                pc: 0,
                output: Vec::new(),
                input: Vec::new(),
                label: ("VM".to_string() + &x.to_string()),
                blocked: false,
                relative_base: 0,
            });
        }

        vms[0].input = vec![0, p[0]];
        vms[0].run();
        if vms[0].output.len() > 1 {
            println!("Bad len VM1 output: {:?}", vms[0].output);
        }
        output = vms[0].output.pop().unwrap();
        vms[1].input = vec![output, p[1]];
        vms[1].run();
        if vms[1].output.len() > 1 {
            println!("Bad len VM2 output: {:?}", vms[1].output);
        }
        output = vms[1].output.pop().unwrap();
        vms[2].input = vec![output, p[2]];
        vms[2].run();
        if vms[2].output.len() > 1 {
            println!("Bad len VM3 output: {:?}", vms[2].output);
        }
        output = vms[2].output.pop().unwrap();
        vms[3].input = vec![output, p[3]];
        vms[3].run();
        if vms[3].output.len() > 1 {
            println!("Bad len VM4 output: {:?}", vms[3].output);
        }
        output = vms[3].output.pop().unwrap();
        vms[4].input = vec![output, p[4]];
        vms[4].run();
        if vms[4].output.len() > 1 {
            println!("Bad len VM5 output: {:?}", vms[4].output);
        }
        output = vms[4].output.pop().unwrap();

        while vms[0].blocked {
            vms[0].input = vec![output];
            vms[0].run();
            if vms[0].output.len() > 1 {
                println!("Bad len VM1 output: {:?}", vms[0].output);
            }
            output = vms[0].output.pop().unwrap();
            vms[1].input = vec![output];
            vms[1].run();
            if vms[1].output.len() > 1 {
                println!("Bad len VM2 output: {:?}", vms[1].output);
            }
            output = vms[1].output.pop().unwrap();
            vms[2].input = vec![output];
            vms[2].run();
            if vms[2].output.len() > 1 {
                println!("Bad len VM3 output: {:?}", vms[2].output);
            }
            output = vms[2].output.pop().unwrap();
            vms[3].input = vec![output];
            vms[3].run();
            if vms[3].output.len() > 1 {
                println!("Bad len VM4 output: {:?}", vms[3].output);
            }
            output = vms[3].output.pop().unwrap();
            vms[4].input = vec![output];
            vms[4].run();
            if vms[4].output.len() > 1 {
                println!("Bad len VM5 output: {:?}", vms[4].output);
            }
            output = vms[4].output.pop().unwrap();
        }

        if output > max_signal {
            max_signal = output;
            max_phases = p.clone();
        }
    }

    println!("{:?}: {:?}", max_signal, max_phases);
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
    #[test]
    fn test_part2_1() {
        assert_eq!(
            solve_part2(&input_generator(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            )),
            139629729
        );
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(
            solve_part2(&input_generator(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            )),
            18216
        );
    }
}
