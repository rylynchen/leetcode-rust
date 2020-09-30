use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

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
      right: None
    }
  }
}

fn main() {

}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut list: Vec<i32> = vec![];
    let mut stack: Vec<Option<TreeNode>> = vec![];
    match root {
        Some(node) => {
            stack.push(Some(node.borrow().val));
        },
        None => {
            return list;
        }
    }
    let mut output: Vec<i32> = vec![];
    if root.is_none() {
        return output;
    }
    stack.push(root.borrow().val);
    loop {
        if stack.is_empty() {
            break;
        }
    }
    return output;
}