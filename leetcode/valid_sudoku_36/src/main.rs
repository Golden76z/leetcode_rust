use std::collections::HashSet;

fn main() {
    let test_1: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let test_2: Vec<Vec<char>> = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let result_1 = Solution::is_valid_sudoku(test_1);
    let result_2 = Solution::is_valid_sudoku(test_2);

    assert!(result_1);
    assert!(!result_2);
}

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Checking the rows
        for i in &board {
            let mut set: HashSet<char> = HashSet::with_capacity(i.len());
            for j in i {
                if *j != '.' && !set.insert(*j) {
                    return false;
                }
            }
        }

        //Checking the columns
        for (i, item) in board.iter().enumerate() {
            let mut set: HashSet<char> = HashSet::with_capacity(item.len());
            for (j, _) in board.iter().enumerate() {
                if board[j][i] != '.' && !set.insert(board[j][i]) {
                    return false;
                }
            }
        }

        // Checking for boxes
        let start: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 3),
            (0, 6),
            (3, 0),
            (3, 3),
            (3, 6),
            (6, 0),
            (6, 3),
            (6, 6),
        ];

        for (x, y) in start {
            let mut set: HashSet<char> = HashSet::with_capacity(board.len());
            for row in x..x + 3 {
                for column in y..y + 3 {
                    let value: char = board[row][column];

                    if value != '.' && !set.insert(value) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
