use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn convert(event: String) -> (i32, i32) {
        let event: Vec<&str> = event.split(":").collect();
        let end_hour: i32 = event[0].parse().unwrap();
        let end_min: i32 = event[1].parse().unwrap();
        (end_hour, end_min)
    }
    fn compare(event1_end_hour: i32, event1_end_min: i32, event2_start_hour: i32, event2_start_min: i32) -> bool {
        println!("{}:{}", event1_end_hour, event1_end_min);
        println!("{}:{}", event2_start_hour, event2_start_min);
        match event1_end_hour.cmp(&event2_start_hour) {
            Ordering::Greater => return true,
            Ordering::Less => return false,
            Ordering::Equal => {
                match event1_end_min.cmp(&event2_start_min) {
                    Ordering::Less => return false,
                    _ => return true,
                }
            }
        }
    }
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let (event1_start_hour, event1_start_min) = Self::convert(event1[0].clone());
        let (event1_end_hour, event1_end_min) = Self::convert(event1[1].clone());
        let (event2_start_hour, event2_start_min) = Self::convert(event2[0].clone());
        let (event2_end_hour, event2_end_min) = Self::convert(event2[1].clone());

        match event1_start_hour.cmp(&event2_start_hour) {
            Ordering::Less => Self::compare(event1_end_hour, event1_end_min, event2_start_hour, event2_start_min),
            _ => Self::compare(event2_end_hour, event2_end_min, event1_start_hour, event1_start_min),
        }
    }
}

fn main() {
    let event1 = vec!["01:15".to_string(), "02:00".to_string()];
    let event2 = vec!["02:00".to_string(), "03:00".to_string()];
    println!("{}", Solution::have_conflict(event1, event2));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ut_true() {
        let event1 = vec!["01:00".to_string(),"02:00".to_string()];
        let event2 = vec!["01:20".to_string(),"03:00".to_string()];
        assert!(Solution::have_conflict(event1, event2));
        
        let event1 = vec!["01:15".to_string(), "02:00".to_string()];
        let event2 = vec!["02:00".to_string(), "03:00".to_string()];
        assert!(Solution::have_conflict(event1, event2));
    }

    #[test]
    fn ut_false() {
        let event1 = vec!["10:00".to_string(),"11:00".to_string()];
        let event2 = vec!["14:00".to_string(),"15:00".to_string()];
        assert!(!Solution::have_conflict(event1, event2));
        
        let event1 = vec!["14:13".to_string(),"22:08".to_string()];
        let event2 = vec!["02:40".to_string(),"08:08".to_string()];
        assert!(!Solution::have_conflict(event1, event2));
    }
}