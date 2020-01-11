extern crate regex;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Process {
    pub input: HashMap<String, u64>,
    pub output: (String, u64),
    pub ore_cost: u64,
}


const COST_LIMIT: u64 = 16_000_000;
const PATH_LIMIT: u32 = 200000;

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
            let mut ore_cost: u64 = 0;
            if let Some(x) = hm_input.remove("ORE") {
                ore_cost = x;
            }
            processes.push(Process {
                input: hm_input,
                output: (String::from(out[1]), out[0].parse().unwrap()),
                ore_cost,
            });
        })
        .count();
    processes
}

fn find_possible_processes_backwards(
    processes: &Vec<Process>,
    resources: &HashMap<String, i64>,
) -> Vec<Process> {
    let mut out: Vec<Process> = Vec::new();
    for p in processes {
        let mut flag: bool = true;
        let (req, cost) = p.output.clone();
            if req == "FUEL" {
                continue;
            }
            if let Some(x) = resources.get(&req) {
               if *x <= 0 {
                    flag = false;
               }
            } else {
                flag = false;
            }
        if flag {
            out.push((*p).clone());
        };
    }
    out.sort_by(|a, b| (a.ore_cost).partial_cmp(&b.ore_cost).unwrap());
    out.reverse();
    out
}


fn calc_total_wastage(p: &Process, r: &HashMap<String, i64>) -> u64 {
    let sub: i64 = p.output.1 as i64 - *(r.get(&p.output.0).unwrap());
    if sub >= 0 {sub as u64} else {0}
}

#[derive(Debug, Clone)]
struct Job {
    process: Process,
    resources: HashMap<String, i64>,
    ore_cost: u64,
    step: u32,
}

impl Job {
    fn new(process: Process, resources: HashMap<String, i64>, ore_cost: u64, step: u32) -> Job {
        Job {process, resources, ore_cost, step}
    }

    fn process_backwards(&mut self, processes: &Vec<Process>, MIN_COST: &mut u64) -> Option<Vec<Job>> {

    // update resources and ore_cost
    self.ore_cost += self.process.ore_cost;
    if self.step > PATH_LIMIT {
        return None
    }

    let (req, cost) = self.process.output.clone();
        if req != "FUEL" {
        *(self.resources.get_mut(&req).unwrap()) -= cost as i64;
    }
    
    for (req, cost) in self.process.input.iter() {
        if req == "ORE" {
            continue;
        }
    if self.resources.contains_key(req) {
        *(self.resources.get_mut(req).unwrap()) += *(cost) as i64;
    } else {
        self.resources.insert(req.clone(), *(cost) as i64);
    }
    }

    if self.ore_cost >= *MIN_COST {
        return None
    }
    if self.resources.is_empty() || self.resources.values().filter(|x| **x>0).count() == 0 {
        if self.ore_cost < *MIN_COST {
            *MIN_COST = self.ore_cost;
        }
        println!("found fuel: {:?}, {:?}", self.ore_cost, MIN_COST);
        return None
    }
    let mut out = Vec::new();
    for p in find_possible_processes_backwards(processes, &self.resources) {
        // recurse
        out.push(Job::new(p, self.resources.clone(), self.ore_cost, self.step + 1));
    }

    out.sort_by(|a,b| calc_total_wastage(&a.process, &a.resources).partial_cmp(&calc_total_wastage(&b.process, &b.resources)).unwrap());
    //out.reverse();
    //Some(vec![out.pop().unwrap()])
   // if calc_total_wastage(&out[0].process, &out[0].resources) == 0 {
    Some(vec![out[0].clone()])

    //}
    //Some(out.iter().cloned().take(4).collect())
    }
}


