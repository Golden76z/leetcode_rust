use std::i32::MAX;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_price = 0;
        let mut min_price = MAX;

        for price in prices {
            if price < min_price {
                min_price = price;
            }

            let profit = price - min_price;

            if profit > max_price {
                max_price = profit;
            }
        }

        max_price
    }
}

fn main() {
    let tests = vec![
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
        (vec![2, 4, 1], 2),
        (vec![4, 1, 5, 2, 7], 6),
        (vec![1, 2, 3, 4, 5], 4),
    ];

    for (prices, expected) in tests {
        let result = Solution::max_profit(prices.clone());
        println!(
            "prices = {:?} → result = {}, expected = {}",
            prices, result, expected
        );
    }
}
