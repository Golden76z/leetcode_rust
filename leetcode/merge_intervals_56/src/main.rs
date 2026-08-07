fn main() {
    let test_1: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let test_2: Vec<Vec<i32>> = vec![vec![1, 4], vec![4, 5]];
    let test_3: Vec<Vec<i32>> = vec![vec![4, 7], vec![1, 4]];
    let test_4: Vec<Vec<i32>> = vec![vec![1, 4], vec![5, 6]];

    let result_1: Vec<Vec<i32>> = Solution::merge(test_1);
    let result_2: Vec<Vec<i32>> = Solution::merge(test_2);
    let result_3: Vec<Vec<i32>> = Solution::merge(test_3);
    let result_4: Vec<Vec<i32>> = Solution::merge(test_4);

    assert_eq!(result_1, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    assert_eq!(result_2, vec![vec![1, 5]]);
    assert_eq!(result_3, vec![vec![1, 7]]);
    assert_eq!(result_4, vec![vec![1, 4], vec![5, 6]]);
}

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        intervals.sort_by_key(|x| x[0]);
        result.push(intervals[0].clone());

        for item in intervals.iter().skip(1) {
            let last = result.last_mut().unwrap();

            if item[0] <= last[1] {
                last[1] = last[1].max(item[1]);
            } else {
                result.push(item.clone());
            }
        }

        result
    }
}
