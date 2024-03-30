struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..=row_index {
            res.push(Self::combination(row_index, i))
        }
        res
    }

    fn combination(n: i32, k: i32) -> i32 {
        (Self::factorial(n as i128) / Self::factorial(k as i128) / Self::factorial((n - k) as i128))
            as i32
    }

    fn factorial(n: i128) -> i128 {
        if n <= 1 {
            return 1;
        }
        n * Self::factorial(n - 1)
    }
}

fn main() {
    assert_eq!(
        Solution::get_row(13),
        vec![1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1]
    )
}
