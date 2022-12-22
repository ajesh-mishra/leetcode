struct Solution {}

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut most_populated_year = i32::MIN;
        let mut most_population = i32::MIN;
        let count = |y: i32| logs.iter().filter(|l| y >= l[0] && y < l[1]).count() as i32;
        for year in (1950..2050).rev() {
            let population = count(year);
            if most_population <= population {
                most_population = population;
                most_populated_year = year;
            }
        }
        most_populated_year
    }
}

fn main() {
    let logs = vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]];
    println!("{}", Solution::maximum_population(logs));
}
