use std::collections::HashSet;

struct Solution {}

impl Solution {
    // Un-optimised Solution using HashSet for Easy Understanding
    pub fn find_poisoned_duration_1(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut posioned_seconds: HashSet<i32> = HashSet::new();

        for i in time_series {
            for j in i..i + duration {
                posioned_seconds.insert(j);
            }
        }

        posioned_seconds.len() as _
    }

    // Uptimized Solution
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let previous_second = time_series[0];
        let mut delta = duration;
        let mut total_seconds = delta;
        let mut posioned_till = previous_second + delta;

        for i in time_series {
            if previous_second == i {
                println!("{i}, {posioned_till}, {total_seconds}");
                continue;
            }
            
            delta = if posioned_till > i {
                (i + duration) - posioned_till
            } else {
                duration
            };
            
            total_seconds += delta;
            posioned_till = i + duration;
            println!("{i}, {posioned_till}, {total_seconds}");
        }

        total_seconds
    }
}

fn main() {
    let time_series = vec![1, 4];
    let duration = 2;
    println!(
        "\nTotal posioned duration: {}\n",
        Solution::find_poisoned_duration_1(time_series.clone(), duration)
    );
    println!(
        "\nTotal posioned duration: {}\n",
        Solution::find_poisoned_duration(time_series, duration)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let time_series = vec![1, 4];
        let duration = 2;
        assert_eq!(Solution::find_poisoned_duration(time_series, duration), 4);
    }

    #[test]
    fn tc_2() {
        let time_series = vec![1, 2];
        let duration = 2;
        assert_eq!(Solution::find_poisoned_duration(time_series, duration), 3);
    }

    #[test]
    fn tc_3() {
        let time_series = vec![1, 2, 3, 4, 5];
        let duration = 5;
        assert_eq!(Solution::find_poisoned_duration(time_series, duration), 9);
    }
}
