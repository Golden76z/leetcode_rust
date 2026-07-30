fn main() {
    let test_1: Vec<i32> = vec![1, 2, 3, 4];
    let test_2: Vec<i32> = vec![-1, 1, 0, -3, 3];

    let result_1: Vec<i32> = Solution::product_except_self(test_1);
    let result_2: Vec<i32> = Solution::product_except_self(test_2);

    assert_eq!(result_1, vec![24, 12, 8, 6]);
    assert_eq!(result_2, vec![0, 0, 9, 0, 0]);
}

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let n = nums.len();

        let mut left: Vec<i32> = vec![1; n];
        let mut right: Vec<i32> = vec![1; n];

        let mut l_mult: i32 = 1;
        let mut r_mult: i32 = 1;

        for i in 0..n {
            let j = n - i - 1;
            // println!("{}", j);
            left[i] = l_mult;
            right[j] = r_mult;

            l_mult *= nums[i];
            r_mult *= nums[j];
        }

        // println!("Left side: {:?}", left);
        // println!("Right side: {:?}", right);

        for i in 0..n {
            result.push(left[i] * right[i]);
        }

        result
    }
}
