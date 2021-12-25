
pub struct Solution;


impl Solution {
    pub fn calculate(s: String) -> i32 {
        let len = s.len();
        if len == 0{
            return 0;
        }
        let mut current_num : i32 = 0;
        // in rust: vec is stack
        let mut stack = Vec::<i32>::new();
        let mut operation : char = '+';

        for id in 0..len{
            // in rust: only utf-8
            let current_char = s.as_bytes()[id] as char;
            if current_char.is_digit(10){
                current_num = current_num * 10 + (current_char as i32 - '0' as i32);
            }
            if !current_char.is_digit(10) && current_char != ' ' || id == len - 1{
                match operation{
                    '-' => stack.push(-current_num),
                    '+' => stack.push(current_num),
                    '*' => {
                        let top = stack.pop().unwrap();
                        stack.push(top * &current_num)
                    },
                    '/' =>{
                        let top = stack.pop().unwrap();
                        stack.push(top / &current_num)
                    },
                    _ => (),
                    // rust version of switch default
                    // https://stackoverflow.com/questions/49510965/how-to-not-do-anything-on-the-rest-case-when-matching-a-string/49511205
                }
                operation = current_char;
                current_num = 0;
            }
        }
        
        stack.iter().fold(0, |acc, x| acc + x)
    }
    
    pub fn calculate_better(s: String) -> i32{
        let len = s.len();
        if len == 0{
            return 0;
        }

        let mut current_num = 0;
        let mut last_num = 0;
        let mut result = 0;
        let mut operation : char = '+';

        for id in 0..len{
            let current_char = s.as_bytes()[id] as char;
            if current_char.is_digit(10){
                current_num = current_num * 10 + (current_char as i32 - '0' as i32);
            }
            if !current_char.is_digit(10) && current_char != ' ' || id == len - 1{
                if operation == '+' || operation == '-'{
                    result += last_num;
                    last_num = if operation == '+' {current_num} else {-current_num}
                }else if operation == '*'{
                    last_num = last_num * current_num;
                }else if operation == '/'{
                    last_num = last_num / current_num;
                }
                operation = current_char;
                current_num = 0;
            }
        }
        result += last_num;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(Solution::calculate_better(String::from("3+2*2")), 7);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::calculate_better(String::from(" 3/2 ")), 1);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::calculate_better(String::from(" 3+5 / 2 ")), 5);
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::calculate_better(String::from("   ")), 0);
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::calculate_better(String::from("16 + 21/6")), 19);
    }
    #[test]
    fn example6() {
        assert_eq!(Solution::calculate_better(String::from("")), 0);
    }
}
