fn main() {
    let test_1: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    let test_2: Vec<i32> = vec![7, 6, 4, 3, 1];

    let result_1: i32 = Solution::max_profit(test_1);
    let result_2: i32 = Solution::max_profit(test_2);

    assert_eq!(result_1, 5);
    assert_eq!(result_2, 0)
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_price: i32 = prices[0];
        let mut profit: i32 = 0;

        for item in prices.iter() {
            if item < &min_price {
                min_price = *item;
            }

            if item - min_price > profit {
                profit = *item - min_price;
            }
        }

        profit
    }
}
