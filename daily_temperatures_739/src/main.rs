fn main() {
    let test_1: Vec<i32> = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let test_2: Vec<i32> = vec![30, 40, 50, 60];
    let test_3: Vec<i32> = vec![30, 60, 90];

    let result_1: Vec<i32> = Solution::daily_temperatures(test_1);
    let result_2: Vec<i32> = Solution::daily_temperatures(test_2);
    let result_3: Vec<i32> = Solution::daily_temperatures(test_3);

    assert_eq!(result_1, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    assert_eq!(result_2, vec![1, 1, 1, 0]);
    assert_eq!(result_3, vec![1, 1, 0]);
}

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..temperatures.len() {
            while let Some(&previous_index) = stack.last() {
                if temperatures[previous_index] >= temperatures[i] {
                    break;
                }
                stack.pop();
                result[previous_index] = (i - previous_index) as i32;
            }
            stack.push(i);
        }

        result
    }
}