fn find_possible_processes(
    processes: &Vec<Process>,
    resources: &HashMap<String, i64>,
) -> Vec<Process> {
    let mut out: Vec<Process> = Vec::new();
    for p in processes {
        let mut flag: bool = true;
        if p.input.is_empty(){
            continue;
        }
        for (req, cost) in p.input.iter() {
            if req == "ORE" {
                continue;
            }
            if let Some(x) = resources.get(req) {
                if *x < (*cost as i64) {
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
    //out.sort_by(|a, b| (a.ore_cost).partial_cmp(&b.ore_cost).unwrap());
    //println!("{:?}, {:?}", resources, out);
    out
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Vec<Process>) -> u64 {

    let mut fuel: u64 = 0;
    // Run P1, get ore cost and final resources
    let (ore_cost, mut resources) = solve_part1_inner(input);
    // Divide 1 trillion by ore_cost
    let base_iterations: u64 = 1_000_000_000_000 / ore_cost;
    let remaining_ore: u64 = 1_000_000_000_000 % ore_cost;
    fuel += base_iterations;
    // Multiply remaining resources by this
    resources.iter_mut().for_each(|x| *(x.1) = (x.1).abs() * base_iterations as i64);
    println!("{:?}", resources);
    

    // Run forward with these resources?
    loop {
    let possible = find_possible_processes(input, &resources);
    if possible.is_empty() { break;}

    let p: Process = possible[0].clone();

    // Apply max times
    let mut mindiv: i64 = COST_LIMIT as i64;

    for (req, cost) in p.input.iter() {
        if req == "ORE" {
            continue;
        }
        if let Some(x) = resources.get(req){
        let x = x / *cost as i64;
        if x < mindiv {
            mindiv = x;
        }
        }
    }

    for (req, cost) in p.input.iter() {
        if req == "ORE" {
            continue;
        }
        //println!("{:?}, {:?}", (req, cost), self.resources);
        *(resources.get_mut(req).unwrap()) -= *cost as i64 * mindiv;
    }

    if resources.contains_key(&p.output.0) {
        *(resources.get_mut(&p.output.0).unwrap()) += p.output.1 as i64 * mindiv;
    } else {
        resources.insert(p.output.0.clone(), p.output.1 as i64 * mindiv);
    }
    }
    if let Some(x) = resources.get("FUEL") {
    fuel += *x as u64;
    }

    println!("{:?}, ORE: {:?}", resources, remaining_ore);
    fuel
    }

#[aoc(day14, part1)]
pub fn solve_part1(input: &Vec<Process>) -> u64 {
    solve_part1_inner(input).0
}


fn solve_part1_inner(input: &Vec<Process>) -> (u64, HashMap<String, i64>){
    let mut resources: HashMap<String, i64> = HashMap::new();
    let mut final_resources: HashMap<String, i64> = HashMap::new();
    let mut stack: Vec<Job> = Vec::new();
    let mut init_stack: Vec<Job> = Vec::new();
    let mut MIN_COST: u64 = COST_LIMIT;

    // BFS all possible processes
    // Recursively until we find a solution and all other recursions have exceeded found cost
    let mut processes: Vec<Process> = input.clone();

    let i: usize = processes.iter().position(|x| x.output.0 == "FUEL").unwrap();
    let startp: Process = processes.remove(i);
    
    let mut j1: Job = Job::new(startp, resources.clone(), 0, 0);
    j1.process_backwards(&processes, &mut MIN_COST);
    resources=j1.resources.clone();
    let start_ore_cost: u64 = j1.ore_cost;

    for p in find_possible_processes_backwards(&processes, &resources) {
        init_stack.push(Job::new(p, resources.clone(), start_ore_cost, 1));
    }
    //println!("{:?}", init_stack.iter().map(|x| (x, calc_total_wastage(&x.process, &x.resources))).collect::<Vec<(&Job, u64)>>());
    init_stack.sort_by(|a,b| calc_total_wastage(&a.process, &a.resources).partial_cmp(&calc_total_wastage(&b.process, &b.resources)).unwrap());
    init_stack.reverse();
    stack.push(init_stack.pop().unwrap());
    //println!("{:?}", &stack);

    while !stack.is_empty() {
        let mut origjob: Job =  stack.pop().unwrap();
        if let Some(newjobs) = origjob.process_backwards(&processes, &mut MIN_COST) {

        stack.clear();
        for job in newjobs {
            //println!("{:?}", job);
            stack.push(job);

        }
        } else {
        stack.clear();
        }
        if stack.is_empty(){
            final_resources = origjob.resources.clone();
        }
    }

    (MIN_COST, final_resources)

}


#[cfg(test)]
mod day14tests {
    use super::*;

    #[test]
    fn p1_1() { let res: HashMap<String, i64> = hashmap!{String::from("A") => -2, String::from("B") => 0, String::from("C") => 0, String::from("D") => 0, String::from("E") => 0};
        assert_eq!(
            solve_part1_inner(&mut input_generator(
                "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            )),
            (31, res)
        );
    }
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
    fn sample5() {
        assert_eq!(
            solve_part1(&mut input_generator("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
            )),
            165
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
    #[test]
    fn test_wasteage() {
        let res: HashMap<String, i64> = hashmap!{String::from("A") => 7, String::from("E") => 1};

        assert_eq!(calc_total_wastage(&Process{input: HashMap::new(), output: (String::from("A"), 10), ore_cost: 10}, &res), 3) 
    }
    #[test]
    fn p2_1() {
        assert_eq!(
            solve_part2(&mut input_generator(
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
            82892753
        );
    }
    #[test]
    fn p2_2() {
        assert_eq!(
            solve_part2(&mut input_generator(
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
            5586022
        );
    }
    #[test]
    fn p2_3() {
        assert_eq!(
            solve_part2(&mut input_generator(
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
            460664
        );
    }
}
