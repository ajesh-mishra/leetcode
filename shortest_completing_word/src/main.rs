use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn _license_analyser(word: &str) -> HashMap<u8, u32> {
        let offset = b'a' - b'A';
        let mut word_map: HashMap<u8, u32> = HashMap::new();

        let clean_word = word.bytes().filter_map(|b| match b {
            b'a'..=b'z' => Some(b),
            b'A'..=b'Z' => Some(b + offset),
            _ => None,
        });

        for b in clean_word {
            let entry = word_map.entry(b).or_insert(0);
            *entry += 1;
        }

        word_map
    }

    pub fn shortest_completing_word_1(license_plate: String, mut words: Vec<String>) -> String {
        let license_map = Solution::_license_analyser(&license_plate);
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        for word in words {
            let mut found = true;
            let mut word_map: HashMap<u8, u32> = HashMap::new();

            for b in word.bytes() {
                let entry = word_map.entry(b).or_insert(0);
                *entry += 1;
            }

            if word_map.len() < license_map.len() {
                continue;
            }

            for (b, c) in &license_map {
                let x = word_map.get(&b).unwrap_or(&0);
                if x < c {
                    found = false;
                    break;
                }
            }

            if found {
                return word;
            }
        }

        unreachable!()
    }

    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut counter_lic = [0; 26];
        license_plate
            .chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .for_each(|c| counter_lic[((c as u8) - b'a') as usize] += 1);

        let mut short_word = String::new();

        for word in words.into_iter() {
            let mut counter_word = [0; 26];
            word.as_bytes()
                .iter()
                .for_each(|&b| counter_word[(b - b'a') as usize] += 1);

            if counter_lic
                .iter()
                .zip(counter_word.iter())
                .all(|(l, w)| *w >= *l)
                && (short_word.is_empty() || short_word.len() > word.len())
            {
                short_word = word;
            }
        }

        short_word
    }
}

fn main() {
    let license_plate = String::from("1s3 456");
    let words = vec![
        "looks".to_string(),
        "pest".to_string(),
        "stew".to_string(),
        "show".to_string(),
    ];
    println!(
        "{}",
        Solution::shortest_completing_word_1(license_plate.clone(), words.clone())
    );
    println!(
        "{}",
        Solution::shortest_completing_word(license_plate, words)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_only_one() {
        let license_plate = String::from("1s3 PSt");
        let words = vec![
            "step".to_string(),
            "steps".to_string(),
            "stripe".to_string(),
            "stepple".to_string(),
        ];
        assert_eq!(
            Solution::shortest_completing_word(license_plate, words),
            "steps".to_string()
        );
    }

    #[test]
    fn ut_multiple() {
        let license_plate = String::from("1s3 456");
        let words = vec![
            "looks".to_string(),
            "pest".to_string(),
            "stew".to_string(),
            "show".to_string(),
        ];
        assert_eq!(
            Solution::shortest_completing_word(license_plate, words),
            "pest".to_string()
        );
    }
}
