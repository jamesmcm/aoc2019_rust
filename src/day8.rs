use itertools::Itertools;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[char]) -> i32 {
    solve_part1_inner(input, 25, 6)
}

fn solve_part1_inner(input: &[char], width: usize, height: usize) -> i32 {
    let chars: Vec<char> = input
        .chunks(width * height)
        .map(|x| (x, x.iter().filter(|y| **y == '0').count()))
        .fold((&['a'][..], 999999), |x, y| if y.1 < x.1 { y } else { x })
        .0
        .to_vec();

    let mut ones: i32 = 0;
    let mut twos: i32 = 0;
    for c in chars {
        match c {
            '1' => ones += 1,
            '2' => twos += 1,
            _ => {}
        }
    }

    ones * twos
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[char]) -> String {
    solve_part2_inner(input, 25, 6)
        .iter()
        .fold(String::from(""), |x, y| x + "\n" + y)
}

fn solve_part2_inner(input: &[char], width: usize, height: usize) -> Vec<String> {
    let mut chunks = input.chunks(width * height);

    let mut output: String = chunks.next().unwrap().iter().cloned().collect();

    for chunk in chunks {
        output = output
            .chars()
            .zip(chunk)
            .map(|x| match x {
                ('2', z) => *z,
                z => z.0,
            })
            .collect()
    }

    let mut out: Vec<String> = Vec::new();
    for x in output.chars().chunks(width).into_iter() {
        out.push(x.collect::<String>());
    }
    out
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_inner1() {
        assert_eq!(solve_part1_inner(&input_generator("123456789012"), 3, 2), 1);
    }
    #[test]
    fn test_inner2() {
        assert_eq!(
            solve_part2_inner(&input_generator("0222112222120000"), 2, 2),
            vec!["01", "10"]
        );
    }
}
