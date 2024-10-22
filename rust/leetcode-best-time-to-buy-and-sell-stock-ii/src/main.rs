struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..(prices.len() - 1) {
            if prices[i] - prices[i - 1] > 0 {
                profit += prices[i] - prices[i - 1];
            }
        }
        profit
    }
}

fn main() {}
