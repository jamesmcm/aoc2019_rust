use std::collections::HashMap;
use std::collections::HashSet;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Path = Vec<(Direction, i32)>;
type Position = (i32, i32);

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Path, Path) {
    let mut out = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|w| {
                    let dir: Direction = match w.chars().next() {
                        Some('U') => Direction::Up,
                        Some('D') => Direction::Down,
                        Some('L') => Direction::Left,
                        Some('R') => Direction::Right,
                        _ => Direction::Up, //TODO
                    };
                    let mut iterator = w.chars();
                    iterator.next();
                    let num: i32 = iterator.collect::<String>().parse::<i32>().unwrap();

                    (dir, num)
                })
                .collect::<Path>()
        })
        .collect::<Vec<Path>>();

    (out.remove(0), out.remove(0))
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &(Path, Path)) -> i32 {
    let mut path1_visited = HashSet::<Position>::new();
    let mut curpos: Position = (0, 0);
    let mut crosses = Vec::<Position>::new();

    for p in &input.0 {
        match p {
            (Direction::Up, x) => {
                for z in curpos.1..curpos.1 + x + 1 {
                    path1_visited.insert((curpos.0, z));
                }
                curpos.1 += x;
            }
            (Direction::Down, x) => {
                for z in curpos.1 - x..curpos.1 + 1 {
                    path1_visited.insert((curpos.0, z));
                }
                curpos.1 -= x;
            }
            (Direction::Left, x) => {
                for z in curpos.0 - x..curpos.0 + 1 {
                    path1_visited.insert((z, curpos.1));
                }
                curpos.0 -= x;
            }
            (Direction::Right, x) => {
                for z in curpos.0..curpos.0 + x + 1 {
                    path1_visited.insert((z, curpos.1));
                }
                curpos.0 += x;
            }
        }
    }

    curpos = (0, 0);
    for p in &input.1 {
        match p {
            (Direction::Up, x) => {
                for z in curpos.1..curpos.1 + x + 1 {
                    if path1_visited.contains(&(curpos.0, z)) {
                        crosses.push((curpos.0, z));
                    }
                }
                curpos.1 += x;
            }
            (Direction::Down, x) => {
                for z in curpos.1 - x..curpos.1 + 1 {
                    if path1_visited.contains(&(curpos.0, z)) {
                        crosses.push((curpos.0, z));
                    }
                }
                curpos.1 -= x;
            }
            (Direction::Left, x) => {
                for z in curpos.0 - x..curpos.0 + 1 {
                    if path1_visited.contains(&(z, curpos.1)) {
                        crosses.push((z, curpos.1));
                    }
                }
                curpos.0 -= x;
            }
            (Direction::Right, x) => {
                for z in curpos.0..curpos.0 + x + 1 {
                    if path1_visited.contains(&(z, curpos.1)) {
                        crosses.push((z, curpos.1));
                    }
                }
                curpos.0 += x;
            }
        }
    }
    // println!("{:?}", crosses);
    crosses
        .iter()
        .filter(|x| **x != (0, 0))
        .map(|x| x.0.abs() + x.1.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &(Path, Path)) -> i32 {
    let mut path1_visited = HashMap::<Position, i32>::new();
    let mut curpos: Position = (0, 0);
    let mut crosses = Vec::<(Position, i32)>::new();
    let mut stepcount: i32 = 0;

    for (_cnt, p) in input.0.iter().enumerate() {
        match p {
            (Direction::Up, x) => {
                for z in curpos.1..curpos.1 + x + 1 {
                    path1_visited.insert((curpos.0, z), stepcount);
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.1 += x;
            }
            (Direction::Down, x) => {
                for z in (curpos.1 - x..curpos.1 + 1).rev() {
                    path1_visited.insert((curpos.0, z), stepcount);
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.1 -= x;
            }
            (Direction::Left, x) => {
                for z in (curpos.0 - x..curpos.0 + 1).rev() {
                    path1_visited.insert((z, curpos.1), stepcount);
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.0 -= x;
            }
            (Direction::Right, x) => {
                for z in curpos.0..curpos.0 + x + 1 {
                    path1_visited.insert((z, curpos.1), stepcount);
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.0 += x;
            }
        }
    }

    curpos = (0, 0);
    stepcount = 0;
    // println!("{:?}", path1_visited);
    for (_cnt2, p) in input.1.iter().enumerate() {
        match p {
            (Direction::Up, x) => {
                for z in curpos.1..curpos.1 + x + 1 {
                    if let Some(key) = path1_visited.get(&(curpos.0, z)) {
                        crosses.push(((curpos.0, z), key + stepcount as i32));
                    }
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.1 += x;
            }
            (Direction::Down, x) => {
                for z in (curpos.1 - x..curpos.1 + 1).rev() {
                    if let Some(key) = path1_visited.get(&(curpos.0, z)) {
                        crosses.push(((curpos.0, z), key + stepcount as i32));
                    }
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.1 -= x;
            }
            (Direction::Left, x) => {
                for z in (curpos.0 - x..curpos.0 + 1).rev() {
                    if let Some(key) = path1_visited.get(&(z, curpos.1)) {
                        crosses.push(((z, curpos.1), key + stepcount as i32));
                    }
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.0 -= x;
            }
            (Direction::Right, x) => {
                for z in curpos.0..curpos.0 + x + 1 {
                    if let Some(key) = path1_visited.get(&(z, curpos.1)) {
                        crosses.push(((z, curpos.1), key + stepcount as i32));
                    }
                    stepcount += 1;
                }
                stepcount -= 1;
                curpos.0 += x;
            }
        }
    }
    // println!("{:?}", crosses);
    crosses
        .iter()
        .filter(|x| x.0 != (0, 0))
        .map(|x| x.1)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator("R8,U5,L5,D3\nU7,R6,D4,L4")), 6);
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solve_part2(&input_generator("R8,U5,L5,D3\nU7,R6,D4,L4")),
            30
        );
    }
    #[test]
    fn sample3() {
        assert_eq!(
            solve_part2(&input_generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
    }
    #[test]
    fn sample4() {
        assert_eq!(
            solve_part2(&input_generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
    #[test]
    fn sample5() {
        assert_eq!(solve_part2(&input_generator("R10\nU2,R2,D4")), 8);
    }
}
