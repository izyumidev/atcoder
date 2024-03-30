struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut two_count = 0;
        let mut five_count = 0;
        for i in 1..=n {
            let mut num = i;
            while num % 2 == 0 {
                two_count += 1;
                num /= 2;
            }
            while num % 5 == 0 {
                five_count += 1;
                num /= 5;
            }
        }
        two_count.min(five_count)
    }
}

fn main() {}
