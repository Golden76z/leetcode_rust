fn main() {
    let test_1: (String, String) = ("aA".to_string(), "aAAbbbb".to_string());
    let test_2: (String, String) = ("z".to_string(), "ZZ".to_string());

    let result_1: i32 = Solution::num_jewels_in_stones(test_1.0, test_1.1);
    let result_2: i32 = Solution::num_jewels_in_stones(test_2.0, test_2.1);

    assert_eq!(result_1, 3);
    assert_eq!(result_2, 0);
}

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut result: i32 = 0;

        stones.chars().for_each(|item| {
            if jewels.contains(item) {
                result += 1;
            }
        });

        result
    }
}
