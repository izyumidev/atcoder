struct Solution;

impl Solution {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn main() {
    assert_eq!(Solution::add(1, 2), 3);
}
