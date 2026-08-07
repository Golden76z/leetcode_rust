struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut result: i32 = nums[0];

        for (i, _) in nums.iter().enumerate() {
            if nums[i].abs() <= result.abs() {
                if nums[i].abs() == result.abs() && nums[i] < result {
                } else {
                    result = nums[i];
                }
            }
        }

        result
    }
}

fn main() {
    let nums = vec![-4, -2, 1, 4, 8];

    let result = Solution::find_closest_number(nums);

    assert_eq!(result, 1);
}
