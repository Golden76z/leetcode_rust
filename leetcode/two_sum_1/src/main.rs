use std::collections::HashMap;

fn main() {
    let test_1: (Vec<i32>, i32) = (vec![2, 7, 11, 15], 9);
    let test_2: (Vec<i32>, i32) = (vec![3, 2, 4], 6);
    let test_3: (Vec<i32>, i32) = (vec![3, 3], 6);

    let result_1: Vec<i32> = Solution::two_sum(test_1.0, test_1.1);
    let result_2: Vec<i32> = Solution::two_sum(test_2.0, test_2.1);
    let result_3: Vec<i32> = Solution::two_sum(test_3.0, test_3.1);

    assert_eq!(result_1, vec![0, 1]);
    assert_eq!(result_2, vec![1, 2]);
    assert_eq!(result_3, vec![0, 1]);
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let mut result: Vec<i32> = Vec::with_capacity(2);

        for (i, item) in nums.iter().enumerate() {
            nums_map.insert(*item, i);
        }

        for (i, item) in nums.iter().enumerate() {
            let rest: i32 = target - item;

            if let Some(answer) = nums_map.get_mut(&rest)
                && *answer != i
            {
                result.push(i as i32);
                result.push(*answer as i32);
                break;
            }
        }

        result
    }
}
