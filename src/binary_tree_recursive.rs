use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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

/// https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/description/
/// 1. 树的高度(二叉树的深度为根节点到最远叶子节点的最长路径上的节点数)
/// 说明: 叶子节点是指没有子节点的节点。
/// 104. Maximum Depth of Binary Tree (Easy)
#[allow(dead_code)]
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  use std::cmp::max;
  match root {
    Some(node) => {
      let l = node.borrow().left.clone();
      let r = node.borrow().right.clone();
      return max(max_depth(l), max_depth(r)) + 1;
    }
    None => return 0,
  }
}

/// https://leetcode-cn.com/problems/balanced-binary-tree/description/
/// 2. 平衡树(一个二叉树每个节点的左右两个子树的高度差的绝对值不超过1)
/// 110. Balanced Binary Tree (Easy)
fn balance(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
  match root {
    Some(node) => {
      let (l, ln) = balance(&node.borrow().left.clone());
      let (r, rn) = balance(&node.borrow().right.clone());
      // println!("node: {}, l = {}:{}, r = {}:{}", &node.borrow().val, l, ln, r, rn);
      if l && r && (ln - rn).abs() <= 1 {
        return (true, 1 + ln.max(rn));
      } else {
        return (false, 1 + ln.max(rn));
      }
    }
    None => (true, 0),
  }
}
#[allow(dead_code)]
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  let (flag, _) = balance(&root);
  // println!("{}", n);
  return flag;
}

/// https://leetcode-cn.com/problems/diameter-of-binary-tree/description/
/// 3. 两节点的最长路径
/// 543. Diameter of Binary Tree (Easy)
pub fn diameter(max: &mut i32, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
  match root {
    Some(node) => {
      let l = diameter(max, &node.borrow().left.clone());
      let r = diameter(max, &node.borrow().right.clone());
      // 將臨時最大路徑值和當前路徑值進行比較
      *max = (*max).max(l + r);
      return l.max(r) + 1;
    }
    None => 0,
  }
}
#[allow(dead_code)]
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  let mut max = 0i32;
  diameter(&mut max, &root);
  // println!("{}", max);
  return max;
}

/// https://leetcode-cn.com/problems/invert-binary-tree/description/
/// 4. 翻转树
/// 226. Invert Binary Tree (Easy)
#[allow(dead_code)]
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  if let Some(node) = root {
    let l2 = node.borrow().left.clone();
    let r2 = node.borrow().right.clone();
    node.borrow_mut().left = invert_tree(r2);
    node.borrow_mut().right = invert_tree(l2);
    return Some(node);
  }
  None
}

/// https://leetcode-cn.com/problems/merge-two-binary-trees/description/
/// 5. 归并两棵树
/// 617. Merge Two Binary Trees (Easy)
#[allow(dead_code)]
pub fn merge_trees( t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
  match (t1, t2) {
    (Some(node1), Some(node2)) => {
      // 修改當前值
      let n1_v = node1.borrow().val;
      let n2_v = node2.borrow().val;
      node1.borrow_mut().val = n1_v + n2_v;

      let n1_l = node1.borrow().left.clone();
      let n2_l = node2.borrow().left.clone();

      let n1_r = node1.borrow().right.clone();
      let n2_r = node2.borrow().right.clone();
      // 修改左樹
      node1.borrow_mut().left = merge_trees(n1_l, n2_l);
      // 修改右樹
      node1.borrow_mut().right = merge_trees(n1_r, n2_r);
      return Some(node1);
    }
    (Some(node1), None) => {
      return Some(node1);
    }
    (None, Some(node2)) => {
      return Some(node2);
    }
    (None, None) => return None,
  }
}

/// https://leetcode-cn.com/problems/path-sum/description/
/// 6. 判断路径和是否等于一个数
/// Leetcdoe : 112. Path Sum (Easy)

/// https://leetcode-cn.com/problems/path-sum-iii/description/
/// 7. 统计路径和等于一个数的路径数量
/// 437. Path Sum III (Easy)

/// https://leetcode-cn.com/problems/subtree-of-another-tree/description/
/// 8. 子树
/// 572. Subtree of Another Tree (Easy)

/// https://leetcode-cn.com/problems/symmetric-tree/description/
/// 9. 树的对称
/// 101. Symmetric Tree (Easy)

/// https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/description/
/// 10. 最小路径
/// 111. Minimum Depth of Binary Tree (Easy)

/// https://leetcode-cn.com/problems/sum-of-left-leaves/description/
/// 11. 统计左叶子节点的和
/// 404. Sum of Left Leaves (Easy)

/// https://leetcode-cn.com/problems/longest-univalue-path/
/// 12. 相同节点值的最大路径长度
/// 687. Longest Univalue Path (Easy)

