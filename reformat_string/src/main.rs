struct Solution {}

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut alpha = String::from("");
        let mut num = String::from("");

        for c in s.chars() {
            match c {
                'a'..='z' => alpha.push(c),
                '0'..='9' => num.push(c),
                _ => {}
            }
        }

        let alpha_len = alpha.len() as i32;
        let num_len = num.len() as i32;

        if (alpha_len - num_len).abs() > 1 {
            return String::from("");
        }

        let (mut long_iter, mut short_iter) = if alpha_len > num_len {
            (alpha.chars(), num.chars())
        } else {
            (num.chars(), alpha.chars())
        };
        
        let mut long_next = long_iter.next();
        let mut short_next = short_iter.next();
        
        let mut result = String::from("");
        while !long_next.is_none() || !short_next.is_none() {
            if let Some(x) = long_next {
                result.push(x);
            }
            if let Some(x) = short_next {
                result.push(x);
            }
            long_next = long_iter.next();
            short_next = short_iter.next();
        }
        
        result
    }
}

fn main() {
    let s = String::from("a0b1c2");
    println!("{}", Solution::reformat(s));
}
