struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut counter = 0;
        let mut running = 0;
        for c in s.chars() {
            running += if c == 'R' { 1 } else { -1 };
            if running == 0 {
                counter += 1;
            }
        }
        counter
    }
    pub fn balanced_string_split_1(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(res, mut counter), c| {
                counter += if c == 'L' { 1 } else { -1 };
                (res + (counter == 0) as i32, counter)
            })
            .0
    }
}

fn main() {
    let s = String::from("RLRRLLRLRL");
    println!("{}", Solution::balanced_string_split(s.clone()));
    println!("{}", Solution::balanced_string_split_1(s));
}
