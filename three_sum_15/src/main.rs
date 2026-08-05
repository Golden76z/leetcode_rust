fn main() {
    let test_1: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let test_2: Vec<i32> = vec![0, 1, 1];
    let test_3: Vec<i32> = vec![0, 0, 0];

    let result_1: Vec<Vec<i32>> = Solution::three_sum(test_1);
    let result_2: Vec<Vec<i32>> = Solution::three_sum(test_2);
    let result_3: Vec<Vec<i32>> = Solution::three_sum(test_3);

    assert_eq!(result_1, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    assert!(result_2.is_empty());
    assert_eq!(result_3, vec![vec![0, 0, 0]]);
}

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut sorted: Vec<i32> = nums.clone();
        sorted.sort();

        let length: usize = sorted.len();
        let mut index: (usize, usize, usize) = (0, 1, length - 1);
        let mut previous: (usize, usize, usize) = (0, 1, length - 1);

        while index.0 < length && sorted[index.0] < 1 {
            if index.0 > 0 && sorted[index.0] == sorted[index.0 - 1] {
                index.0 += 1;
                continue;
            }

            index.1 = index.0 + 1;
            index.2 = length - 1;
            while index.1 < index.2 {
                let addition: i32 = sorted[index.0] + sorted[index.1] + sorted[index.2];
                if addition < 0 {
                    index.1 += 1;
                } else if addition > 0 {
                    index.2 -= 1;
                } else {
                    result.push(vec![sorted[index.0], sorted[index.1], sorted[index.2]]);
                    // println!("{:?}", result);

                    previous.1 = index.1;
                    previous.2 = index.2;

                    // println!("current: {:?} & previous: {:?}", index, previous);

                    while index.1 < index.2 && sorted[index.1] == sorted[previous.1] {
                        index.1 += 1;
                    }
                    while index.1 < index.2 && sorted[index.2] == sorted[previous.2] {
                        index.2 -= 1;
                    }
                }
            }
            index.0 += 1;
        }

        result
    }
}
