struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;

        for (i, x) in grid.iter().enumerate() {
            for (j, &y) in x.iter().enumerate() {
                if y == 1 {
                    if i == 0 {
                        perimeter += 1;
                    } else {
                        if grid[i - 1][j] == 0 {
                            perimeter += 1;
                        }
                    }

                    match grid.get(i + 1) {
                        Some(ix) => {
                            if ix[j] == 0 {
                                perimeter += 1
                            }
                        }
                        None => perimeter += 1,
                    }

                    match x.get(j + 1) {
                        Some(uu) => {
                            if *uu == 0 {
                                perimeter += 1
                            }
                        }
                        None => perimeter += 1,
                    }

                    if j == 0 {
                        perimeter += 1;
                    } else {
                        if grid[i][j - 1] == 0 {
                            perimeter += 1;
                        }
                    }

                    println!("{i}, {j} -> {}", perimeter);
                }
            }
        }

        perimeter
    }
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    println!("16 - {}", Solution::island_perimeter(grid));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let grid = vec![vec![1]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }

    #[test]
    fn tc_2() {
        let grid = vec![vec![1, 0]];
        assert_eq!(Solution::island_perimeter(grid), 4);
    }
}
