fn main() {
    let test_1: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let test_2: Vec<i32> = vec![1, 1];

    let result_1: i32 = Solution::max_area(test_1);
    let result_2: i32 = Solution::max_area(test_2);

    assert_eq!(result_1, 49);
    assert_eq!(result_2, 1);
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let length: usize = height.len();
        let mut index: (usize, usize) = (0, length - 1);

        while index.0 < index.1 {
            let space = height[index.0].min(height[index.1]) * (index.1 - index.0) as i32;
            if space > result {
                result = space;
            }
            if height[index.0] < height[index.1] {
                index.0 += 1;
            } else {
                index.1 -= 1;
            }
        }

        result
    }
}
