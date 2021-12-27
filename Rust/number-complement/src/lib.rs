pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut ret = 0;
        let mut bit = 0;
        let mut div = num;

        while div != 1{
            let remain = div % 2;
            ret += if remain == 0 {1} else {0} * i32::pow(2, bit);
            div /= 2;
            bit += 1;
        }
        
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(Solution::find_complement(5), 2);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::find_complement(1), 0);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::find_complement(116), 11);
    }
}
