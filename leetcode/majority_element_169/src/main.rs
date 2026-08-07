use std::collections::HashMap;

fn main() {
    let test_1: Vec<i32> = vec![3, 2, 3];
    let test_2: Vec<i32> = vec![2, 2, 1, 1, 1, 2, 2];

    let result_1: i32 = Solution::majority_element(test_1);
    let result_2: i32 = Solution::majority_element(test_2);

    assert_eq!(result_1, 3);
    assert_eq!(result_2, 2);
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut max: (i32, i32) = (0, 0);

        for i in nums {
            *count.entry(i).or_insert(0) += 1;
        }

        for (key, value) in count {
            if value > max.1 {
                max = (key, value);
            }
        }

        max.0
    }
}
