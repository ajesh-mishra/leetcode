struct Solution {}

const VOLUME_THRESHOLD: i32 = 1_000_000_000;
const DIMENSION_THRESHOLD: i32 = 10_000;
const MASS_THRESHOLD: i32 = 100;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let is_heavy = || mass >= MASS_THRESHOLD;
        let is_bulky = || {
            let mut volume: i32 = 1;
            for d in [length, width, height] {
                if d >= DIMENSION_THRESHOLD {
                    return true;
                }
                match volume.checked_mul(d) {
                    Some(x) => volume = x,
                    None => return true,
                }
            }
            volume >= VOLUME_THRESHOLD
        };
        match (is_bulky(), is_heavy()) {
            (true, true) => String::from("Both"),
            (false, false) => String::from("Neither"),
            (true, _) => String::from("Bulky"),
            (_, true) => String::from("Heavy"),
        }
    }
}

fn main() {
    println!("{}", Solution::categorize_box(1000, 35, 700, 300));
}
