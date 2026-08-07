// use std::collections::HashMap;

fn main() {
    let test_1: String = "III".to_string();
    let test_2: String = "LVIII".to_string();
    let test_3: String = "MCMXCIV".to_string();

    let result_1 = Solution::roman_to_int(test_1);
    let result_2 = Solution::roman_to_int(test_2);
    let result_3 = Solution::roman_to_int(test_3);

    assert_eq!(result_1, 3);
    assert_eq!(result_2, 58);
    assert_eq!(result_3, 1994);
}

struct Solution;

impl Solution {
    // pub fn match_number(letter: &str) -> i32 {
    //     let roman_numbers = HashMap::from([
    //         ("I", 1),
    //         ("IV", 4),
    //         ("V", 5),
    //         ("IX", 9),
    //         ("X", 10),
    //         ("XL", 40),
    //         ("L", 50),
    //         ("XC", 90),
    //         ("C", 100),
    //         ("CD", 400),
    //         ("D", 500),
    //         ("CM", 900),
    //         ("M", 1000),
    //     ]);
    //
    //     roman_numbers.get(&letter).copied().unwrap_or(0)
    // }

    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
        let str: Vec<char> = s.chars().collect();
        let mut switch: bool = true;

        for (i, &letter) in str.iter().enumerate() {
            if switch {
                match letter {
                    'M' => {
                        result += 1000;
                    }
                    'D' => {
                        result += 500;
                    }
                    'C' => {
                        if i < str.len() - 1 {
                            if str[i + 1] == 'M' {
                                result += 900;
                                switch = false;
                            } else if str[i + 1] == 'D' {
                                result += 400;
                                switch = false;
                            } else {
                                result += 100;
                            }
                        } else {
                            result += 100;
                        }
                    }
                    'L' => {
                        result += 50;
                    }
                    'X' => {
                        if i < str.len() - 1 {
                            if str[i + 1] == 'C' {
                                result += 90;
                                switch = false;
                            } else if str[i + 1] == 'L' {
                                result += 40;
                                switch = false;
                            } else {
                                result += 10;
                            }
                        } else {
                            result += 10;
                        }
                    }
                    'V' => result += 5,
                    'I' => {
                        if i < str.len() - 1 {
                            if str[i + 1] == 'X' {
                                result += 9;
                                switch = false;
                            } else if str[i + 1] == 'V' {
                                result += 4;
                                switch = false;
                            } else {
                                result += 1;
                            }
                        } else {
                            result += 1;
                        }
                    }
                    _ => {
                        return 0;
                    }
                }
            } else {
                switch = true;
            }
        }

        result
    }
}
