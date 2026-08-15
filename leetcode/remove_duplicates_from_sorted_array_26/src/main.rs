fn main() {
    let mut test_1: Vec<i32> = vec![1, 1, 2];
    let mut test_2: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

    let result_1: i32 = Solution::remove_duplicates(&mut test_1);
    let result_2: i32 = Solution::remove_duplicates(&mut test_2);

    assert_eq!(result_1, 2);
    assert_eq!(result_2, 5);
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut previous = nums[0];

        for i in 0..nums.len() {
            if previous != nums[i] || result == 0 {
                previous = nums[i];
                nums[result as usize] = nums[i];
                result += 1;
            }
        }

        result
    }
}
