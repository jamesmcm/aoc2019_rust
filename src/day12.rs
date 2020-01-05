extern crate regex;

use num::Integer;
use regex::Regex;

type Tuple = (i64, i64, i64);

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Hash)]
pub struct Planet {
    pub position: Tuple,
    pub velocity: Tuple,
}

impl Planet {
    fn new(position: Tuple) -> Planet {
        Planet {
            position,
            velocity: (0, 0, 0),
        }
    }

    fn update_position(&mut self) -> () {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn total_energy(&self) -> u64 {
        let pot_e: u64 =
            (self.position.0.abs() + self.position.1.abs() + self.position.2.abs()) as u64;
        let kin_e: u64 =
            (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()) as u64;
        pot_e * kin_e
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Planet> {
    let mut planets: Vec<Planet> = Vec::new();
    input
        .lines()
        .map(|l| {
            lazy_static! {
                static ref REG: Regex =
                    Regex::new(r"<x=([0-9\-]+), y=([0-9\-]+), z=([0-9\-]+)>").unwrap();
            }
            let p = REG.captures(l).unwrap();
            planets.push(Planet::new((
                p[1].parse().unwrap(),
                p[2].parse().unwrap(),
                p[3].parse().unwrap(),
            )));
        })
        .count();
    planets
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Vec<Planet>) -> u64 {
    let mut planets = input.clone();
    solve_part1_inner(&mut planets, 1000)
}

pub fn solve_part1_inner(planets: &mut Vec<Planet>, limit: usize) -> u64 {
    for _step in 1..limit + 1 {
        let planets2 = planets.clone();
        let mut iter = planets.iter_mut();
        while let Some(p1) = iter.next() {
            for p2 in &planets2 {
                if *p1 == *p2 {
                    continue;
                };

                if p1.position.0 > p2.position.0 {
                    p1.velocity.0 -= 1;
                }
                if p1.position.0 < p2.position.0 {
                    p1.velocity.0 += 1;
                }

                if p1.position.1 > p2.position.1 {
                    p1.velocity.1 -= 1;
                }
                if p1.position.1 < p2.position.1 {
                    p1.velocity.1 += 1;
                }

                if p1.position.2 > p2.position.2 {
                    p1.velocity.2 -= 1;
                }
                if p1.position.2 < p2.position.2 {
                    p1.velocity.2 += 1;
                }
            }
        }
        for p1 in planets.iter_mut() {
            p1.update_position();
        }
    }

    planets.iter().map(|p| p.total_energy()).sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Vec<Planet>) -> u64 {
    let mut planets = input.clone();
    let start = planets.clone();
    let mut periods: (Option<u64>, Option<u64>, Option<u64>) = (None, None, None);

    let mut i: u64 = 1;
    while periods.0.is_none() || periods.1.is_none() || periods.2.is_none() {
        let planets2 = planets.clone();
        let mut iter = planets.iter_mut();
        while let Some(p1) = iter.next() {
            for p2 in &planets2 {
                if *p1 == *p2 {
                    continue;
                };

                if p1.position.0 > p2.position.0 {
                    p1.velocity.0 -= 1;
                }
                if p1.position.0 < p2.position.0 {
                    p1.velocity.0 += 1;
                }

                if p1.position.1 > p2.position.1 {
                    p1.velocity.1 -= 1;
                }
                if p1.position.1 < p2.position.1 {
                    p1.velocity.1 += 1;
                }

                if p1.position.2 > p2.position.2 {
                    p1.velocity.2 -= 1;
                }
                if p1.position.2 < p2.position.2 {
                    p1.velocity.2 += 1;
                }
            }
        }
        for p1 in planets.iter_mut() {
            p1.update_position();
        }

        if planets
            .iter()
            .map(|p| (p.position.0, p.velocity.0))
            .collect::<Vec<(i64, i64)>>()
            == start
                .iter()
                .map(|p| (p.position.0, p.velocity.0))
                .collect::<Vec<(i64, i64)>>()
        {
            periods.0 = Some(i);
        }
        if planets
            .iter()
            .map(|p| (p.position.1, p.velocity.1))
            .collect::<Vec<(i64, i64)>>()
            == start
                .iter()
                .map(|p| (p.position.1, p.velocity.1))
                .collect::<Vec<(i64, i64)>>()
        {
            periods.1 = Some(i);
        }
        if planets
            .iter()
            .map(|p| (p.position.2, p.velocity.2))
            .collect::<Vec<(i64, i64)>>()
            == start
                .iter()
                .map(|p| (p.position.2, p.velocity.2))
                .collect::<Vec<(i64, i64)>>()
        {
            periods.2 = Some(i);
        }

        i += 1;
    }
    // (periods.0.unwrap().lcm(&periods.1.unwrap())).lcm(&periods.2.unwrap())
    println!("{:?}", periods);
    periods
        .0
        .unwrap()
        .lcm(&(periods.1.unwrap().lcm(&periods.2.unwrap())))
    //periods.0.unwrap() * periods.1.unwrap() * periods.2.unwrap()
}

#[cfg(test)]
mod day12tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            solve_part1_inner(
                &mut input_generator(
                    "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"
                ),
                10
            ),
            179
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            solve_part1_inner(
                &mut input_generator(
                    "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"
                ),
                100
            ),
            1940
        );
    }
    #[test]
    fn sample3() {
        assert_eq!(
            solve_part2(&input_generator(
                "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"
            )),
            2772
        );
    }
    #[test]
    fn sample4() {
        assert_eq!(
            solve_part2(&input_generator(
                "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"
            )),
            4686774924
        );
    }
}
