use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();
  let mut it = input.split_whitespace();

  for case in 0..t {
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut adj_lst = vec![Vec::new; n+1];
     for _ in 0..n - 1 {
            let u: usize = it.next().unwrap().parse().unwrap();
            let v: usize = it.next().unwrap().parse().unwrap();
            adj[u].push(v);
            adj[v].push(u);
        }
