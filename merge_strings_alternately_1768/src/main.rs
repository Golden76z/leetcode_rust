struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let length: (usize, usize) = (word1.len(), word2.len());
        let mut index: (usize, usize) = (0, 0);

        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();

        let mut switch: bool = true;
        let mut result = String::with_capacity(length.0 + length.1);

        while index.0 < length.0 && index.1 < length.1 {
            if switch {
                result.push(chars1[index.0]);
                index.0 += 1;
            } else {
                result.push(chars2[index.1]);
                index.1 += 1;
            }
            switch = !switch;
        }

        while index.0 < length.0 {
            result.push(chars1[index.0]);
            index.0 += 1;
        }

        while index.1 < length.1 {
            result.push(chars2[index.1]);
            index.1 += 1;
        }

        result
    }
}

fn main() {
    let test_cases = vec![("abc", "pqr"), ("ab", "pqrs"), ("abcd", "pq")];

    for (w1, w2) in test_cases {
        let result = Solution::merge_alternately(w1.to_string(), w2.to_string());
        println!("word1 = {:?}, word2 = {:?} => {:?}", w1, w2, result);
    }
}
