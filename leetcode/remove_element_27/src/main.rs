fn main() {
    let mut test_1: (Vec<i32>, i32) = (vec![3, 2, 2, 3], 3);
    let mut test_2: (Vec<i32>, i32) = (vec![0, 1, 2, 2, 3, 0, 4, 2], 2);

    let result_1: i32 = Solution::remove_element(&mut test_1.0, test_1.1);
    let result_2: i32 = Solution::remove_element(&mut test_2.0, test_2.1);

    assert_eq!(result_1, 2);
    assert_eq!(result_2, 5);
}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut result: i32 = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[result as usize] = nums[i];
                result += 1;
            }
        }

        result
    }
}
