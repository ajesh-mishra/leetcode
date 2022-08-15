use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min: usize = usize::MAX;
        let mut result = Vec::new();

        for (i, l1) in list1.iter().enumerate() {
            if let Some(j) = list2.iter().position(|x| x == l1) {
                match (i + j).cmp(&min) {
                    Ordering::Equal => result.push(l1.to_string()),
                    Ordering::Less => {
                        result = vec![l1.to_string()];
                        min = i + j;
                    },
                    _ => {}
                }
            }
        }

        result
    }
}

fn main() {
    let list1 = vec![
        "Shogun".to_string(),
        "Tapioca Express".to_string(),
        "Burger King".to_string(),
        "KFC".to_string(),
    ];
    let list2 = vec![
        "Piatti".to_string(),
        "The Grill at Torrey Pines".to_string(),
        "Hungry Hunter Steakhouse".to_string(),
        "Shogun".to_string(),
    ];
    println!("{:?}", Solution::find_restaurant(list1, list2));
}

#[cfg(test)]
mod test {
    use super::*;

    fn compare_vector(output: &Vec<String>, expected: &Vec<String>) -> bool {
        if output.len() != expected.len() {
            return false;
        }
        for o in output {
            if !expected.contains(o) {
                return false;
            }
        }
        true
    }

    #[test]
    fn ut_single() {
        let list1 = vec![
            "Shogun".to_string(),
            "Tapioca Express".to_string(),
            "Burger King".to_string(),
            "KFC".to_string(),
        ];
        let list2 = vec![
            "KFC".to_string(),
            "Shogun".to_string(),
            "Burger King".to_string(),
        ];
        assert_eq!(
            Solution::find_restaurant(list1, list2),
            vec!["Shogun".to_string()]
        );
    }

    #[test]
    fn ut_multiple() {
        let list1 = vec![
            "Shogun".to_string(),
            "Tapioca Express".to_string(),
            "Burger King".to_string(),
            "KFC".to_string(),
        ];
        let list2 = vec![
            "KFC".to_string(),
            "Burger King".to_string(),
            "Tapioca Express".to_string(),
            "Shogun".to_string(),
        ];
        assert!(compare_vector(
            &Solution::find_restaurant(list1, list2),
            &vec![
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "Shogun".to_string(),
                "KFC".to_string(),
            ]
        ));
    }
}
