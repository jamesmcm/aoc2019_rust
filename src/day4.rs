use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<i32> {
  let r: Vec<i32> = input.split("-").map(|x| x.parse().unwrap()).collect();
  (r[0] .. r[1] + 1).collect()
}


#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
  input.iter().map(|x| check_int(x) as i32).sum()  
}

fn check_int(x: &i32) -> bool {
  let mut adj_flag: bool = false;

  for window in x.to_string().chars().collect::<Vec<char>>().windows(2) {
    if window[0].to_digit(10).unwrap() > window[1].to_digit(10).unwrap() { 
      return false
      }

    if window[0] == window[1] {
      adj_flag = true
    }
  }
  adj_flag
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
  input.iter().map(|x| check_int2(x) as i32).sum()  
}

fn check_int2(x: &i32) -> bool {
  let mut seen: HashMap<char,bool> = HashMap::new();

  for window in x.to_string().chars().collect::<Vec<char>>().windows(2) {
    if window[0].to_digit(10).unwrap() > window[1].to_digit(10).unwrap() { 
      return false
      }

    if window[0] == window[1] {
      if seen.contains_key(&window[0]){
        *seen.get_mut(&window[0]).unwrap() = false;
      } else {
      seen.insert(window[0], true);
      }
    }
  }
  seen.values().fold(false, |x, y| x|y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert!(check_int(&111111));
    }
    #[test]
    fn sample2() {
        assert!(!check_int(&223450));
    }
    #[test]
    fn sample3() {
        assert!(!check_int(&123789));
    }
    #[test]
    fn sample4() {
        assert!(!check_int2(&123444));
    }
    #[test]
    fn sample5() {
        assert!(check_int2(&111122));
    }
    #[test]
    fn sample6() {
        assert!(check_int2(&112233));
    }
    #[test]
    fn part2_fail() {
        assert!(!check_int2(&111115));
    }
    #[test]
    fn part2_fail2() {
        assert!(!check_int2(&155555));
    }
    #[test]
    fn part2_fail3() {
        assert!(check_int2(&115555));
    }
    #[test]
    fn part2_fail4() {
        assert!(check_int2(&155566));
    }
    #[test]
    fn part2_fail5() {
        assert!(check_int2(&155666));
    }

}
