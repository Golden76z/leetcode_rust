use std::collections::HashMap;

fn main() {
    let test_1: (String, String) = ("a".to_string(), "b".to_string());
    let test_2: (String, String) = ("aa".to_string(), "ab".to_string());
    let test_3: (String, String) = ("aa".to_string(), "aab".to_string());

    let result_1: bool = Solution::can_construct(test_1.0, test_1.1);
    let result_2: bool = Solution::can_construct(test_2.0, test_2.1);
    let result_3: bool = Solution::can_construct(test_3.0, test_3.1);

    assert!(!result_1);
    assert!(!result_2);
    assert!(result_3);
}

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut str_map: HashMap<char, usize> = HashMap::with_capacity(magazine.len());

        for character in magazine.chars() {
            *str_map.entry(character).or_insert(0) += 1;
        }

        for character in ransom_note.chars() {
            if let Some(count) = str_map.get_mut(&character) {
                if *count == 0 {
                    return false;
                }

                *count -= 1;
            } else {
                return false;
            }
        }

        true
    }
}
