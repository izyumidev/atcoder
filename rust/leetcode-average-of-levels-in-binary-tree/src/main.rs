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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut tracker = Vec::new();
        fn process_level(node: &Rc<RefCell<TreeNode>>, level: usize, tracker: &mut Vec<Vec<i64>>) {
            if tracker.len() <= level {
                tracker.push(Vec::new());
            }
            let mut tracker_level = tracker.get_mut(level).unwrap();
            let n = &node.borrow();
            tracker_level.push(n.val as i64);
            if let Some(left) = n.left.as_ref() {
                process_level(left, level + 1, tracker);
            }
            if let Some(right) = n.right.as_ref() {
                process_level(right, level + 1, tracker);
            }
        }
        match root {
            Some(root) => {
                process_level(&root, 0, &mut tracker);
                let mut res = Vec::new();
                for level_vec in tracker {
                    let level_sum = level_vec.iter().sum::<i64>() as f64;
                    res.push(level_sum / level_vec.len() as f64);
                }
                res
            }
            None => Vec::new(),
        }
    }
}

fn main() {}
