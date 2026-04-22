use crate::solutions::structs::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

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
    use crate::solutions::structs::binary_tree;

    use super::*;

    #[test]
    fn test_00226_invert_binary_tree() {
        // Input: root = [1,2,3,4,5,6,7]
        // Output: [1,3,2,7,6,5,4]
        let root = binary_tree::from_vec(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let expected = binary_tree::from_vec(vec![
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
