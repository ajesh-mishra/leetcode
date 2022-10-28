pub enum Position {
    Starts,
    Ends,
}

#[derive(PartialEq)]
pub enum Check {
    Hour,
    Min,
}

struct Solution {}

impl Solution {
    fn get_combinations(n: i32, pos: Position, check: Check) -> i32 {
        let mut count = 0;
        for i in 0..=9 {
            let time = match pos {
                Position::Starts => (i * 10) + n,
                Position::Ends => (n * 10) + i,
            };
            if check == Check::Hour && time >= 0 && time < 24 {
                count += 1;
            } else if check == Check::Min && time >= 0 && time < 60 {
                count += 1;
            }
        }
        count
    }
    fn parse_time(time: &str, check: Check) -> i32 {
        match time {
            "??" if check == Check::Hour => 24,
            "??" if check == Check::Min => 60,
            _ if time.starts_with('?') => {
                let n: i32 = time.replace('?', "").parse().unwrap();
                Self::get_combinations(n, Position::Starts, check)
            }
            _ if time.ends_with('?') => {
                let n: i32 = time.replace('?', "").parse().unwrap();
                Self::get_combinations(n, Position::Ends, check)
            }
            _ => 1,
        }
    }
    pub fn count_time(time: String) -> i32 {
        let mut t: Vec<&str> = time.split(":").collect();
        let min = t.pop().unwrap();
        let hour = t.pop().unwrap();

        Self::parse_time(hour, Check::Hour) * Self::parse_time(min, Check::Min)
    }
}

fn main() {
    let time = String::from("?5:00");
    println!("{}", Solution::count_time(time));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_hour() {
        let time = String::from("?1:00");
        assert_eq!(Solution::count_time(time), 3);
        let time = String::from("1?:00");
        assert_eq!(Solution::count_time(time), 10);
    }

    #[test]
    fn ut_both() {
        // let time = String::from("0?:0?");
        // assert_eq!(Solution::count_time(time), 100);
        let time = String::from("??:??");
        assert_eq!(Solution::count_time(time), 1440);
    }
}
