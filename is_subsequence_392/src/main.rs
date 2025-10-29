struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let chars1: Vec<char> = s.chars().collect();
        let chars2: Vec<char> = t.chars().collect();

        if (s.len() > t.len()) || (s.len() == t.len() && s != t) {
            return false;
        }

        let mut is_found = false;
        let mut index = 0;

        for (_, char1) in chars1.iter().enumerate() {
            while index < t.len() {
                if char1 == &chars2[index] {
                    is_found = true;
                    index += 1;
                    break;
                } else {
                    index += 1;
                }
            }
            if !is_found {
                return false;
            }
            is_found = false;
        }

        true
    }
}

fn main() {
    let tests = vec![
        ("abc", "ahbgdc", true),
        ("axc", "ahbgdc", false),
        ("", "", true),
        ("acb", "ahbgdc", false),
        (
            "leeeeetcode",
            "yyyyylyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyeyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
            false,
        ),
    ];

    for (s, t, expected) in tests {
        let result = Solution::is_subsequence(s.to_string(), t.to_string());
        println!(
            "s = {:?}, t = [truncated], result = {}, expected = {}",
            s, result, expected
        );
    }
}
