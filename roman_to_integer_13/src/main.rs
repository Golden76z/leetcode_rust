struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
        fn convertion(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let chars: Vec<char> = s.chars().collect();
        let mut index = 0;

        while index < chars.len() {
            let first = convertion(chars[index]);
            if index + 1 < chars.len() {
                let second = convertion(chars[index + 1]);
                if first < second {
                    result += second - first;
                    index += 2;
                    continue;
                }
            }
            result += first;
            index += 1;
        }

        result
    }
}

fn main() {
    // You can modify or add test cases here
    let tests = [("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

    for (roman, expected) in tests {
        let result = Solution::roman_to_int(roman.to_string());
        println!("{} -> {} (expected {})", roman, result, expected);
    }
}
