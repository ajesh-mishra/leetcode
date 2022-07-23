use std::iter::zip;

struct Solution {}

impl Solution {
    pub fn is_isomorphic_1(s: String, t: String) -> bool {
        let mut s_vec: Vec<char> = Vec::new();
        let mut s_num = 1;
        
        let mut t_vec: Vec<char> = Vec::new();
        let mut t_num = 1;

        for (sc, tc) in zip(s.chars(), t.chars()) {
            if let Some(pos) = s_vec.iter().position(|&x| x == sc) {
                s_num = (s_num * 10) + pos;
            } else {
                s_vec.push(sc);
                let pos = s_vec.len() - 1;
                s_num = (s_num * 10) + pos;
            }

            if let Some(pos) = t_vec.iter().position(|&x| x == tc) {
                t_num = (t_num * 10) + pos;
            } else {
                t_vec.push(tc);
                let pos = t_vec.len() - 1;
                t_num = (t_num * 10) + pos;
            }

            if s_num != t_num {
                return false;
            }
        }

        true
    }

    pub fn is_isomorphic_2(s: String, t: String) -> bool {
        let mut s_vec: Vec<char> = Vec::new();
        let mut s_num = 1;
        
        let mut t_vec: Vec<char> = Vec::new();
        let mut t_num = 1;

        for sc in s.chars(){
            if let Some(pos) = s_vec.iter().position(|&x| x == sc) {
                s_num = (s_num * 10) + pos;
            } else {
                s_vec.push(sc);
                let pos = s_vec.len() - 1;
                s_num = (s_num * 10) + pos;
            }
        }

        for tc in t.chars() {
            if let Some(pos) = t_vec.iter().position(|&x| x == tc) {
                t_num = (t_num * 10) + pos;
            } else {
                t_vec.push(tc);
                let pos = t_vec.len() - 1;
                t_num = (t_num * 10) + pos;
            }
        }

        s_num == t_num
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (mut map_st, mut map_ts) = ([0; 256], [0; 256]);
        let s_it = s.as_bytes().iter();
        let t_it = t.as_bytes().iter();

        for (sb, tb) in s_it.zip(t_it).map(|(a, b)| (*a + 1, *b + 1)) {
            let tbc = &mut map_st[sb as usize];
            let sbc = &mut map_ts[tb as usize];
            match *tbc == 0 && *sbc == 0 {
                true => {
                    *tbc = tb;
                    *sbc = sb;
                }
                false if *tbc != tb || *sbc != sb => return false,
                _ => (),
            }
        }
        
        true
    }
}

fn main() {
    let s = String::from("egg");
    let t = String::from("add");

    println!("{}", Solution::is_isomorphic(s.clone(), t.clone()));
    println!("{}", Solution::is_isomorphic_1(s.clone(), t.clone()));
    println!("{}", Solution::is_isomorphic_2(s, t));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let s = String::from("egg");
        let t = String::from("add");
        assert!(Solution::is_isomorphic(s, t));
    }

    #[test]
    fn tc_2() {
        let s = String::from("foo");
        let t = String::from("bar");
        assert!(!Solution::is_isomorphic(s, t));
    }

    #[test]
    fn tc_3() {
        let s = String::from("paper");
        let t = String::from("title");
        assert!(Solution::is_isomorphic(s, t));
    }
}