/// https://leetcode-cn.com/problems/house-robber-iii/description/
/// 13. 间隔遍历
/// 337. House Robber III (Medium)

/// https://leetcode-cn.com/problems/second-minimum-node-in-a-binary-tree/description/
/// 14. 找出二叉树中第二小的节点
/// 671. Second Minimum Node In a Binary Tree (Easy)

#[cfg(test)]
mod tests {
  use super::TreeNode;
  use std::cell::RefCell;
  use std::rc::Rc;

  #[test]
  pub fn test_max_depth() {
    //    3
    //   /  \
    //  9    20
    //      /  \
    //     15   7
    let n1 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(9)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(20)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(15)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(7)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n3.borrow_mut().left = Some(n4.clone());
    n3.borrow_mut().right = Some(n5.clone());

    assert_eq!(super::max_depth(Some(n1)), 3);
  }

  #[test]
  pub fn test_is_balanced() {
    //    3
    //   /  \
    //  9    20
    //      /  \
    //     15   7
    let n1 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(9)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(20)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(15)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(7)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n3.borrow_mut().left = Some(n4.clone());
    n3.borrow_mut().right = Some(n5.clone());

    assert_eq!(super::is_balanced(Some(n1)), true);
  }

  #[test]
  pub fn test_diameter_of_binary_tree() {
    //     1
    //    /  \
    //   2    3
    //  / \
    // 4   5
    let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(5)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n2.borrow_mut().left = Some(n4.clone());
    n2.borrow_mut().right = Some(n5.clone());

    assert_eq!(super::diameter_of_binary_tree(Some(n1)), 3);
  }

  #[test]
  pub fn test_invert_tree() {
    //      4
    //    /   \
    //   2     7
    //  / \   / \
    // 1   3 6   9
    let n1 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(7)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let n7 = Rc::new(RefCell::new(TreeNode::new(9)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n2.borrow_mut().left = Some(n4.clone());
    n2.borrow_mut().right = Some(n5.clone());
    n3.borrow_mut().left = Some(n6.clone());
    n3.borrow_mut().right = Some(n7.clone());

    //      4
    //    /   \
    //   7     2
    //  / \   / \
    // 9   6 3   1
    let m1 = Rc::new(RefCell::new(TreeNode::new(4)));
    let m2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let m3 = Rc::new(RefCell::new(TreeNode::new(7)));
    let m4 = Rc::new(RefCell::new(TreeNode::new(1)));
    let m5 = Rc::new(RefCell::new(TreeNode::new(3)));
    let m6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let m7 = Rc::new(RefCell::new(TreeNode::new(9)));

    m1.borrow_mut().left = Some(m3.clone());
    m1.borrow_mut().right = Some(m2.clone());
    m3.borrow_mut().left = Some(m7.clone());
    m3.borrow_mut().right = Some(m6.clone());
    m2.borrow_mut().left = Some(m5.clone());
    m2.borrow_mut().right = Some(m4.clone());

    assert_eq!(super::invert_tree(Some(n1)), Some(m1));
  }

  #[test]
  pub fn test_merge_trees() {
    //      1
    //    /   \
    //   3     2
    //  /
    // 5
    let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(5)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n2.borrow_mut().left = Some(n4.clone());

    //      2
    //    /   \
    //   1     3
    //    \      \
    //     4      7
    let m1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let m2 = Rc::new(RefCell::new(TreeNode::new(1)));
    let m3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let m4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let m5 = Rc::new(RefCell::new(TreeNode::new(7)));

    m1.borrow_mut().left = Some(m2.clone());
    m1.borrow_mut().right = Some(m3.clone());
    m2.borrow_mut().right = Some(m4.clone());
    m3.borrow_mut().right = Some(m5.clone());

    //      3
    //    /   \
    //   4     5
    //  / \     \
    // 5   4     7
    let k1 = Rc::new(RefCell::new(TreeNode::new(3)));
    let k2 = Rc::new(RefCell::new(TreeNode::new(4)));
    let k3 = Rc::new(RefCell::new(TreeNode::new(5)));
    let k4 = Rc::new(RefCell::new(TreeNode::new(5)));
    let k5 = Rc::new(RefCell::new(TreeNode::new(4)));
    let k6 = Rc::new(RefCell::new(TreeNode::new(7)));

    k1.borrow_mut().left = Some(k2.clone());
    k1.borrow_mut().right = Some(k3.clone());
    k2.borrow_mut().left = Some(k4.clone());
    k2.borrow_mut().right = Some(k5.clone());
    k3.borrow_mut().right = Some(k6.clone());

    assert_eq!(super::merge_trees(Some(n1), Some(m1)), Some(k1));
  }
}
