use std::collections::HashSet;

fn main() {
    let test_1: Vec<i32> = vec![100, 4, 200, 1, 3, 2];
    let test_2: Vec<i32> = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    let test_3: Vec<i32> = vec![1, 0, 1, 2];

    let result_1: i32 = Solution::longest_consecutive(test_1);
    let result_2: i32 = Solution::longest_consecutive(test_2);
    let result_3: i32 = Solution::longest_consecutive(test_3);

    assert_eq!(result_1, 4);
    assert_eq!(result_2, 9);
    assert_eq!(result_3, 3);
}

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut highest: i32 = 0;

        for &i in set.iter() {
            if !set.contains(&(i - 1)) {
                let mut current_number = i;
                let mut current_length = 1;

                while set.contains(&(current_number + 1)) {
                    current_number += 1;
                    current_length += 1;
                }

                highest = highest.max(current_length);
            }
        }

        highest
    }
}
