use std::io::{self, Read};

fn main() {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();
  let mut it = input.split_whitespace();

  for case in 0..t {
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap()'
    let mut adj_lst = vec![Vec::new; n+1]; // Adjacency list
    for _ in 0..n - 1 {
            let u: usize = it.next().unwrap().parse().unwrap();
            let v: usize = it.next().unwrap().parse().unwrap();
            adj[u].push(v);
            adj[v].push(u);
        }
    print!("{}", out);
}  

  let (_parent, mut sub) = dfs1(n, &adj);

        let f_root = compute_root(n, k, &sub);

        let mut ans = vec![0i64; n + 1];
        ans[1] = f_root;
        dfs2_iterative(1, 0, n, k, &adj, &mut sub, &mut ans);

        let total = final_sum(&ans);
        out.push_str(&format!("{}\n", total));
    }
    print!("{}", out);
}

