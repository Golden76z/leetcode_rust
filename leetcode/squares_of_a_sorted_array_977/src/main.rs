fn main() {
    let test_1: Vec<i32> = vec![-4, -1, 0, 3, 10];
    let test_2: Vec<i32> = vec![-7, -3, 2, 3, 11];

    let result_1: Vec<i32> = Solution::sorted_squares(test_1);
    let result_2: Vec<i32> = Solution::sorted_squares(test_2);

    assert_eq!(result_1, vec![0, 1, 9, 16, 100]);
    assert_eq!(result_2, vec![4, 9, 9, 49, 121]);
}

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut squared: Vec<i32> = Vec::with_capacity(nums.len());
        let mut sorted_squared: Vec<i32> = Vec::with_capacity(nums.len());

        for i in nums.iter() {
            squared.push(i * i);
        }

        let mut i = 0;
        let mut j = squared.len();
        while i < j {
            let index = j - 1;
            if squared[i] > squared[index] {
                sorted_squared.push(squared[i]);
                i += 1;
            } else {
                sorted_squared.push(squared[index]);
                j -= 1;
            }
        }

        sorted_squared.reverse();
        sorted_squared
    }
}
