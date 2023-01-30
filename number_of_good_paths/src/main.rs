use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn dfs(
        adjs: &HashMap<i32, Vec<i32>>,
        paths: &mut Vec<Vec<i32>>,
        vals: &Vec<i32>,
        node: i32,
        prev: i32,
    ) -> Vec<Vec<i32>> {
        let mut path: Vec<Vec<i32>> = vec![];
        let node_val = vals[node as usize];
        for &adj in adjs.get(&node).unwrap() {
            if adj == prev {
                continue;
            }
            let p = Self::dfs(&adjs, paths, vals, adj, node);
            for mut temp in p {
                temp.push(node_val);
                path.push(temp);
            }
        }
        for i in 0..path.len() {
            
        }
        for p in &path {
            if p.first() != p.last() {
                continue;
            }
            if (p.first().unwrap() * p.len() as i32) < p.iter().sum() {
                continue;
            }
            paths.push(p.clone());
        }
        path.push(vec![node_val]);
        path
    }
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut adjs: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            let (a, b) = (edge[0], edge[1]);
            adjs.entry(a).and_modify(|x| x.push(b)).or_insert(vec![b]);
            adjs.entry(b).and_modify(|x| x.push(a)).or_insert(vec![a]);
        }
        // println!("{:#?}", adjs);
        let mut paths: Vec<Vec<i32>> = vec![];
        Self::dfs(&adjs, &mut paths, &vals, 0, -1);
        println!("{:#?}", paths);
        (paths.len() + vals.len()) as _
    }
}

fn main() {
    let vals = vec![1, 1, 2, 2, 3];
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]];
    println!("{}", Solution::number_of_good_paths(vals, edges));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let vals = vec![1, 3, 2, 1, 3];
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 6);
    }

    #[test]
    fn ut_2() {
        let vals = vec![1, 1, 2, 2, 3];
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4]];
        assert_eq!(Solution::number_of_good_paths(vals, edges), 7);
    }
}
