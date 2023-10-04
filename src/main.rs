use proconio::*;
use std::time::Instant;

#[fastout]
fn main() {
    let start_time = Instant::now();

    println!("{}", Solution::calculate("1 + 1".to_string()));

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed_time);
}

#[allow(dead_code)]
struct Solution;
impl Solution {
    pub fn calculate(s: String) -> i32 {
        0
    }
}
