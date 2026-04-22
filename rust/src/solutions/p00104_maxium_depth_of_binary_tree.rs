use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::structs::binary_tree::TreeNode;

struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth_helper(root, 0)
    }

    fn max_depth_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if let Some(node) = root {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            std::cmp::max(
                Solution::max_depth_helper(left, depth + 1),
                Solution::max_depth_helper(right, depth + 1),
            )
        } else {
            depth
        }
    }

    // Iterative solution using level order traversal
    pub fn max_depth_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> = vec![(root, 1)];
        let mut res = 0;

        while let Some((node, depth)) = stack.pop() {
            if let Some(n) = node {
                let n = n.borrow();
                res = res.max(depth);
                stack.push((n.left.clone(), depth + 1));
                stack.push((n.right.clone(), depth + 1));
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::structs::binary_tree;

    #[test]
    fn test_00104_max_depth() {
        // Input: root = [3,9,20,null,null,15,7]
        // Output: 3
        let root = binary_tree::from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }
}
