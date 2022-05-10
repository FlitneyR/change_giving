use std::env;

#[derive(Clone)]
struct Solution {
  target: usize,
  possible: bool,
  necessary_coins: Option<Vec<usize>>
}

trait Countable<T> {
  fn count_where<P>(self: &Self, predicate: P) -> usize where P: Fn(&T) -> bool;
}

impl Countable<usize> for &[usize] {
  fn count_where<P>(&self, predicate: P) -> usize where
  P: Fn(&usize) -> bool {
    self.iter().fold(0, |acc, i| {
      match predicate(i) {
        true => acc + 1,
        false => acc
      }
    })
  }
}

impl Countable<usize> for &Vec<usize> {
  fn count_where<P>(&self, predicate: P) -> usize where
  P: Fn(&usize) -> bool {
    self.iter().fold(0, |acc, i| {
      match predicate(i) {
        true => acc + 1,
        false => acc
      }
    })
  }
}

fn generate_partials(target: usize) -> Vec<Solution>{
  let mut ret = Vec::new();

  ret.push(Solution {
    target: 0,
    possible: true,
    necessary_coins: Some(vec![]),
  });

  for i in 1..=target {
    ret.push(Solution {
      target: i,
      possible: false,
      necessary_coins: None 
    })
  }

  ret
}

fn available_coin<C1, C2>(coin: usize, from_coins: C1, without_coins: C2) -> bool where
  C1: Countable<usize>,
  C2: Countable<usize> {
  from_coins.count_where(|c| *c == coin) -
  without_coins.count_where(|c| *c == coin)
  > 0
}

fn all_solutions(target: usize, coins: &Vec<usize>) -> Vec<Solution> {
  let mut partials = generate_partials(target);

  let mut coins = coins.clone();
  coins.sort();
  coins.reverse();

  for target in 1..=target {
    let mut partial = partials[target].clone();

    for coin in coins.iter() {
      if target < *coin { continue }

      let next_inter = partials[target - coin].clone();
      if !next_inter.possible { continue }

      let next_coins = next_inter.necessary_coins.unwrap();

      if available_coin(*coin, &coins, &next_coins) {
        let mut new_coins = next_coins;
        new_coins.push(*coin);

        partial.possible = true;
        partial.necessary_coins = Some(new_coins);
        break;
      }
    }

    partials[target] = partial.clone();
  }

  // this is not necessary for the program to function, it just looks nicer
  for i in 0..=target {
    let mut partial = partials[i].clone();

    partial.necessary_coins = partial.necessary_coins.and_then(|cs| {
      let mut cs = cs;
      cs.sort();
      cs.reverse();
      Some(cs)
    });

    partials[i] = partial;
  }

  partials
}

#[allow(dead_code)]
fn find_solution(target: usize, coins: &Vec<usize>) -> Solution {
  all_solutions(target, coins).last().unwrap().clone()
}

fn print_solution(solution: &Solution) {
  print!("{}: ", solution.target);
  match &solution.necessary_coins {
    None => println!("impossible"),
    Some(coins) => match coins.len() {
      0 => println!("no change needed"),
      _ => println!("{}", coins.iter().map(|s| format!("{s}")).collect::<Vec<String>>().join(" "))
    }
  }
}


fn main() {
  let coins: Vec<usize> = env::args()
    .map(|s| s.parse())
    .filter(|u| u.is_ok())
    .map(|u| u.unwrap())
    .collect();

  let target: usize = coins.iter().fold(0, |acc, i| acc + i);

  let _ = all_solutions(target, &coins).iter().map(print_solution);
}
