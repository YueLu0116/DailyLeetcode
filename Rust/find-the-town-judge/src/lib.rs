use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1{
            return 1;
        }
        let mut non_candidate = HashSet::new();
        let mut candi_numtrust_map = HashMap::<i32, i32>::new();

        for pair in trust{
            non_candidate.insert(pair[0]);
            if !non_candidate.contains(&pair[1]){
                let count = candi_numtrust_map.entry(pair[1]).or_insert(0);
                *count += 1;
            }
        }

        for cn in candi_numtrust_map.iter(){
            if *cn.1 == n-1 && !non_candidate.contains(cn.0){
                return *cn.0;
            }
        }
        return -1;
    }

    pub fn find_judge_one_array(n: i32, trust: Vec<Vec<i32>>) -> i32{
        if (trust.len() as i32) < n-1{
            return -1
        }
        if n == 1{
            return 1;
        }
        let mut trust_scores = vec![0;n as usize];
        for pair in trust.iter(){
            trust_scores[(pair[0]-1) as usize] -= 1;
            trust_scores[(pair[1]-1) as usize] += 1;
        }

        for (id,scores) in trust_scores.iter().enumerate(){
            if *scores == n-1{
                return (id+1) as i32;
            }
        }
        return -1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let trust = vec!(vec![1,2]);
        assert_eq!(Solution::find_judge_one_array(2, trust), 2);
    }
    #[test]
    fn example2() {
        let trust = vec!(vec![1,3], vec![2,3]);
        assert_eq!(Solution::find_judge_one_array(3, trust), 3);
    }
    #[test]
    fn example3() {
        let trust = vec!(vec![1,3], vec![2,3], vec![3, 1]);
        assert_eq!(Solution::find_judge_one_array(3, trust), -1);
    }
    #[test]
    fn example4() {
        let trust = vec!(vec![1,3], vec![1,2], vec![2,3]);
        assert_eq!(Solution::find_judge_one_array(3, trust), 3);
    }
    #[test]
    fn example5() {
        let trust = vec!();
        assert_eq!(Solution::find_judge_one_array(1, trust), -1);
    }
}
