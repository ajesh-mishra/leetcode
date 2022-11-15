struct Solution {}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current_time: Vec<&str> = current.split(":").collect();
        let current_hour: i32 = current_time[0].parse().unwrap();
        let current_min: i32 = current_time[1].parse().unwrap();

        let correct_time: Vec<&str> = correct.split(":").collect();
        let correct_hour: i32 = correct_time[0].parse().unwrap();
        let correct_min: i32 = correct_time[1].parse().unwrap();

        let mut count = 0;
        let mut minutes = match correct_hour - current_hour {
            0 => correct_min - current_min,
            x => {
                count += x - 1;
                (60 - current_min) + correct_min
            }
        };

        while minutes != 0 {
            if minutes >= 60 {
                count += minutes / 60;
                minutes = minutes % 60;
            } else if minutes >= 15 {
                count += minutes / 15;
                minutes = minutes % 15;
            } else if minutes >= 5 {
                count += minutes / 5;
                minutes = minutes % 5;
            } else {
                count += minutes;
                minutes = 0;
            }
        }

        count
    }
}

fn main() {
    let current = String::from("02:30");
    let correct = String::from("04:35");
    println!("{}", Solution::convert_time(current, correct));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let current = String::from("11:00");
        let correct = String::from("11:01");
        assert_eq!(Solution::convert_time(current, correct), 1);
    }
}
