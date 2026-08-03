use std::collections::HashSet;

fn main() {
    let test_1: Vec<i32> = vec![1, 2, 3, 1];
    let test_2: Vec<i32> = vec![1, 2, 3, 4];
    let test_3: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

    let result_1: bool = Solution::contains_duplicate(test_1);
    let result_2: bool = Solution::contains_duplicate(test_2);
    let result_3: bool = Solution::contains_duplicate(test_3);

    assert!(result_1);
    assert!(!result_2);
    assert!(result_3);
}

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums_map: HashSet<i32> = HashSet::with_capacity(nums.len());

        for i in nums {
            if !nums_map.insert(i) {
                return true;
            }
        }

        false
    }
}
