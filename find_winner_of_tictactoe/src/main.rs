struct Solution {}

impl Solution {
    fn check_sum(sum: i32) -> Option<String> {
        match sum {
            3 => return Some(String::from("A")),
            15 => return Some(String::from("B")),
            _ => None
        }
    }
    pub fn check_straight_line(moves: &Vec<Vec<i32>>) -> Option<String> {
        let mut state = [[0; 3]; 3];
        for (n, mv) in moves.iter().enumerate() {
            let (i, j) = (mv[0], mv[1]);
            match n % 2 {
                0 => state[i as usize][j as usize] = 1,
                _ => state[i as usize][j as usize] = 5,
            }
        }
        // Check Horizontal
        for row in state {
            let sum = row.iter().sum::<i32>();
            if let Some(x) = Self::check_sum(sum) {
                return Some(x);
            }
        }
        // Check Vertical
        for i in 0..3 {
            let mut sum = 0;
            for j in 0..3 {
                sum += state[j][i];
            }
            if let Some(x) = Self::check_sum(sum) {
                return Some(x);
            }
        }
        // Check cross
        let mut sum = 0;
        for (i, j) in [(0, 0), (1, 1), (2, 2)] {
            sum += state[i][j];
        }
        if let Some(x) = Self::check_sum(sum) {
            return Some(x);
        }
        // Check cross
        let mut sum = 0;
        for (i, j) in [(0, 2), (1, 1), (2, 0)] {
            sum += state[i][j];
        }
        if let Some(x) = Self::check_sum(sum) {
            return Some(x);
        }
        None
    }
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let moves_len = moves.len();
        if moves_len < 5 {
            return String::from("Pending");
        }

        if let Some(x) = Self::check_straight_line(&moves) {
            return x;
        }

        match moves_len {
            9 => return String::from("Draw"),
            _ => return String::from("Pending"),
        }
    }
}

fn main() {
    let moves = vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]];
    println!("{}", Solution::tictactoe(moves));

    let moves = vec![
        vec![1, 2],
        vec![2, 1],
        vec![1, 0],
        vec![0, 0],
        vec![0, 1],
        vec![2, 0],
        vec![1, 1],
    ];
    Solution::tictactoe(moves);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_pending() {
        let moves = vec![vec![1, 0], vec![2, 2], vec![2, 0], vec![0, 1], vec![1, 1]];
        assert_eq!(Solution::tictactoe(moves), String::from("Pending"));

        let moves = vec![vec![0, 0], vec![1, 2], vec![0, 2], vec![1, 1], vec![2, 1]];
        assert_eq!(Solution::tictactoe(moves), String::from("Pending"));
    }

    #[test]
    fn ut_a_wins() {
        let moves = vec![
            vec![1, 2],
            vec![2, 1],
            vec![1, 0],
            vec![0, 0],
            vec![0, 1],
            vec![2, 0],
            vec![1, 1],
        ];
        assert_eq!(Solution::tictactoe(moves), String::from("A"))
    }

    #[test]
    fn ut_b_wins() {
        let moves = vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0],
        ];
        assert_eq!(Solution::tictactoe(moves), String::from("B"))
    }

    #[test]
    fn ut_draw() {
        let moves = vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2],
        ];
        assert_eq!(Solution::tictactoe(moves), String::from("Draw"))
    }
}
