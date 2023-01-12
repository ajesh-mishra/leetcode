use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn dfs(
        adjs: &HashMap<i32, Vec<i32>>,
        final_result: &mut Vec<i32>,
        labels: &[u8],
        node: i32,
        prev: i32,
    ) -> [i32; 26] {
        let mut result = [0; 26];
        for &adj in adjs.get(&node).unwrap() {
            if adj == prev {
                continue;
            }
            let res = Self::dfs(adjs, final_result, labels, adj, node);
            for i in 0..26 {
                result[i] += res[i];
            }
        }
        let node = node as usize;
        let pos = (labels[node] - b'a') as usize;
        result[pos] += 1;
        final_result[node] = result[pos];
        result
    }
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut adjs: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            let (a, b) = (edge[0], edge[1]);
            adjs.entry(a).and_modify(|x| x.push(b)).or_insert(vec![b]);
            adjs.entry(b).and_modify(|x| x.push(a)).or_insert(vec![a]);
        }
        let mut final_result = vec![0; n as usize];
        Self::dfs(&adjs, &mut final_result, labels.as_bytes(), 0, -1);
        final_result
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
    let labels = String::from("abaedcd");
    println!("{:?}", Solution::count_sub_trees(7, edges, labels));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 3]];
        let labels = String::from("bbbb");
        assert_eq!(
            Solution::count_sub_trees(4, edges, labels),
            vec![4, 2, 1, 1]
        );
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]];
        let labels = String::from("aabab");
        assert_eq!(
            Solution::count_sub_trees(5, edges, labels),
            vec![3, 2, 1, 1, 1]
        );
    }
}
