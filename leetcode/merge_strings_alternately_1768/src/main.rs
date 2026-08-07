struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result: String = String::new();

        if word1.is_empty() && word2.is_empty() {
            return result;
        } else if word1.is_empty() {
            return word2;
        } else if word2.is_empty() {
            return word1;
        }

        let index: (usize, usize) = (word1.len(), word2.len());
        let words: (Vec<char>, Vec<char>) = (word1.chars().collect(), word2.chars().collect());

        if index.0 > index.1 {
            for (i, _) in words.1.iter().enumerate() {
                result.extend([words.0[i], words.1[i]]);
            }
            result.extend(words.0[index.1..].iter().copied());
        } else {
            for (i, _) in words.0.iter().enumerate() {
                result.extend([words.0[i], words.1[i]]);
            }

            result.extend(words.1[index.0..].iter().copied());
        }

        result
    }
}

fn main() {}
