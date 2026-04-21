use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let mut node_ref = node.borrow_mut();
            let left = node_ref.left.take();
            let right = node_ref.right.take();
            node_ref.left = right;
            node_ref.right = left;
            Self::invert_tree(node_ref.left.clone());
            Self::invert_tree(node_ref.right.clone());
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree_from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vec.len() {
            let node = queue.pop_front().unwrap();
            if let Some(val) = vec[i] {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().left.as_ref().unwrap().clone());
            }
            i += 1;
            if i < vec.len() {
                if let Some(val) = vec[i] {
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    queue.push_back(node.borrow().right.as_ref().unwrap().clone());
                }
                i += 1;
            }
        }
        Some(root)
    }

    #[test]
    fn test_00226_invert_binary_tree() {
        // Input: root = [1,2,3,4,5,6,7]
        // Output: [1,3,2,7,6,5,4]
        let root = build_tree_from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let expected = build_tree_from_vec(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(7),
            Some(6),
            Some(5),
            Some(4),
        ]);
        assert_eq!(Solution::invert_tree(root), expected);
    }
}
