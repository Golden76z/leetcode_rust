fn main() {
    let test_1: Vec<String> = vec![
        "5".to_string(),
        "2".to_string(),
        "C".to_string(),
        "D".to_string(),
        "+".to_string(),
    ];
    let test_2: Vec<String> = vec![
        "5".to_string(),
        "-2".to_string(),
        "4".to_string(),
        "C".to_string(),
        "D".to_string(),
        "9".to_string(),
        "+".to_string(),
        "+".to_string(),
    ];
    let test_3: Vec<String> = vec!["1".to_string(), "C".to_string()];

    let result_1: i32 = Solution::cal_points(test_1);
    let result_2: i32 = Solution::cal_points(test_2);
    let result_3: i32 = Solution::cal_points(test_3);

    assert_eq!(result_1, 30);
    assert_eq!(result_2, 27);
    assert_eq!(result_3, 0);
}

struct Solution;

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut result: i32 = 0;
        let mut values: Vec<i32> = vec![];

        for i in operations {
            match i.as_str() {
                "+" => {
                    let addition: i32 = values[values.len() - 1] + values[values.len() - 2];
                    result += addition;
                    values.push(addition);
                }
                "C" => {
                    if let Some(minus) = values.pop() {
                        result -= minus;
                    };
                }
                "D" => {
                    let mul: i32 = values[values.len() - 1] * 2;
                    result += mul;
                    values.push(mul);
                }
                number => {
                    if let Ok(value) = number.parse::<i32>() {
                        values.push(value);
                        result += value;
                    }
                }
            }
        }

        result
    }
}
