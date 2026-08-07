fn main() {
    let test_1: (String, String) = ("anagram".to_string(), "nagaram".to_string());
    let test_2: (String, String) = ("rat".to_string(), "car".to_string());

    let result_1: bool = Solution::is_anagram(test_1.0, test_1.1);
    let result_2: bool = Solution::is_anagram(test_2.0, test_2.1);

    assert!(result_1);
    assert!(!result_2);
}

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut letters_s = [0; 26];
        let mut letters_t = [0; 26];

        for byte in s.bytes() {
            letters_s[(byte - b'a') as usize] += 1;
        }
        for byte in t.bytes() {
            letters_t[(byte - b'a') as usize] += 1;
        }

        letters_s == letters_t
    }
}
