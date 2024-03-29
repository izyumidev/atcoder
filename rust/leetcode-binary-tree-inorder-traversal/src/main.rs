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

struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traversed: Vec<i32> = Vec::new();

        fn traverse(node: Option<Rc<RefCell<TreeNode>>>, traversed: &mut Vec<i32>) {
            if let Some(node) = node {
                traverse(node.borrow().left.clone(), traversed);
                traversed.push(node.borrow().val);
                traverse(node.borrow().right.clone(), traversed);
            }
        }

        traverse(root, &mut traversed);
        traversed
    }
}

fn main() {}
