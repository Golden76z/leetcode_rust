use std::collections::HashMap;

fn main() {
    let test_1: Vec<String> = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let test_2: Vec<String> = vec!["".to_string()];
    let test_3: Vec<String> = vec!["a".to_string()];

    let result_1: Vec<Vec<String>> = Solution::group_anagrams(test_1);
    let result_2: Vec<Vec<String>> = Solution::group_anagrams(test_2);
    let result_3: Vec<Vec<String>> = Solution::group_anagrams(test_3);

    assert_eq!(
        result_1,
        vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
    );
    assert_eq!(result_2, vec![vec!["".to_string()]]);
    assert_eq!(result_3, vec![vec!["a".to_string()]]);
}

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let mut anagram_map: HashMap<Vec<i32>, Vec<i32>> = HashMap::with_capacity(strs.len());

        for (i, item) in strs.iter().enumerate() {
            let mut indexes: Vec<i32> = vec![0; 26];
            for letter in item.bytes() {
                indexes[(letter - b'a') as usize] += 1;
            }

            anagram_map.entry(indexes).or_insert(vec![]).push(i as i32);
        }

        for (_, value) in anagram_map {
            let mut temp: Vec<String> = vec![];
            for i in value {
                temp.push(strs[i as usize].clone());
            }
            result.push(temp);
        }

        result
    }
}
