use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize
    }
    // 1, 11, 111, 1111
    let mut first = 1u8;
    let mut second = 1u8;
    let mut third = 1u8;
    for counter in 0..n {
        match counter % 3 {
            0 => first *= 2,
            1 => second *= 2,
            2 => third *= 2,
            _ => unreachable!("unreachable"),
        }
    }
}

#[allow(dead_code)]
struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (left, right) = nums.split_at(nums.len() / 2);
        [
            Self::helper(left.to_vec(), target),
            Self::helper(right.to_vec(), target),
        ]
        .to_vec()
    }
    fn helper(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        if nums[0] > target && nums[nums.len() - 1] < target {
            return -1;
        }
        -1
    }
}
