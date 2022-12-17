struct Solution {}

// impl Solution {
//     pub fn replace_digits(s: String) -> String {
//         let alpha = [
//             'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
//             'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
//         ];
//         let mut result = String::from("");
//         let mut prev = 0;
//         for c in s.chars() {
//             if c.is_ascii_digit() {
//                 let x = prev + char::to_digit(c, 10).unwrap() as usize;
//                 result.push(alpha[x]);
//             } else {
//                 prev = (c as u8 - 97) as usize;
//                 result.push(c);
//             }
//         }
//         result
//     }
// }

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut prev_b = b'a';

        String::from_utf8(
            s.as_bytes()
                .iter()
                .map(|&cur_b| match cur_b >= b'a' {
                    true => {
                        prev_b = cur_b;
                        cur_b
                    }
                    false => prev_b + (cur_b - b'0'),
                })
                .collect(),
        )
        .unwrap()
    }
}

fn main() {
    let s = String::from("a1c1e1b");
    println!("{}", Solution::replace_digits(s));
}
