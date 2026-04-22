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

pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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
