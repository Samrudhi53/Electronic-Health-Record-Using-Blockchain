use std::io;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

fn main() {
    // Manually constructing a binary tree:
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    let root = TreeNode::new(1);
    let left = TreeNode::new(2);
    let right = TreeNode::new(3);
    let left_left = TreeNode::new(4);
    let left_right = TreeNode::new(5);

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    left.borrow_mut().left = Some(left_left.clone());
    left.borrow_mut().right = Some(left_right.clone());

    let depth = max_depth(Some(root));
    println!("The maximum depth of the tree is: {}", depth);
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            1 + std::cmp::max(left_depth, right_depth)
        },
        None => 0,
    }
}
