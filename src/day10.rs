use core::f64::consts::PI;
use num::integer::Integer;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Hash, Copy)]
pub struct Asteroid {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone)]
pub struct Space {
    width: usize,
    height: usize,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> (Space, Vec<Asteroid>) {
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let numlines: usize = input
        .lines()
        .enumerate()
        .map(|l| {
            for c in l.1.chars().enumerate() {
                if c.1 == '#' {
                    asteroids.push(Asteroid { y: l.0, x: c.0 });
                }
            }
        })
        .count();
    (
        Space {
            width: input.lines().next().unwrap().len(),
            height: numlines,
        },
        asteroids,
    )
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &(Space, Vec<Asteroid>)) -> usize {
    solve_part1_inner(input).0
}

pub fn solve_part1_inner(input: &(Space, Vec<Asteroid>)) -> (usize, (usize, usize)) {
    let asteroids: Vec<Asteroid> = input.1.clone();
    let width: usize = input.0.width;
    let height: usize = input.0.height;

    let mut out: (usize, (usize, usize)) = (0, (0, 0));

    for a in asteroids.iter() {
        let mut set: HashSet<Asteroid> = HashSet::from_iter(asteroids.iter().cloned());

        set.remove(a);
        for b in set.clone() {
            let mut diffx: isize = b.x as isize - a.x as isize;
            let mut diffy: isize = b.y as isize - a.y as isize;

            let gcd: isize = diffx.gcd(&diffy);

            diffx /= gcd;
            diffy /= gcd;

            let mut posx: isize = a.x as isize + (2 * diffx);
            let mut posy: isize = a.y as isize + (2 * diffy);
            // Repeat and pop from set until OOB
            while (posx >= 0 && posx < width as isize) && (posy >= 0 && posy < height as isize) {
                if ((a.x as isize - b.x as isize).signum()
                    != (posx as isize - b.x as isize).signum()
                    || (posx as isize - b.x as isize).signum() == 0)
                    && ((a.y as isize - b.y as isize).signum()
                        != (posy as isize - b.y as isize).signum()
                        || (posy as isize - b.y as isize).signum() == 0)
                    && !(posx == b.x as isize && posy == b.y as isize)
                {
                    let test: Asteroid = Asteroid {
                        y: posy as usize,
                        x: posx as usize,
                    };
                    if set.contains(&test) {
                        set.remove(&test);
                    }
                }
                posx += diffx;
                posy += diffy;
            }
        }

        if set.len() > out.0 {
            out = (set.len(), (a.x, a.y));
        }
        // if set size is greater, then replace output
    }

    println!("{:?}", out);
    out
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &(Space, Vec<Asteroid>)) -> usize {
    let pos: (usize, usize) = solve_part1_inner(&input.clone()).1;
    let sorted = get_destroyed(&input, pos);
    100 * sorted[199].0.x + sorted[199].0.y
}

fn get_destroyed(input: &(Space, Vec<Asteroid>), pos: (usize, usize)) -> Vec<(Asteroid, f64)> {
    let width: usize = input.0.width;
    let height: usize = input.0.height;

    let mut existing: HashSet<Asteroid> = HashSet::from_iter(input.1.iter().cloned());
    let mut destroyed: Vec<Asteroid> = Vec::new();

    let a: Asteroid = Asteroid { y: pos.1, x: pos.0 };

    while existing.len() > 1 {
        let mut set: HashSet<Asteroid> = HashSet::from_iter(existing.iter().cloned());

        set.remove(&a);
        for b in set.clone() {
            let mut diffx: isize = b.x as isize - a.x as isize;
            let mut diffy: isize = b.y as isize - a.y as isize;

            let gcd: isize = diffx.gcd(&diffy);

            diffx /= gcd;
            diffy /= gcd;

            let mut posx: isize = a.x as isize + (2 * diffx);
            let mut posy: isize = a.y as isize + (2 * diffy);
            // Repeat and pop from set until OOB
            while (posx >= 0 && posx < width as isize) && (posy >= 0 && posy < height as isize) {
                if ((a.x as isize - b.x as isize).signum()
                    != (posx as isize - b.x as isize).signum()
                    || (posx as isize - b.x as isize).signum() == 0)
                    && ((a.y as isize - b.y as isize).signum()
                        != (posy as isize - b.y as isize).signum()
                        || (posy as isize - b.y as isize).signum() == 0)
                    && !(posx == b.x as isize && posy == b.y as isize)
                {
                    let test: Asteroid = Asteroid {
                        y: posy as usize,
                        x: posx as usize,
                    };
                    if set.contains(&test) {
                        set.remove(&test);
                    }
                }
                posx += diffx;
                posy += diffy;
            }
        }

        for d in set {
            existing.remove(&d);
            destroyed.push(d);
        }
        // TODO: Sort here
    }

    let mut sorted: Vec<(Asteroid, f64)> = destroyed
        .iter()
        .cloned()
        .map(|c| {
            let mut angle: f64 = ((c.y as f64 - a.y as f64) as f64).atan2(c.x as f64 - a.x as f64);

            if angle >= 0.0 && angle <= PI / 2.0 {
                angle = (PI / 2.0) - angle;
            } else {
                if angle > PI / 2.0 {
                    angle = (2.0 * PI) - angle;
                } else {
                    angle = angle.abs() + (PI / 2.0);
                }
            }

            if c.x == a.x {
                angle = 0.0;
            }
            (c, angle)
        })
        .collect();

    sorted.sort_by(|c, d| c.1.partial_cmp(&d.1).unwrap());
    println!("{:?}", sorted);
    sorted
}

#[cfg(test)]
mod day10tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            solve_part1(&input_generator(
                ".#..#
.....
#####
....#
...##"
            )),
            8
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            solve_part1(&input_generator(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            )),
            33
        );
    }
    #[test]
    fn sample3() {
        assert_eq!(
            solve_part1(&input_generator(
                "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            )),
            35
        );
    }
    #[test]
    fn sample4() {
        assert_eq!(
            solve_part1(&input_generator(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            )),
            41
        );
    }
    #[test]
    fn sample5() {
        assert_eq!(
            solve_part1(&input_generator(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            )),
            210
        );
    }
    #[test]
    fn sample6() {
        assert_eq!(
            solve_part2(&input_generator(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            )),
            802
        );
    }
    #[test]
    fn sample7() {
        assert_eq!(
            get_destroyed(
                &input_generator(
                    ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"
                ),
                (8, 3)
            )[1]
            .0,
            Asteroid { y: 0, x: 9 }
        );
    }
}
