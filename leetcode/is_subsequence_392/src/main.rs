fn main() {
    let test_1: (String, String) = ("abc".to_string(), "ahbgdc".to_string());
    let test_2: (String, String) = ("axc".to_string(), "ahbgdc".to_string());

    let result_1 = Solution::is_subsequence(test_1.0, test_1.1);
    let result_2 = Solution::is_subsequence(test_2.0, test_2.1);

    assert!(result_1);
    assert!(result_2);
}

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let vec_s: Vec<char> = s.chars().collect();

        // Checking if the "subsequent" is shorter than the container
        if s.len() > t.len() {
            return false;
        } else if s.is_empty() {
            return true;
        } else if s.len() == t.len() {
            return s == t;
        }

        let mut index: usize = 0;
        for char_t in t.chars() {
            if char_t == vec_s[index] {
                index += 1;
            }

            if index == s.len() {
                return true;
            }
        }

        false
    }
}
