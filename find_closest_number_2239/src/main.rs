struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            panic!("ERROR")
        }

        let mut closest = nums[0];

        for num in 0..nums.len() {
            if nums[num].abs() < closest.abs() {
                closest = nums[num];
            } else if nums[num].abs() == closest.abs() && nums[num] > closest {
                closest = nums[num];
            }
        }

        closest
    }
}

fn main() {
    let tests = vec![
        (vec![1, -1, 2], 1),
        (vec![-4, -2, 1, 4, 8], 1),
        (vec![2, -2, -1, 1], 1),
        (vec![-5, -3, -1], -1),
        (vec![5, 3, 1], 1),
    ];

    for (nums, expected) in tests {
        let result = Solution::find_closest_number(nums.clone());
        println!("{:?} -> {} (expected {})", nums, result, expected);
    }
}
