use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map = HashMap::new();
        for (i, word) in arr.iter().enumerate() {
            let (count, _) = map.entry(word).or_insert((0, i));
            *count += 1;
        }
        println!("{map:#?}");
        let mut distinct_elements = map
            .iter()
            .filter(|(_, (c, _))| c == &1)
            .map(|(w, (_, p))| (w, p))
            .collect::<Vec<(&&String, &usize)>>();
        distinct_elements.sort_by(|a, b| (a.1).cmp(b.1));
        if let Some((w, _)) = distinct_elements.get((k - 1) as usize) {
            return format!("{}", w);
        }
        String::from("")
    }
}

fn main() {
    let arr = vec![
        "d".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
        "b".to_owned(),
        "c".to_owned(),
        "a".to_owned(),
    ];
    println!("{}", Solution::kth_distinct(arr, 2));
}
