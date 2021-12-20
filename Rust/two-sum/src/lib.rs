use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_id_map = HashMap::new();
        let mut ret : Vec<i32>= Vec::new();

        for (id, num) in nums.iter().enumerate(){
            if !num_id_map.contains_key(&(target-num)){
                num_id_map.insert(num, id);
            }
            else{
                ret.push(num_id_map.get(&(target-num)).unwrap().clone() as i32);
                ret.push(id as i32);
                return ret;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target : i32 = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    }
    #[test]
    fn example2() {
        let nums = vec![3,2,4];
        let target : i32 = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![1,2]);
    }
    #[test]
    fn example3() {
        let nums = vec![3,3];
        let target : i32 = 6;
        assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    }

    #[test]
    fn example4() {
        let nums = vec![-1,0,3,7];
        let target : i32 = 2;
        assert_eq!(Solution::two_sum(nums, target), vec![0,2]);
    }
}
