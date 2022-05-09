

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

fn find_solution<'a>(target: usize, coins: Vec<usize>) -> Solution {
  let mut intermediates = generate_intermediates(target);

  let mut coins = coins;
  coins.sort();

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

        intermediate.possible = true;
        intermediate.necessary_coins = Some(new_coins);
      }
    }

    intermediates[target] = intermediate.clone();
  }

  intermediates[target].clone()
}

fn print_solution(solution: Solution) {
  if !solution.possible {
    println!("{} is not possible", solution.target);
  } else {
    print!("{} is possible with: ", solution.target);
    for coin in solution.necessary_coins.unwrap() {
      print!("{coin} ");
    }
    println!("");
  }
}


fn main() {
  let target: usize = 27;
  let coins: Vec<usize> = vec![50, 20, 10, 5, 2, 1];

  print_solution(find_solution(target, coins));
  
  let target: usize = 28;
  let coins: Vec<usize> = vec![50, 20, 10, 5, 2, 1];

  print_solution(find_solution(target, coins));
  
  let target: usize = 29;
  let coins: Vec<usize> = vec![50, 20, 10, 5, 2, 1];

  print_solution(find_solution(target, coins));
}
