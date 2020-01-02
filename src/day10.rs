use num::integer::Integer;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Hash)]
pub struct Asteroid {
    y: usize,
    x: usize,
}

#[derive(Debug)]
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
    out.0
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
}
