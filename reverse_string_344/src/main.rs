fn main() {
    let mut test_1: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    let mut test_2: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];

    Solution::reverse_string(&mut test_1);
    Solution::reverse_string(&mut test_2);

    assert_eq!(test_1, vec!['o', 'l', 'l', 'e', 'h']);
    assert_eq!(test_2, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let length: usize = s.len();
        for i in 0..length / 2 {
            s.swap(i, length - i - 1);
        }
    }
}
