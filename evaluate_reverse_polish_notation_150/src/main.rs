fn main() {
    let test_1: Vec<String> = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    let test_2: Vec<String> = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];
    let test_3: Vec<String> = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];

    let result_1: i32 = Solution::eval_rpn(test_1);
    let result_2: i32 = Solution::eval_rpn(test_2);
    let result_3: i32 = Solution::eval_rpn(test_3);

    assert_eq!(result_1, 9);
    assert_eq!(result_2, 6);
    assert_eq!(result_3, 22);
}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operations: Vec<i32> = vec![];

        for operation in tokens.iter() {
            match operation.as_str() {
                "+" | "-" | "*" | "/" | "%" => {
                    let y: i32 = operations
                        .pop()
                        .expect("Error: Not enough numbers for the operation");
                    let x: i32 = operations
                        .pop()
                        .expect("Error: Not enough numbers for the operation");

                    if (operation == "/" || operation == "%") && y == 0 {
                        panic!("Error: Division by 0 impossible");
                    }
                    if operation == "+" {
                        operations.push(x + y);
                    } else if operation == "-" {
                        operations.push(x - y);
                    } else if operation == "*" {
                        operations.push(x * y);
                    } else if operation == "/" {
                        operations.push(x / y);
                    } else {
                        operations.push(x % y);
                    }
                }
                number => {
                    let nbr: i32 = number.parse::<i32>().expect("Error: Not a valid number");
                    operations.push(nbr)
                }
            }
        }

        operations.pop().expect("Error")
    }
}
