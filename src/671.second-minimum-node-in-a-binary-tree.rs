/*
 * @lc app=leetcode id=671 lang=rust
 *
 * [671] Second Minimum Node In a Binary Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_second_minimum_value_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp::min;
        if let Some(node) = root {
            let node = node.borrow();
            let v = node.val;
            let mut lv = -1;
            let mut rv = -1;
            if let Some(left) = &node.left {
                lv = left.borrow().val;
            }
            if let Some(right) = &node.right {
                rv = right.borrow().val;
            }
            if lv == -1 && rv == -1 {
                return -1;
            }
            if lv == v {
                lv = Solution::find_second_minimum_value_ref(&node.left)
            }
            if rv == v {
                rv = Solution::find_second_minimum_value_ref(&node.right)
            }
            if lv != -1 && rv != -1 {
                return min(lv, rv);
            }
            if lv != -1 {
                return lv;
            }
            if rv != -1 {
                return rv;
            }
        }
        -1
    }
    //    2
    //   / \
    //  2   5
    //     / \
    //    5   7
    // output: 5
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Solution::find_second_minimum_value_ref(&root);
    }
}
// @lc code=end
