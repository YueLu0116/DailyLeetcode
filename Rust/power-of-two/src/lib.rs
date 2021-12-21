pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 1{
            return true;
        }
        if n <=0 {
            return false;
        }
        let mut in_num = n;
        while in_num / 2 > 0 {
            if in_num % 2 != 0{
                return false;
            }
            else{
                in_num = in_num / 2;
                if in_num == 1{
                    return true;
                }
            }

        }
        false
    }
    pub fn is_power_of_two_clean(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn sample1() {
        assert_eq!(Solution::is_power_of_two_clean(4), true);
    }
    #[test]
    fn sample2() {
        assert_eq!(Solution::is_power_of_two_clean(1), true);
    }
    #[test]
    fn sample3() {
        assert_eq!(Solution::is_power_of_two_clean(128), true);
    }
    #[test]
    fn sample4() {
        assert_eq!(Solution::is_power_of_two_clean(-256), false);
    }
    #[test]
    fn sample5() {
        assert_eq!(Solution::is_power_of_two_clean(3), false);
    }
    #[test]
    fn sample6() {
        assert_eq!(Solution::is_power_of_two_clean(0), false);
    }
}
