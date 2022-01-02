pub struct Solution;

// reference: module operation
// https://en.wikipedia.org/wiki/Modulo_operation#Properties_(identities)
// (a + b) mod n = [(a mod n) + (b mod n)] mod n
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut remainder = vec![0; 60];
        let mut result = 0;

        for t in time.iter(){
            if t % 60 == 0{
                result += remainder[0];
            } else{
                result += remainder[(60 - t % 60) as usize];
            }
            remainder[(t % 60) as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        let time = vec![60, 60, 60];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
    }
    #[test]
    fn example2() {
        let time = vec![30, 20, 150, 100, 40];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
    }
    #[test]
    fn example3() {
        let time = vec![60, 30, 150, 100, 40, 80, 120];
        assert_eq!(Solution::num_pairs_divisible_by60(time), 4);
    }
}
