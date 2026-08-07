fn main() {
    let mut test_1: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut test_2: Vec<Vec<i32>> = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];

    Solution::rotate(&mut test_1);
    Solution::rotate(&mut test_2);

    assert_eq!(test_1, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    assert_eq!(
        test_2,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]
    );
}

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let matrix_len: usize = matrix.len();
        for i in 0..matrix_len {
            for j in i + 1..matrix_len {
                let temp: i32 = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}
