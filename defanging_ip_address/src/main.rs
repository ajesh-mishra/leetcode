struct Solution {}

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut result = String::from("");
        for c in address.chars() {
            match c {
                '.' => result.push_str("[.]"),
                _ => result.push(c)
            }
        }
        result
    }
}

fn main() {
    let address = String::from("1.1.1.1");
    println!("{}", Solution::defang_i_paddr(address));
}
