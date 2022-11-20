use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn letter_combinations_1(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let lookup = HashMap::from([
            ('2', "abc".chars().collect::<Vec<_>>()),
            ('3', "def".chars().collect::<Vec<_>>()),
            ('4', "ghi".chars().collect::<Vec<_>>()),
            ('5', "jkl".chars().collect::<Vec<_>>()),
            ('6', "mno".chars().collect::<Vec<_>>()),
            ('7', "pqrs".chars().collect::<Vec<_>>()),
            ('8', "tuv".chars().collect::<Vec<_>>()),
            ('9', "wxyz".chars().collect::<Vec<_>>()),
        ]);

        fn permute(
            digits: &Vec<char>,
            permutation: &mut String,
            pressed: usize,
            res: &mut Vec<String>,
            lookup: &HashMap<char, Vec<char>>,
        ) {
            if permutation.len() == digits.len() {
                res.push(permutation.to_owned());
                return;
            }

            let letters = lookup.get(&digits[pressed]).unwrap();
            for ch in letters {
                permutation.push(*ch);
                println!("permutation: {permutation}");
                println!("res: {res:?}");
                permute(digits, permutation, pressed + 1, res, lookup);
                println!("after recursive relation ..\n");
                permutation.pop();
            }
        }

        let mut res = vec![];
        permute(
            &digits.chars().collect::<Vec<_>>(),
            &mut String::new(),
            0,
            &mut res,
            &lookup,
        );
        res
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: HashMap<char, Vec<char>> = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);
        let mut result = vec![];
        for c in digits.chars() {
            let new = map.get(&c).unwrap();
            if result.is_empty() {
                result.extend(new.iter().map(|x| x.to_string()));
                continue;
            }
            let temp = result.clone();
            result.clear();
            for t in temp {
                for n in new {
                    result.push(format!("{t}{n}"));
                }
            }
        }
        result
    }
}

fn main() {
    // println!("{:?}", Solution::letter_combinations("23".to_owned()));
    println!("{:?}", Solution::letter_combinations_1("23".to_owned()));
}
