use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(unused)]
struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                let target_sum = target_sum - node.val;
                if node.left.is_none() && node.right.is_none() {
                    return target_sum == 0;
                }
                Self::has_path_sum(node.left.clone(), target_sum)
                    || Self::has_path_sum(node.right.clone(), target_sum)
            }
            None => false,
        }
    }
}

fn main() {}
