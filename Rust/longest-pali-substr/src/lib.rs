pub struct Solution;

impl Solution {
    

    pub fn longest_palindrome(s: String) -> String {
        if (s.len() as i32) < 2{
            return s;
        }
        let mut max_len : u32 = 1;
        let mut left : u32 = 0;
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        for idx in 0..n{
            dp[idx][idx] = true;
            for idy in 0..idx{
                dp[idy][idx] = (s.as_bytes()[idx] == s.as_bytes()[idy]) &&
                               ((idx - idy < 2) || dp[idy+1][idx-1]);
                if dp[idy][idx] && max_len < ((idx - idy + 1) as u32){
                    max_len = (idx - idy + 1) as u32;
                    left = idy as u32;
                }
            }
        }
        let left = left as usize;
        let max_len = max_len as usize;
        s[left..left+max_len].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let s = String::from("babad");
        assert_eq!(Solution::longest_palindrome(s), String::from("bab"));
    }
    #[test]
    fn example2() {
        let s = String::from("cbbd");
        assert_eq!(Solution::longest_palindrome(s), String::from("bb"));
    }
}
