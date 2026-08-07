fn main() {
    let test_1: String = "()".to_string();
    let test_2: String = "()[]{}".to_string();
    let test_3: String = "(]".to_string();
    let test_4: String = "([])".to_string();
    let test_5: String = "([)]".to_string();

    let result_1: bool = Solution::is_valid(test_1);
    let result_2: bool = Solution::is_valid(test_2);
    let result_3: bool = Solution::is_valid(test_3);
    let result_4: bool = Solution::is_valid(test_4);
    let result_5: bool = Solution::is_valid(test_5);

    assert!(result_1);
    assert!(result_2);
    assert!(!result_3);
    assert!(result_4);
    assert!(!result_5);
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut storage: Vec<char> = Vec::with_capacity(s.len());

        for i in s.chars() {
            match i {
                '{' | '(' | '[' => {
                    storage.push(i);
                }
                '}' => {
                    if storage.pop() != Some('{') {
                        return false;
                    }
                }
                ')' => {
                    if storage.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if storage.pop() != Some('[') {
                        return false;
                    }
                }
                _ => {}
            }
        }

        storage.is_empty()
    }
}
