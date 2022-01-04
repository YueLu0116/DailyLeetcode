pub struct Solution;

impl Solution{
    pub fn bitwise_complement(n: i32) -> i32{
        if n == 0{
            return 1;
        }
        let mut todo = n;
        let mut ret = n;
        let mut bit = 1;

        while todo!=0{
            ret = ret ^ bit;
            bit = bit << 1;
            todo = todo >> 1;
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(Solution::bitwise_complement(5), 2);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::bitwise_complement(7), 0);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::bitwise_complement(10), 5);
    }
}
