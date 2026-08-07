fn main() {
    let test_1: Vec<String> = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let test_2: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    let result_1: String = Solution::longest_common_prefix(test_1);
    let result_2: String = Solution::longest_common_prefix(test_2);

    assert_eq!(result_1, "fl".to_string());
    assert_eq!(result_2, "".to_string());
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result: String = String::new();

        if strs.is_empty() {
            return "".to_string();
        }

        let mut lowest: usize = strs[0].len();

        for (i, _) in strs.iter().enumerate() {
            if strs[i].len() < lowest {
                lowest = strs[i].len();
            }
        }

        for i in 0..lowest {
            let letter_1: char = strs[0].chars().nth(i).unwrap();
            for j in 0..strs.len() {
                let letter_2: char = strs[j].chars().nth(i).unwrap();

                // println!("{}", letter_1);
                // println!("{}", letter_2);

                if letter_1 != letter_2 {
                    return result;
                }
            }
            result.push(letter_1);
        }

        result
    }
}
