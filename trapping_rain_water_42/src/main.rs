fn main() {
    let test_1: Vec<i32> = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let test_2: Vec<i32> = vec![4, 2, 0, 3, 2, 5];

    let result_1: i32 = Solution::trap(test_1);
    let result_2: i32 = Solution::trap(test_2);

    assert_eq!(result_1, 6);
    assert_eq!(result_2, 9);
}

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let length: usize = height.len();

        let mut max: (i32, i32) = (0, 0);
        let mut left: Vec<i32> = vec![0; length];
        let mut right: Vec<i32> = vec![0; length];

        for i in 0..length - 1 {
            let j: usize = length - i - 1;
            left[i] = max.0;
            right[j] = max.1;

            if height[i] > max.0 {
                max.0 = height[i];
            }
            if height[j] > max.1 {
                max.1 = height[j];
            }
        }

        // println!("{:?}", left);
        // println!("{:?}", right);

        for i in 0..length - 1 {
            // println!("result: {}", result);
            let potential_water: i32 = left[i].min(right[i]) - height[i];
            if potential_water > 0 {
                result += left[i].min(right[i]) - height[i];
            }
        }

        result
    }
}
