use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match &root {
            Some(root) => {
                let mut root = root.borrow_mut();
                let left = root.left.take();
                root.left = Self::invert_tree(root.right.take());
                root.right = Self::invert_tree(left);
            }
            None => return None,
        }
        root
    }
}

fn main() {}
