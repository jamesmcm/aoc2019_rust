use crate::VM;
use num::signum;
// use std::io::stdin;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

struct Arcade {
    cpu: VM,
}

impl Arcade {
    fn new(program: Vec<i64>, label: String) -> Arcade {
        Arcade {
            cpu: VM::new(program, vec![], label),
        }
    }

    fn run(&mut self) -> () {
        self.cpu.run();
    }

    fn parse_output(&self) -> Vec<(i64, i64, i64)> {
        let mut output: Vec<(i64, i64, i64)> = Vec::new();
        for x in self.cpu.output.chunks(3) {
            output.push((x[0], x[1], x[2]));
        }
        output
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    let mut arcade: Arcade = Arcade::new(input.to_vec(), String::from("arcade1"));
    arcade.run();
    let x: Vec<i64> = arcade.parse_output().iter().map(|x| x.0).collect();
    let y: Vec<i64> = arcade.parse_output().iter().map(|x| x.1).collect();
    println!("X: {:?}, {:?}", x.iter().min(), x.iter().max());
    println!("Y: {:?}, {:?}", y.iter().min(), y.iter().max());
    arcade.parse_output().iter().filter(|x| x.2 == 2).count()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut screen: Vec<Vec<u64>> = Vec::new();
    let mut score: i64 = 0;
    // 44x20
    for _y in 0..20 {
        screen.push(vec![0; 44]);
    }
    let mut paddle_x: usize = 21;
    let mut ball_x: usize = 21;
    let mut ball_y: usize = 5;
    let mut blocks_left: usize;

    let mut arcade: Arcade = Arcade::new(input.to_vec(), String::from("arcade1"));
    arcade.cpu.ram[0] = 2;
    arcade.run();
    blocks_left = 0;
    for ins in arcade.parse_output() {
        match ins {
            (-1, 0, x) => {
                score = x;
            }
            (x, y, 2) => {
                blocks_left += 1;
                screen[y as usize][x as usize] = 2 as u64;
            }
            (x, y, 3) => {
                paddle_x = x as usize;
                screen[y as usize][x as usize] = 3 as u64;
            }
            (x, y, 4) => {
                ball_x = x as usize;
                ball_y = y as usize;
                screen[y as usize][x as usize] = 4 as u64;
            }
            (x, y, v) => {
                screen[y as usize][x as usize] = v as u64;
            }
        }
    }

    // Print screen
    // for l in screen.iter() {
    //     println!("{:?}", l);
    // }

    while arcade.cpu.blocked && blocks_left > 0 {
        // Get input
        // let mut s = String::new();
        // stdin()
        //     .read_line(&mut s)
        //     .expect("Did not enter a correct string");

        // let input: char = s.chars().next().unwrap();
        // let i: i64 = match input {
        //     'a' => -1,
        //     's' => 0,
        //     'd' => 1,
        //     _ => 0,
        // };
        arcade.cpu.input = vec![signum(ball_x as i64 - paddle_x as i64)];

        // Run VM
        arcade.run();

        // Update screen
        for ins in arcade.parse_output() {
            match ins {
                (-1, 0, x) => {
                    score = x;
                }
                (x, y, 2) => {
                    screen[y as usize][x as usize] = 2 as u64;
                }
                (x, y, 3) => {
                    paddle_x = x as usize;
                    screen[y as usize][x as usize] = 3 as u64;
                }
                (x, y, 4) => {
                    ball_x = x as usize;
                    screen[y as usize][x as usize] = 4 as u64;
                }
                (x, y, v) => {
                    screen[y as usize][x as usize] = v as u64;
                }
            }
        }

        blocks_left = screen
            .iter()
            .map(|v| v.iter().filter(|x| **x == 2).count())
            .sum();

        // Print screen
        //for l in screen.iter() {
        //    println!("{:?}", l);
        //}
        println!(
            "Paddle X: {:?}, Ball: {:?}, Blocks: {:?}",
            paddle_x,
            (ball_x, ball_y),
            blocks_left
        );
    }
    score
}
