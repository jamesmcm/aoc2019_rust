extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2019 }

pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub struct VM {
    ram: Vec<i64>,
    pc: usize,
    pub output: Vec<i64>,
    pub input: Vec<i64>,
    label: String,
    blocked: bool,
    relative_base: i64,
}

impl VM {
    fn run(&mut self) -> () {
        loop {
            let opcode: i64 = self.ram[self.pc] % 100;
            let param_modes: Vec<i64> = vec![
                (self.ram[self.pc] / 100) % 10,
                (self.ram[self.pc] / 1000) % 10,
                (self.ram[self.pc] / 10000) % 10,
            ];
            // println!("{:?}, {:?}", opcode, param_modes);
            match opcode {
                1 => {
                    // ADD
                    let mut dest: usize = 0;
                    let src1: usize = self.ram[self.pc + 1] as usize;
                    let src2: usize = self.ram[self.pc + 2] as usize;
                    let mut val1: i64 = 0;
                    let mut val2: i64 = 0;

                    match param_modes[0] {
                        0 => {
                            val1 = self.ram[src1];
                        }
                        1 => {
                            val1 = src1 as i64;
                        }
                        2 => {
                            val1 = self.ram[(src1 as i64 + self.relative_base) as usize];
                        }
                        _ => {}
                    }
                    match param_modes[1] {
                        0 => {
                            val2 = self.ram[src2];
                        }
                        1 => {
                            val2 = src2 as i64;
                        }
                        2 => {
                            val2 = self.ram[(src2 as i64 + self.relative_base) as usize];
                        }
                        _ => {}
                    }
                    match param_modes[2] {
                        0 => {
                            dest = self.ram[self.pc + 3] as usize;
                        }
                        1 => {
                            dest = self.ram[self.pc + 3] as usize;
                        }
                        2 => {
                            dest = (self.ram[self.pc + 3] + self.relative_base) as usize;
                        }
                        _ => {}
                    }
                    self.ram[dest] = val1 + val2;
                    self.pc += 4;
                }
                2 => {
                    // MUL
                    let mut dest: usize = 0;
                    let src1: usize = self.ram[self.pc + 1] as usize;
                    let src2: usize = self.ram[self.pc + 2] as usize;
                    let mut val1: i64 = 0;
                    let mut val2: i64 = 0;

                    // TODO DRY
                    match param_modes[0] {
                        0 => {
                            val1 = self.ram[src1];
                        }
                        1 => {
                            val1 = src1 as i64;
                        }
                        2 => {
                            val1 = self.ram[(src1 as i64 + self.relative_base) as usize];
                        }
                        _ => {}
                    }
                    match param_modes[1] {
                        0 => {
                            val2 = self.ram[src2];
                        }
                        1 => {
                            val2 = src2 as i64;
                        }
                        2 => {
                            val2 = self.ram[(src2 as i64 + self.relative_base) as usize];
                        }
                        _ => {}
                    }
                    match param_modes[2] {
                        0 => {
                            dest = self.ram[self.pc + 3] as usize;
                        }
                        1 => {
                            dest = self.ram[self.pc + 3] as usize;
                        }
                        2 => {
                            dest = (self.ram[self.pc + 3] + self.relative_base) as usize;
                        }
                        _ => {}
                    }
                    self.ram[dest] = val1 * val2;
                    self.pc += 4;
                }
                3 => {
                    // IN
                    let dest: usize;

                    match param_modes[0] {
                        0 => {
                            dest = self.ram[self.pc + 1] as usize;
                        }
                        1 => {
                            dest = self.ram[self.pc + 1] as usize;
                        }
                        2 => {
                            dest = (self.ram[self.pc + 1] + self.relative_base) as usize;
                        }
                        _ => {
                            dest = 0;
                        }
                    }

                    if let Some(x) = self.input.pop() {
                        self.blocked = false;
                        self.ram[dest] = x;
                        self.pc += 2;
                    } else {
                        // println!("Blocking for input: {}", self.label);
                        self.blocked = true;
                        break;
                    }
                }
                4 => {
                    // OUT
                    match param_modes[0] {
                        0 => {
                            self.output
                                .push(self.ram[self.ram[self.pc + 1 as usize] as usize]);
                        }
                        1 => {
                            self.output.push(self.ram[self.pc + 1]);
                        }
                        2 => {
                            self.output.push(
                                self.ram[(self.ram[self.pc + 1 as usize] + self.relative_base)
                                    as usize],
                            );
                        }
                        _ => {}
                    }

                    self.pc += 2;
                }
                5 => {
                    // JNZ
                    let mut check: i64 = 0;

                    match param_modes[0] {
                        0 => {
                            check = self.ram[self.ram[self.pc + 1 as usize] as usize];
                        }
                        1 => {
                            check = self.ram[self.pc + 1 as usize];
                        }
                        2 => {
                            check = self.ram
                                [(self.ram[self.pc + 1 as usize] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    if check != 0 {
                        match param_modes[1] {
                            0 => {
                                self.pc =
                                    self.ram[self.ram[self.pc + 2 as usize] as usize] as usize;
                            }
                            1 => {
                                self.pc = self.ram[self.pc + 2 as usize] as usize;
                            }
                            2 => {
                                self.pc = self.ram
                                    [(self.ram[self.pc + 2 as usize] + self.relative_base) as usize]
                                    as usize;
                            }
                            _ => {}
                        }
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    // JEZ
                    let mut check: i64 = 0;

                    match param_modes[0] {
                        0 => {
                            check = self.ram[self.ram[self.pc + 1 as usize] as usize];
                        }
                        1 => {
                            check = self.ram[self.pc + 1 as usize];
                        }
                        2 => {
                            check = self.ram
                                [(self.ram[self.pc + 1 as usize] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    if check == 0 {
                        match param_modes[1] {
                            0 => {
                                self.pc =
                                    self.ram[self.ram[self.pc + 2 as usize] as usize] as usize;
                            }
                            1 => {
                                self.pc = self.ram[self.pc + 2 as usize] as usize;
                            }
                            2 => {
                                self.pc = self.ram
                                    [(self.ram[self.pc + 2 as usize] + self.relative_base) as usize]
                                    as usize;
                            }
                            _ => {}
                        }
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    // LT
                    let mut check: i64 = 0;
                    let mut check2: i64 = 0;
                    let mut dest: usize = 0;

                    match param_modes[0] {
                        0 => {
                            check = self.ram[self.ram[self.pc + 1 as usize] as usize];
                        }
                        1 => {
                            check = self.ram[self.pc + 1];
                        }
                        2 => {
                            check = self.ram[(self.ram[self.pc + 1] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    match param_modes[1] {
                        0 => {
                            check2 = self.ram[self.ram[self.pc + 2] as usize];
                        }
                        1 => {
                            check2 = self.ram[self.pc + 2];
                        }
                        2 => {
                            check2 =
                                self.ram[(self.ram[self.pc + 2] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    match param_modes[2] {
                        0 => {
                            dest = self.ram[self.pc + 3 as usize] as usize;
                        }
                        1 => {
                            dest = self.ram[self.pc + 3 as usize] as usize;
                        }
                        2 => {
                            dest = (self.ram[self.pc + 3 as usize] + self.relative_base) as usize;
                        }
                        _ => {}
                    }

                    if check < check2 {
                        self.ram[dest] = 1;
                    } else {
                        self.ram[dest] = 0;
                    }
                    self.pc += 4;
                }
                8 => {
                    // EQ
                    let mut check: i64 = 0;
                    let mut check2: i64 = 0;
                    let mut dest: usize = 0;

                    match param_modes[0] {
                        0 => {
                            check = self.ram[self.ram[self.pc + 1 as usize] as usize];
                        }
                        1 => {
                            check = self.ram[self.pc + 1 as usize];
                        }
                        2 => {
                            check = self.ram
                                [(self.ram[self.pc + 1 as usize] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    match param_modes[1] {
                        0 => {
                            check2 = self.ram[self.ram[self.pc + 2 as usize] as usize];
                        }
                        1 => {
                            check2 = self.ram[self.pc + 2 as usize];
                        }
                        2 => {
                            check2 = self.ram
                                [(self.ram[self.pc + 2 as usize] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    match param_modes[2] {
                        0 => {
                            dest = self.ram[self.pc + 3] as usize;
                        }
                        1 => {
                            dest = self.ram[self.pc + 3] as usize;
                        }
                        2 => {
                            dest = (self.ram[self.pc + 3] + self.relative_base) as usize;
                        }
                        _ => {}
                    }

                    if check == check2 {
                        self.ram[dest] = 1;
                    } else {
                        self.ram[dest] = 0;
                    }
                    self.pc += 4;
                }
                9 => {
                    // Increment relative base
                    let mut val: i64 = 0;
                    match param_modes[0] {
                        0 => {
                            val = self.ram[self.ram[self.pc + 1] as usize];
                        }
                        1 => {
                            val = self.ram[self.pc + 1];
                        }
                        2 => {
                            val = self.ram[(self.ram[self.pc + 1] + self.relative_base) as usize];
                        }
                        _ => {}
                    }

                    self.relative_base += val;
                    self.pc += 2;
                }
                99 => {
                    // println!("Terminating: {}", self.label);
                    break;
                }
                _ => {
                    println! {"Error: {}, opcode: {:?}", self.label, opcode};
                    break;
                }
            }
        }
    }
}

pub fn intcode_vm(input: &[i64], input_vec: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut vm: VM = VM {
        ram: input.to_vec(),
        pc: 0,
        output: Vec::new(),
        input: input_vec.clone(),
        label: "IntcodeVM".to_string(),
        blocked: false,
        relative_base: 0,
    };
    vm.ram.append(&mut vec![0; 8000]);
    vm.run();
    (vm.ram, vm.output)
}

#[cfg(test)]
mod intcodevm {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            intcode_vm(&input_generator("1,0,0,0,99"), Vec::new()).0[0],
            2
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            intcode_vm(&input_generator("2,3,0,3,99"), Vec::new()).0[0],
            2
        );
    }
    #[test]
    fn sample3() {
        assert_eq!(
            intcode_vm(&input_generator("2,4,4,5,99,0"), Vec::new()).0[0],
            2
        );
    }
    #[test]
    fn sample4() {
        assert_eq!(
            intcode_vm(&input_generator("1,1,1,4,99,5,6,0,99"), Vec::new()).0[0],
            30
        );
    }
    #[test]
    fn sample5() {
        assert_eq!(
            intcode_vm(
                &input_generator("1,9,10,3,2,3,11,0,99,30,40,50"),
                Vec::new()
            )
            .0[0],
            3500
        );
    }
    #[test]
    fn sample6() {
        assert_eq!(
            intcode_vm(&input_generator("1002,5,3,5,99,33"), Vec::new()).0[5],
            99
        );
    }
    #[test]
    fn sample7() {
        assert_eq!(
            intcode_vm(&input_generator("1101,100,-1,5,99,0"), Vec::new()).0[5],
            99
        );
    }
    #[test]
    fn output_eq8_true() {
        assert_eq!(
            intcode_vm(&input_generator("3,9,8,9,10,9,4,9,99,-1,8"), vec![8]).1[0],
            1
        );
    }
    #[test]
    fn output_eq8_false() {
        assert_eq!(
            intcode_vm(&input_generator("3,9,8,9,10,9,4,9,99,-1,8"), vec![9]).1[0],
            0
        );
    }
    #[test]
    fn output_lt8_true() {
        assert_eq!(
            intcode_vm(&input_generator("3,9,7,9,10,9,4,9,99,-1,8"), vec![6]).1[0],
            1
        );
    }
    #[test]
    fn output_lt8_false() {
        assert_eq!(
            intcode_vm(&input_generator("3,9,7,9,10,9,4,9,99,-1,8"), vec![9]).1[0],
            0
        );
    }
    #[test]
    fn output_eq8_imm_true() {
        assert_eq!(
            intcode_vm(&input_generator("3,3,1108,-1,8,3,4,3,99"), vec![8]).1[0],
            1
        );
    }
    #[test]
    fn output_eq8_imm_false() {
        assert_eq!(
            intcode_vm(&input_generator("3,3,1108,-1,8,3,4,3,99"), vec![9]).1[0],
            0
        );
    }
    #[test]
    fn output_lt8_imm_true() {
        assert_eq!(
            intcode_vm(&input_generator("3,3,1107,-1,8,3,4,3,99"), vec![6]).1[0],
            1
        );
    }
    #[test]
    fn output_lt8_imm_false() {
        assert_eq!(
            intcode_vm(&input_generator("3,3,1107,-1,8,3,4,3,99"), vec![9]).1[0],
            0
        );
    }
    #[test]
    fn jmp_test_true() {
        assert_eq!(
            intcode_vm(
                &input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
                vec![0]
            )
            .1[0],
            0
        );
    }
    #[test]
    fn jmp_test_false() {
        assert_eq!(
            intcode_vm(
                &input_generator("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"),
                vec![3]
            )
            .1[0],
            1
        );
    }
    #[test]
    fn jmp_test_imm_true() {
        assert_eq!(
            intcode_vm(
                &input_generator("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"),
                vec![0]
            )
            .1[0],
            0
        );
    }
    #[test]
    fn jmp_test_imm_false() {
        assert_eq!(
            intcode_vm(
                &input_generator("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"),
                vec![3]
            )
            .1[0],
            1
        );
    }
    #[test]
    fn long_check_lt() {
        assert_eq!(
            intcode_vm(&input_generator("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), vec![5]).1[0],
            999
        );
    }
    #[test]
    fn long_check_eq() {
        assert_eq!(
            intcode_vm(&input_generator("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), vec![8]).1[0],
            1000
        );
    }
    #[test]
    fn long_check_gt() {
        assert_eq!(
            intcode_vm(&input_generator("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), vec![12]).1[0],
            1001
        );
    }
    #[test]
    fn opcode_9_1() {
        assert_eq!(
            intcode_vm(
                &input_generator("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
                vec![]
            )
            .1,
            input_generator("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")
        );
    }
    #[test]
    fn opcode_9_2() {
        assert_eq!(
            (intcode_vm(
                &input_generator("1102,34915192,34915192,7,4,7,99,0"),
                vec![]
            )
            .1
            .pop()
            .unwrap() as f64)
                .log10()
                .floor() as i64,
            15
        );
    }
    #[test]
    fn opcode_9_3() {
        assert_eq!(
            intcode_vm(&input_generator("104,1125899906842624,99"), vec![])
                .1
                .pop()
                .unwrap(),
            1125899906842624
        );
    }
}
