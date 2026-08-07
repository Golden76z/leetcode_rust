fn main() {
    let test_1: Vec<i32> = vec![0, 1, 2, 4, 5, 7];
    let test_2: Vec<i32> = vec![0, 2, 3, 4, 6, 8, 9];

    let result_1: Vec<String> = Solution::summary_ranges(test_1);
    let result_2: Vec<String> = Solution::summary_ranges(test_2);

    assert_eq!(result_1, vec!["0->2", "4->5", "7"]);
    assert_eq!(result_2, vec!["0", "2->4", "6", "8->9"]);
}

struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        if nums.is_empty() {
            return result;
        } else if nums.len() == 1 {
            return vec![nums[0].to_string()];
        }

        let mut min: i32 = nums[0];
        let mut index: i32 = nums[0];

        for item in nums[1..].iter() {
            if &(index + 1) == item {
                index += 1;
            } else {
                if min == index {
                    result.push(index.to_string());
                } else {
                    result.push(min.to_string() + "->" + &index.to_string());
                }
                min = *item;
                index = min;
            }
        }

        if min == index {
            result.push(index.to_string());
        } else {
            result.push(min.to_string() + "->" + &index.to_string());
        }

        result
    }
}
