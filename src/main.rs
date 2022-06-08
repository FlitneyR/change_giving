use std::{env, fmt::Display};

struct Solution {
  target: usize,
  coins_used: usize,
  next_coin: Option<usize>
}

impl Solution {
  fn new(target: usize) -> Self {
    Self {
      target: target,
      coins_used: 0,
      next_coin: None
    }
  }

  fn is_possible(&self) -> bool {
    // if there is a next coin, or no change is needed, there is a solution
    self.target == 0 || self.next_coin.is_some()
  }

  fn set_next_coin(&mut self, coin: usize, coins_used: usize) {
    self.next_coin = Some(coin);
    self.coins_used = coins_used;
  }
}

fn solutions(coins: Vec<usize>) -> Vec<Solution> {
  // sort coins, largets to smallest
  // optimisation for later
  let mut coins = coins.clone();
  coins.sort();
  coins.reverse();

  // calculate max target (sum of coin values)
  let max_target: usize = coins.iter()
    .fold(0, |a, b| a + b);

  // build mutable solutions for each target
  let mut solutions: Vec<Solution> = (0..=max_target)
    .map(|t| Solution::new(t)).collect();

  // for each coin from largest to smallest
  for coin in coins.iter() {
    // for each target from largest to smallest
    for target in (0..=max_target).rev() {
      // subgoal <- target - coin
      let subgoal = target.checked_sub(*coin);
      if subgoal.is_none() { continue }
      let subgoal = subgoal.unwrap();

      let subgoal_solution: Option<&Solution> = solutions.get(subgoal);

      // if subgoal is possible then goal is possible with this coin
      let subgoal_is_possible: bool = subgoal_solution
        .map(|s| s.is_possible())
        .unwrap_or(false);

      if subgoal_is_possible {
        let solution = &solutions[target];

        // if new coin will reduce the number of coins, update the next coin
        let subgoal = subgoal_solution.unwrap();
        let new_coins_used = subgoal.coins_used + 1;
        if solution.is_possible() && new_coins_used >= solution.coins_used { continue }

        solutions[target].set_next_coin(*coin, new_coins_used);
      }
      // otherwise try the next coin
    }
  }

  // return solutions
  solutions
}

fn coins_for(target: usize, solutions: &Vec<Solution>) -> Option<Vec<usize>> {
  let mut target = target;
  let solution = solutions.get(target)?;

  if !solution.is_possible() { return None }

  let mut ret = vec![];

  let mut next_coin = solution.next_coin;

  while let Some(coin) = next_coin {
    target -= coin;
    ret.push(coin);
    let solution = solutions.get(target);
    if let Some(solution) = solution {
      next_coin = solution.next_coin
    } else { next_coin = None }
  }

  Some(ret)
}

trait StringJoinable<T> {
  fn join_with(&mut self, joiner: &str) -> String;
}

impl<I, T> StringJoinable<T> for I where I: Iterator<Item = T>, T: Display {
    fn join_with(&mut self, joiner: &str) -> String {
        let mut ret = "".to_string();
        let mut next = self.next();
        while let Some(value) = next {
          ret.push_str(&*value.to_string());
          next = self.next();
          if next.is_some() {
            ret.push_str(&*joiner.to_string());
          }
        }
        ret
    }
}

fn main() {

  // read coins from command
  let coins: Vec<usize> = env::args()
    .map(|s| s.parse())
    .flatten().collect();
  
  // calculate solutions
  let solutions: Vec<Solution> = solutions(coins);

  // print solutions
  for target in 0..solutions.len() {
    let solution = solutions.get(target).unwrap();
    // if !solution.is_possible() { continue }

    print!("{}: ", solution.target);
    
    if let Some(coins) = coins_for(target, &solutions) {
      if coins.len() == 0 { print!("no change needed") }
      print!("{}", coins.iter().join_with(", "))
    } else {
      print!("not possible")
    }

    println!();
  }
}
