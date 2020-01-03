use crate::VM;
use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

macro_attr! {
#[derive(Debug, PartialEq, Eq, NextVariant!, PrevVariant!)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}
}

type Position = (isize, isize);

struct Robot {
    direction: Direction,
    position: Position,
    cpu: VM,
    painted: HashMap<Position, bool>,
}

impl Robot {
    fn new(program: Vec<i64>, label: String) -> Robot {
        Robot {
            direction: Direction::Up,
            position: (0, 0),
            cpu: VM::new(program, vec![0], label),
            painted: HashMap::new(),
        }
    }

    fn run(&mut self) -> () {
        // Loop calling this function until cpu not input blocked
        self.cpu.run();

        // Handle output
        if self.cpu.output.len() == 2 {
            match self.cpu.output.pop() {
                Some(0) => {
                    // turn left
                    self.direction = match self.direction.prev_variant() {
                        Some(x) => x,
                        None => Direction::Left,
                    }
                }
                Some(1) => {
                    // turn right
                    self.direction = match self.direction.next_variant() {
                        Some(x) => x,
                        None => Direction::Up,
                    }
                }
                _ => {}
            }
        }

        match self.cpu.output.pop() {
            Some(0) => {
                // paint black
                self.painted.insert(self.position, false);
            }
            Some(1) => {
                // paint white
                self.painted.insert(self.position, true);
            }
            _ => {}
        }

        // Move forward
        match self.direction {
            Direction::Up => {
                self.position = (self.position.0, self.position.1 + 1);
            }
            Direction::Down => {
                self.position = (self.position.0, self.position.1 - 1);
            }
            Direction::Left => {
                self.position = (self.position.0 - 1, self.position.1);
            }
            Direction::Right => {
                self.position = (self.position.0 + 1, self.position.1);
            }
        }

        // Generate next input
        self.cpu
            .input
            .push(if let Some(x) = self.painted.get(&self.position) {
                *x as i64
            } else {
                0
            });
    }

    fn loop_run(&mut self) -> () {
        self.run();
        while self.cpu.blocked {
            self.run();
        }
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[i64]) -> usize {
    let mut robot: Robot = Robot::new(input.to_vec(), String::from("robot1"));
    robot.loop_run();
    robot.painted.len()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[i64]) -> String {
    let mut robot: Robot = Robot::new(input.to_vec(), String::from("robot1"));
    robot.cpu.input = vec![1];
    robot.loop_run();
    // println!("{:?}", robot.painted.iter().map(|x| (x.0).1).min());

    let mut s = String::new();
    s.push('\n');
    for y in 0..6 {
        for x in 0..45 {
            match robot.painted.get(&(x, -y)) {
                Some(true) => {
                    s.push('#');
                }
                _ => {
                    s.push(' ');
                }
            }
        }
        s.push('\n');
    }

    s
}
