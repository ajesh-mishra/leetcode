struct Solution {}

impl Solution {
    fn check_year(year: i32) -> bool {
        if year % 400 == 0 {
            return true;
        } else if year % 100 == 0 {
            return false;
        } else if year % 4 == 0 {
            return true;
        }
        false
    }
    pub fn day_of_year(date: String) -> i32 {
        let date = date.split("-");
        let (mut year, mut month, mut day) = (0, 0, 0);
        for (i, d) in date.enumerate() {
            match i {
                0 => year = d.parse().unwrap(),
                1 => month = d.parse::<usize>().unwrap() - 1,
                2 => day = d.parse().unwrap(),
                _ => {}
            }
        }

        let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if Self::check_year(year) {
            days_in_month[1] = 29;
        }
        
        let mut days = 0;
        for i in 0..month {
            days += days_in_month[i];
        }

        days + day
    }
}

fn main() {
    let date = String::from("2019-01-09");
    println!("{}", Solution::day_of_year(date));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let date = String::from("2019-02-10");
        assert_eq!(Solution::day_of_year(date), 41);
    }
}