extern crate regex;

use std::collections::HashMap;

use regex::Regex;

use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone)]
pub struct Process {
    pub input: HashMap<String, u64>,
    pub output: (String, u64),
}

static MIN_COST: AtomicU64 = AtomicU64::new(2310736);

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();
    input
        .lines()
        .map(|l| {
            lazy_static! {
                static ref REG: Regex =
                    Regex::new(r"((?:[0-9]+ [A-Z]+,?\s?)+)=> ([0-9]+ [A-Z]+)").unwrap();
            }
            let p: Vec<_> = REG.captures(l).unwrap().iter().collect();
            // inputs
            let mut hm_input: HashMap<String, u64> = HashMap::new();
            for c in p[1] {
                for split in c.as_str().split(", ") {
                    let part: Vec<&str> = split.split(" ").collect();
                    hm_input.insert(String::from(part[1]), part[0].parse().unwrap());
                }
            }
            let out: Vec<&str> = p[2].unwrap().as_str().split(" ").collect();
            processes.push(Process {
                input: hm_input,
                output: (String::from(out[1]), out[0].parse().unwrap()),
            });
        })
        .count();
    processes
}

fn find_possible_processes(
    processes: &Vec<Process>,
    resources: &HashMap<String, u64>,
) -> Vec<Process> {
    let mut out: Vec<Process> = Vec::new();
    for p in processes {
        let mut flag: bool = true;
        for (req, cost) in p.input.iter() {
            if req == "ORE" {
                continue;
            }
            if let Some(x) = resources.get(req) {
                if x < cost {
                    flag = false;
                    break;
                }
            } else {
                flag = false;
                break;
            }
        }
        if flag {
            out.push((*p).clone());
        };
    }
    out
}

#[aoc(day14, part1)]
pub fn solve_part1(processes: &Vec<Process>) -> u64 {
    let resources: HashMap<String, u64> = HashMap::new();

    // BFS all possible processes
    // Recursively until we find a solution and all other recursions have exceeded found cost
    println!("{:?}", find_possible_processes(processes, &resources));
    for p in find_possible_processes(processes, &resources) {
        // recurse
        part1_recurse(p, processes, resources.clone(), 0);
    }

    MIN_COST.load(Ordering::Relaxed)
}

fn part1_recurse(
    process: Process,
    processes: &Vec<Process>,
    mut resources: HashMap<String, u64>,
    mut ore_cost: u64,
) -> () {
    // update resources and ore_cost
    if let Some(x) = process.input.get("ORE") {
        ore_cost += x;
    }
    for (req, cost) in process.input.iter() {
        if req == "ORE" {
            continue;
        }
        *(resources.get_mut(req).unwrap()) -= cost;
    }

    if resources.contains_key(&process.output.0) {
        *(resources.get_mut(&process.output.0).unwrap()) += process.output.1;
    } else {
        resources.insert(process.output.0, process.output.1);
    }

    if ore_cost >= MIN_COST.load(Ordering::Relaxed) {
        return;
    }
    if let Some(_x) = resources.get("FUEL") {
        MIN_COST.fetch_min(ore_cost, Ordering::Relaxed);
    }
    for p in find_possible_processes(processes, &resources) {
        // recurse
        part1_recurse(p, processes, resources.clone(), ore_cost);
    }
}

#[cfg(test)]
mod day14tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            solve_part1(&mut input_generator(
                "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            )),
            31
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            solve_part1(&mut input_generator(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            )),
            13312
        );
    }
    #[test]
    fn sample3() {
        assert_eq!(
            solve_part1(&mut input_generator(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
            )),
            180697
        );
    }
    #[test]
    fn sample4() {
        assert_eq!(
            solve_part1(&mut input_generator(
                "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
            )),
            2210736
        );
    }
}
