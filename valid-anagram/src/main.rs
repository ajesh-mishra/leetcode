fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_count = vec![0; 26];

    for c in s.chars() {
        let pos = c as u32 - 97;
        s_count[pos as usize] += 1;
    }

    for c in t.chars() {
        let pos = c as u32 - 97;
        if s_count[pos as usize] == 0 {
            return false;
        }

        s_count[pos as usize] -= 1;
    }

    true
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    println!("{}", is_anagram(s, t));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(is_anagram(s, t));
    }

    #[test]
    fn tc_2() {
        let s = String::from("cat");
        let t = String::from("rat");
        assert!(!is_anagram(s, t));
    }
}
