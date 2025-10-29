struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut prefix = String::new();
        let min_len = strs.iter().map(|s| s.len()).min().unwrap();

        for i in 0..min_len {
            let c = strs[0].chars().nth(i).unwrap();
            if strs.iter().all(|s| s.chars().nth(i).unwrap() == c) {
                prefix.push(c);
            } else {
                break;
            }
        }

        prefix
    }
}

fn main() {
    let tests = vec![
        (vec!["flower", "flow", "flight"], "fl"),
        (vec!["dog", "racecar", "car"], ""),
        (vec!["interspecies", "interstellar", "interstate"], "inters"),
        (vec!["throne", "throne"], "throne"),
        (vec!["a"], "a"),
        (vec!["", ""], ""),
        (vec!["prefix", "pre", "prevent"], "pre"),
    ];

    for (input, expected) in tests {
        let strs: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        let result = Solution::longest_common_prefix(strs);
        println!(
            "Input = {:?} → result = {:?}, expected = {:?}",
            input, result, expected
        );
    }
}
