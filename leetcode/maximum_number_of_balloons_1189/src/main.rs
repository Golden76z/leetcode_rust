use std::collections::HashMap;

fn main() {
    let test_1: String = "nlaebolko".to_string();
    let test_2: String = "loonbalxballpoon".to_string();
    let test_3: String = "leetcode".to_string();

    let result_1: i32 = Solution::max_number_of_balloons(test_1);
    let result_2: i32 = Solution::max_number_of_balloons(test_2);
    let result_3: i32 = Solution::max_number_of_balloons(test_3);

    assert_eq!(result_1, 1);
    assert_eq!(result_2, 2);
    assert_eq!(result_3, 0);
}

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut result: i32 = 0;
        let mut map: HashMap<char, usize> =
            HashMap::from([('b', 0), ('a', 0), ('l', 0), ('o', 0), ('n', 0)]);

        for i in text.chars() {
            match i {
                'b' | 'a' | 'l' | 'o' | 'n' => {
                    *map.entry(i).or_insert(0) += 1;
                }
                _ => {}
            }
        }

        let mut switch: bool = true;

        while switch {
            if let Some(count) = map.get_mut(&'b') {
                if *count > 0 {
                    *count -= 1;
                } else {
                    switch = false;
                }
            }
            if let Some(count) = map.get_mut(&'a') {
                if *count > 0 {
                    *count -= 1;
                } else {
                    switch = false;
                }
            }
            if let Some(count) = map.get_mut(&'l') {
                if *count > 1 {
                    *count -= 2;
                } else {
                    switch = false;
                }
            }
            if let Some(count) = map.get_mut(&'o') {
                if *count > 1 {
                    *count -= 2;
                } else {
                    switch = false;
                }
            }
            if let Some(count) = map.get_mut(&'n') {
                if *count > 0 {
                    *count -= 1;
                } else {
                    switch = false;
                }
            }

            if switch {
                result += 1;
            }
        }

        result
    }
}
