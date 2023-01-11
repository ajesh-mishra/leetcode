use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    fn dfs(adjs: &HashMap<i32, Vec<i32>>, has_apple: &Vec<bool>, node: i32, prev: i32) -> i32 {
        let mut result = 0;
        for &adj in adjs.get(&node).unwrap() {
            if adj == prev {
                continue;
            }
            let res = Self::dfs(adjs, has_apple, adj, node);
            if res > 0 || has_apple[adj as usize] {
                result += res + 2;
            }
        }
        result
    }
    pub fn min_time(_n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut adjs: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            let (a, b) = (edge[0], edge[1]);
            adjs.entry(a)
                .and_modify(|nodes| nodes.push(b))
                .or_insert(vec![b]);
            adjs.entry(b)
                .and_modify(|nodes| nodes.push(a))
                .or_insert(vec![a]);
        }
        Self::dfs(&adjs, &has_apple, 0, -1)
    }

    fn inner(edges: &Vec<Vec<i32>>, edge: i32, all_paths: &mut HashSet<String>) {
        if edge == 0 {
            return;
        }
        let mut add_path = |a, b| {
            if all_paths.insert(format!("{a}-{b}")) {
                Self::inner(edges, a, all_paths);
            }
        };
        if let Some(pos) = edges.iter().position(|x| x[1] == edge) {
            add_path(edges[pos][0], edges[pos][1]);
            return;
        }
        if let Some(pos) = edges.iter().position(|x| x[0] == edge) {
            add_path(edges[pos][1], edges[pos][0]);
            return;
        }
    }
    pub fn min_time_1(_n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut all_paths: HashSet<String> = HashSet::new();
        has_apple
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .for_each(|(edge, _)| {
                Self::inner(&edges, edge as i32, &mut all_paths);
            });
        println!("{:?}", all_paths);
        (all_paths.len() * 2) as _
    }
}

fn main() {
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];
    let has_apple = vec![false, false, true, false, true, true, false];
    println!("{}", Solution::min_time(7, edges, has_apple));

    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6],
    ];
    let has_apple = vec![false, false, true, false, true, true, false];
    println!("{}", Solution::min_time_1(7, edges, has_apple));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_2() {
        let edges = vec![vec![0, 2], vec![0, 3], vec![1, 2]];
        let has_apple = vec![false, true, false, false];
        assert_eq!(Solution::min_time(4, edges, has_apple), 4);
    }

    #[test]
    fn ut_1() {
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, true, true, false];
        assert_eq!(Solution::min_time(7, edges, has_apple), 8);

        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, true, false, false, true, false];
        assert_eq!(Solution::min_time(7, edges, has_apple), 6);

        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 6],
        ];
        let has_apple = vec![false, false, false, false, false, false, false];
        assert_eq!(Solution::min_time(7, edges, has_apple), 0);
    }
}
