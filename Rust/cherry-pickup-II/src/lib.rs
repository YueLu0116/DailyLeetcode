pub struct Solution;

impl Solution {
    fn dp(row: i32, col1: i32, col2: i32, grid: &Vec<Vec<i32>>, dp_cache: &mut Vec<Vec<Vec<i32>>>) -> i32{
        use std::cmp;

        if col1 < 0 || col1 >= grid[0].len() as i32 || col2 < 0 || col2 >= grid[0].len() as i32{
            return 0;
        }
        
        if dp_cache[row as usize][col1 as usize][col2 as usize] != -1{
            return dp_cache[row as usize][col1 as usize][col2 as usize];
        }
        let mut result = 0;
        result += grid[row as usize][col1 as usize];
        if col1 != col2{
            result += grid[row as usize][col2 as usize];
        }
        if row != (grid.len()-1) as i32{
            let mut max = 0;
            for new_col1 in col1-1..col1+2{
                for new_col2 in col2-1..col2+2{
                    max = cmp::max(max, Solution::dp(row+1, new_col1, new_col2, &grid, dp_cache));
                }
            }
            result += max;
        }

        dp_cache[row as usize][col1 as usize][col2 as usize] = result;
        result
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {

        let m = grid.len();
        let n = grid[0].len();
        let mut dp_cache = vec![vec![vec![-1;n];n];m];

        return Solution::dp(0, 0, (n-1) as i32, &grid, &mut dp_cache);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let grid = vec![vec![3,1,1], vec![2,5,1], vec![1,5,5], vec![2,1,1]];
        assert_eq!(Solution::cherry_pickup(grid), 24);
    }

    #[test]
    fn example2() {
        let grid = vec![vec![1,0,0,0,0,0,1], vec![2,0,0,0,0,3,0],
                        vec![2,0,9,0,0,0,0], vec![0,3,0,5,4,0,0],
                        vec![1,0,2,3,0,0,6]];
        assert_eq!(Solution::cherry_pickup(grid), 28);
    }
}
