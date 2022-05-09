

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

fn generate_intermediates(target: usize) -> Vec<Solution>{
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
  let mut intermediates = generate_intermediates(target);

  let mut coins = coins.clone();
  coins.sort();
  coins.reverse();

  for target in 1..=target {
    let mut intermediate = intermediates[target].clone();

    for coin in coins.iter() {
      if target < *coin { continue }

      let next_inter = intermediates[target - coin].clone();
      if !next_inter.possible { continue }

      let next_coins = next_inter.necessary_coins.unwrap();

      if available_coin(*coin, &coins, &next_coins) {
        let mut new_coins = next_coins;
        new_coins.push(*coin);

        // this is not necessary for the program to function, it just looks nicer
        new_coins.sort();

        intermediate.possible = true;
        intermediate.necessary_coins = Some(new_coins);
        break;
      }
    }

    intermediates[target] = intermediate.clone();
  }

  intermediates
}

fn find_solution(target: usize, coins: &Vec<usize>) -> Solution {
  all_solutions(target, coins)[target].clone()
}

fn print_solution(solution: Solution) {
  print!("{}: ", solution.target);
  match solution.necessary_coins {
    None => println!("impossible"),
    Some(coins) => match coins.len() {
      0 => println!("no change needed"),
      _ => println!("{}", coins.iter().map(|s| format!("{s}")).collect::<Vec<String>>().join(" "))
    }
  }
}


fn main() {
  let coins: Vec<usize> = vec![50, 50, 20, 10, 10, 10, 5, 5, 5, 5, 5, 5, 2, 2, 2, 1, 1, 1];
  let target: usize = 100;

  for solution in all_solutions(target, &coins) {
    print_solution(solution);
  }
}
