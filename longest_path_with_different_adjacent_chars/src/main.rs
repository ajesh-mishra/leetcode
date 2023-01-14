use std::cmp::max;

struct Solution {}

impl Solution {
    fn dfs(u: usize, graph: &Vec<Vec<usize>>, s: &[u8], res: &mut i32) -> i32 {
        let (mut l1, mut l2) = (0, 0);

        for &v in graph[u].iter() {
            let l = Self::dfs(v, graph, s, res);
            if s[u] != s[v] {
                if l > l1 {
                    l2 = l1;
                    l1 = l;
                } else if l > l2 {
                    l2 = l;
                }
            }
        }

        *res = max(*res, l1 + l2 + 1);
        l1 + 1
    }
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut res = 0;
        let mut graph = vec![vec![]; s.len()];

        for (v, &u) in parent.iter().enumerate() {
            if u != -1 {
                graph[u as usize].push(v);
            }
        }

        Self::dfs(0, &graph, s.as_bytes(), &mut res);
        res
    }
}

fn main() {
    let parent = vec![-1, 0, 0, 1, 1, 2];
    let s = String::from("abacbe");
    println!("{}", Solution::longest_path(parent, s));
}
