fn main() {
    let test_1: String = "A man, a plan, a canal: Panama".to_string();
    let test_2: String = "race a car".to_string();
    let test_3: String = " ".to_string();

    let result_1: bool = Solution::is_palindrome(test_1);
    let result_2: bool = Solution::is_palindrome(test_2);
    let result_3: bool = Solution::is_palindrome(test_3);

    assert!(result_1);
    assert!(!result_2);
    assert!(result_3);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        // println!("{:?}", chars);

        let length: usize = chars.len();

        for i in 0..length / 2 {
            let j: usize = length - 1 - i;
            if chars[i] != chars[j] {
                // println!("i: {} j: {}", i, j);
                return false;
            }
        }

        true
    }
}
