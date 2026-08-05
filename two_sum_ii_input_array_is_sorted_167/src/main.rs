fn main() {
    let test_1: (Vec<i32>, i32) = (vec![2, 7, 11, 15], 9);
    let test_2: (Vec<i32>, i32) = (vec![2, 3, 4], 6);
    let test_3: (Vec<i32>, i32) = (vec![-1, 0], -1);

    let result_1: Vec<i32> = Solution::two_sum(test_1.0, test_1.1);
    let result_2: Vec<i32> = Solution::two_sum(test_2.0, test_2.1);
    let result_3: Vec<i32> = Solution::two_sum(test_3.0, test_3.1);

    assert_eq!(result_1, vec![1, 2]);
    assert_eq!(result_2, vec![1, 3]);
    assert_eq!(result_3, vec![1, 2]);
}

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2);

        let mut i: usize = 0;
        let mut j: usize = numbers.len() - 1;

        while i < j {
            if numbers[i] + numbers[j] < target {
                i += 1;
            } else if numbers[i] + numbers[j] > target {
                j -= 1;
            } else {
                result.push((i + 1) as i32);
                result.push((j + 1) as i32);
                break;
            }
        }

        result
    }
}
