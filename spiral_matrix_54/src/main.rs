fn main() {
    let test_1: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let test_2: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

    let result_1: Vec<i32> = Solution::spiral_order(test_1);
    let result_2: Vec<i32> = Solution::spiral_order(test_2);

    assert_eq!(result_1, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    assert_eq!(result_2, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
}

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        let mut width: usize = matrix[0].len();
        let mut height: usize = matrix.len();

        let mut width_index: usize = 0;
        let mut height_index: usize = 0;

        while width_index < width && height_index < height {
            let mut temp_width: Vec<i32> = vec![];
            let mut temp_width_rev: Vec<i32> = vec![];

            for column in width_index..width {
                let reverse_column = (width - 1) - (column - width_index);

                temp_width.push(matrix[height_index][column]);
                if height_index < height - 1 && reverse_column < width - 1 {
                    temp_width_rev.push(matrix[height - 1][reverse_column]);
                }
            }

            let mut temp_height: Vec<i32> = vec![];
            let mut temp_height_rev: Vec<i32> = vec![];

            for row in height_index + 1..height {
                let reverse_row = height - 1 - (row - (height_index + 1));

                temp_height.push(matrix[row][width - 1]);
                if width_index < width - 1 && reverse_row < height - 1 {
                    temp_height_rev.push(matrix[reverse_row][height_index]);
                }
            }

            result.append(&mut temp_width);
            result.append(&mut temp_height);
            result.append(&mut temp_width_rev);
            result.append(&mut temp_height_rev);

            width -= 1;
            height -= 1;
            width_index += 1;
            height_index += 1;
        }

        result
    }
}
