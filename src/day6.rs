use core::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;

type NodeMap = HashMap<String, (Vec<String>, Vec<String>)>;
type Item<'a> = (&'a String, &'a (Vec<String>, Vec<String>));

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|x| {
            let y: Vec<&str> = x.split(")").collect();
            (y[0].to_string(), y[1].to_string())
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<(String, String)>) -> i32 {
    let mut nodes: NodeMap = HashMap::new();

    for n in input {
        if nodes.contains_key(&n.0) {
            // Add n.1 to childen
            nodes.get_mut(&n.0).unwrap().1.push(n.1.clone());
        } else {
            nodes.insert(n.0.clone(), (vec![], vec![n.1.clone()]));
        }

        if nodes.contains_key(&n.1) {
            // Add n.0 to parent
            nodes.get_mut(&n.1).unwrap().0.push(n.0.clone());
        } else {
            nodes.insert(n.1.clone(), (vec![n.0.clone()], vec![]));
        }
    }

    let mut direct: i32 = 0;
    let mut indirect: i32 = 0;
    for k in nodes.keys() {
        let x: i32 = nodes.get(k).unwrap().0.len() as i32;
        direct += x;
        indirect += get_num_parents(&k, &nodes) - x;
    }
    direct + indirect
}

fn get_num_parents(key: &String, nodes: &NodeMap) -> i32 {
    nodes
        .get(key)
        .unwrap()
        .0
        .iter()
        .map(|x| 1 + get_num_parents(x, nodes))
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<(String, String)>) -> i32 {
    let mut nodes: NodeMap = HashMap::new();

    for n in input {
        if nodes.contains_key(&n.0) {
            // Add n.1 to childen
            nodes.get_mut(&n.0).unwrap().1.push(n.1.clone());
        } else {
            nodes.insert(n.0.clone(), (vec![], vec![n.1.clone()]));
        }

        if nodes.contains_key(&n.1) {
            // Add n.0 to parent
            nodes.get_mut(&n.1).unwrap().0.push(n.0.clone());
        } else {
            nodes.insert(n.1.clone(), (vec![n.0.clone()], vec![]));
        }
    }

    // BFS from YOU to SAN
    let start: Item = nodes.get_key_value("YOU").unwrap();
    let mut seen: HashSet<String> = HashSet::new();
    bfs(start, &nodes, &mut seen).unwrap() - 2
}

fn bfs(node: Item, nodes: &NodeMap, seen: &mut HashSet<String>) -> Option<i32> {
    if node.0 == "SAN" {
        return Some(0);
    }

    if seen.contains(node.0) {
        return None;
    }
    seen.insert(node.0.clone());

    let mut pars: Option<i32> = None;
    let mut child: Option<i32> = None;

    if !((node.1).0).is_empty() {
        pars = min_option(
            (node.1)
                .0
                .iter()
                .map(|x| bfs(nodes.get_key_value(x).unwrap(), nodes, seen))
                .collect(),
        );
    }

    if !((node.1).1).is_empty() {
        child = min_option(
            (node.1)
                .1
                .iter()
                .map(|x| bfs(nodes.get_key_value(x).unwrap(), nodes, seen))
                .collect(),
        );
    }

    match (pars, child) {
        (None, None) => {
            // println!("{:?}: {:?}", node.0, "None");
            None
        }
        (Some(x), None) => {
            // println!("{:?}: {:?}", node.0, x + 1);
            Some(x + 1)
        }
        (None, Some(y)) => {
            // println!("{:?}: {:?}", node.0, y + 1);
            Some(y + 1)
        }
        (Some(x), Some(y)) => {
            // println!("{:?}: {:?}", node.0, min(x, y) + 1);
            Some(1 + min(x, y))
        }
    }
}

fn min_option(v: Vec<Option<i32>>) -> Option<i32> {
    let y: i32 = v
        .iter()
        .map(|x| if let Some(z) = x { *z } else { 999999 })
        .min()
        .unwrap();
    if y == 999999 {
        None
    } else {
        Some(y)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            solve_part1(&input_generator(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
            )),
            42
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            solve_part2(&input_generator(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            )),
            4
        );
    }
}
