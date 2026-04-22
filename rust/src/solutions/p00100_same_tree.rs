use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::structs::binary_tree::TreeNode;

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let node_p = p.borrow();
                let node_q = q.borrow();
                if node_p.val != node_q.val {
                    false
                } else {
                    Solution::is_same_tree(node_p.left.clone(), node_q.left.clone())
                        && Solution::is_same_tree(node_p.right.clone(), node_q.right.clone())
                }
            }
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::p00100_same_tree::Solution;
    use crate::solutions::structs::binary_tree;

    #[test]
    fn test_00100_same_tree() {
        let p = binary_tree::from_vec(vec![Some(1), Some(2), Some(3)]);
        let q = binary_tree::from_vec(vec![Some(1), Some(2), Some(3)]);
        assert!(Solution::is_same_tree(p, q));

        let p = binary_tree::from_vec(vec![Some(1), Some(2)]);
        let q = binary_tree::from_vec(vec![Some(1), None, Some(2)]);
        assert!(!Solution::is_same_tree(p, q));

        let p = binary_tree::from_vec(vec![Some(1), Some(2), Some(1)]);
        let q = binary_tree::from_vec(vec![Some(1), Some(1), Some(2)]);
        assert!(!Solution::is_same_tree(p, q));
    }
}
