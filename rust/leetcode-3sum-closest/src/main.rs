struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 3 {
            return nums.iter().sum();
        }

        let mut nums = nums.clone();
        nums.sort();

        let mut min_diff = i32::MAX;

        for i in 0..nums.len() {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            let diff = nums[i] - target;
            while l < r {
                let cur_diff = diff + nums[l] + nums[r];
                if cur_diff.abs() < min_diff.abs() {
                    min_diff = cur_diff
                }
                match cur_diff.signum() {
                    0 => return target,
                    1 => r -= 1,
                    -1 => l += 1,
                    _ => unreachable!(),
                }
            }
        }

        target + min_diff
    }
}

fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
}
