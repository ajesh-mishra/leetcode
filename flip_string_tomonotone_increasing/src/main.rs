use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut count_one = 0;
        let mut flips = 0;
        for b in s.bytes() {
            match b {
                b'1' => count_one += 1,
                b'0' => flips = min(count_one, flips + 1),
                _ => unreachable!(),
            }
        }
        flips
    }
    pub fn min_flips_mono_incr_1(s: String) -> i32 {
        fn inner(mut s: impl Iterator<Item = u8>, mut one_count: i32, mut flips: i32) -> i32 {
            if let Some(c) = s.next() {
                match c {
                    b'1' => one_count += 1,
                    b'0' => flips = min(one_count, flips + 1),
                    _ => unreachable!(),
                }
                inner(s, one_count, flips)
            } else {
                return flips;
            }
        }
        inner(s.bytes(), 0, 0)
    }
}

fn main() {
    let s = String::from("010110");
    println!("{}", Solution::min_flips_mono_incr(s));
    let s = String::from("010110");
    println!("{}", Solution::min_flips_mono_incr_1(s));
}
