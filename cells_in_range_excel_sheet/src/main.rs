const COLUMN: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

struct Solution {}

impl Solution {
    fn extract(cell: &str) -> (char, u32) {
        let mut col = ' ';
        let mut row = String::from("");
        for c in cell.chars() {
            if c.is_alphabetic() {
                col = c;
            } else if c.is_ascii_digit() {
                row.push(c);
            }
        }
        (col, row.parse::<u32>().unwrap())
    }
    pub fn cells_in_range(s: String) -> Vec<String> {
        let cell: Vec<&str> = s.split(':').collect();
        let (first_col, first_row) = Self::extract(cell[0]);
        let (second_col, second_row) = Self::extract(cell[1]);
        let mut result = vec![];
        // println!("{first_col}{first_row}:{second_col}{second_row}");

        // for i in first_row..=second_row {
        //     result.push(format!("{first_col}{i}"));
        // }

        let first_pos = COLUMN.iter().position(|&x| x == first_col).unwrap();
        let second_pos = COLUMN.iter().position(|&x| x == second_col).unwrap();

        for i in first_pos..=second_pos {
            for j in first_row..=second_row {
                result.push(format!("{}{}", COLUMN[i], j));
            }
        }

        result
    }
}

fn main() {
    let s = String::from("K1:L2");
    println!("{:?}", Solution::cells_in_range(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("K1:L2");
        assert_eq!(
            Solution::cells_in_range(s),
            vec![
                "K1".to_owned(),
                "K2".to_owned(),
                "L1".to_owned(),
                "L2".to_owned()
            ]
        );

        let s = String::from("A1:F1");
        assert_eq!(
            Solution::cells_in_range(s),
            vec![
                "A1".to_owned(),
                "B1".to_owned(),
                "C1".to_owned(),
                "D1".to_owned(),
                "E1".to_owned(),
                "F1".to_owned()
            ]
        );

        let s = String::from("U7:X9");
        assert_eq!(
            Solution::cells_in_range(s),
            vec![
                "U7".to_owned(),
                "U8".to_owned(),
                "U9".to_owned(),
                "V7".to_owned(),
                "V8".to_owned(),
                "V9".to_owned(),
                "W7".to_owned(),
                "W8".to_owned(),
                "W9".to_owned(),
                "X7".to_owned(),
                "X8".to_owned(),
                "X9".to_owned()
            ]
        );
    }
}
