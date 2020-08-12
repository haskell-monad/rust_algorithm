/*
 * @lc app=leetcode id=637 lang=rust
 *
 * [637] Average of Levels in Binary Tree
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
    // 层次遍历: 一棵树每层节点的平均数
    // 0 ms, faster than 100.00% 
    // 2.8 MB, less than 100.00%
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut rs: Vec<f64> = vec![];
        if let Some(node) = root {
            let mut queue = vec![];
            queue.push(node);
            while !queue.is_empty() {
                let size = queue.len();
                let mut sum = 0 as f64;
                for (_, _) in (0..size).enumerate() {
                    // 从尾部取值
                    if let Some(node) = queue.pop() {
                        let root = node.borrow();
                        // 设置f64避免溢出 
                        sum += root.val as f64;
                        println!("val: {}, sum: {}", root.val, sum);
                        if let Some(left) = &root.left {
                            // 将新值保存到头部
                            queue.insert(0,left.clone())
                        }
                        if let Some(right) = &root.right {
                            queue.insert(0,right.clone())
                        }
                    }
                }
                let v = (sum as f64) / (size as f64);
                println!("sum: {}, size: {}, val: {}", sum, size, v);
                rs.push(v);
            }
        }
        return rs;
    }
}
// @lc code=end
