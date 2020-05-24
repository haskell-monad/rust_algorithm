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

/// https://leetcode.com/problems/maximum-depth-of-binary-tree/description/
/// 1. 树的高度(二叉树的深度为根节点到最远叶子节点的最长路径上的节点数)
/// 说明: 叶子节点是指没有子节点的节点。
/// 104. Maximum Depth of Binary Tree (Easy)
/// 0 ms
/// 2.7 MB
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

/// https://leetcode.com/problems/balanced-binary-tree/description/
/// 2. 平衡树(一个二叉树每个节点的左右两个子树的高度差的绝对值不超过1)
/// 110. Balanced Binary Tree (Easy)
/// 4 ms
/// 2.8 MB
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

/// https://leetcode.com/problems/diameter-of-binary-tree/description/
/// 3. 两节点的最长路径
/// 543. Diameter of Binary Tree (Easy)
/// 0 ms
/// 2.7 MB
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

/// https://leetcode.com/problems/invert-binary-tree/description/
/// 4. 翻转树
/// 226. Invert Binary Tree (Easy)
/// 0 ms
/// 2 MB
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

/// https://leetcode.com/problems/merge-two-binary-trees/description/
/// 5. 归并两棵树
/// 617. Merge Two Binary Trees (Easy)
/// 4 ms
/// 2.3 MB
#[allow(dead_code)]
pub fn merge_trees(
  t1: Option<Rc<RefCell<TreeNode>>>,
  t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
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

/// https://leetcode.com/problems/path-sum/description/
/// 6. 判断路径和是否等于一个数
/// Leetcdoe : 112. Path Sum (Easy)
/// 4 ms
/// 2.5 MB
#[allow(dead_code)]
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
  if let Some(node) = root {
    let l = node.borrow().left.clone();
    let r = node.borrow().right.clone();
    let v = node.borrow().val;
    if l.is_none() && r.is_none() && v == sum {
      return true;
    }
    return has_path_sum(l, sum - v) || has_path_sum(r, sum - v);
  }
  false
}

/// https://leetcode.com/problems/path-sum-iii/description/
/// 7. 统计路径和等于一个数的路径数量
/// 437. Path Sum III (Easy)
/// 36 ms, faster than 7.14%
/// 2.3 MB, less than 100.00%
pub fn path_sum_with_root(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
  match root {
    Some(node) => {
      let mut total = 0;
      let l = node.borrow().left.clone();
      let r = node.borrow().right.clone();
      let v = node.borrow().val.clone();
      if v == sum {
        total = total + 1;
      }
      total = total + path_sum_with_root(l, sum - v) + path_sum_with_root(r, sum - v);
      return total;
    }
    None => return 0,
  }
}
#[allow(dead_code)]
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
  match root {
    Some(node) => {
      let l = node.borrow().left.clone();
      let r = node.borrow().right.clone();
      return path_sum_with_root(Some(node), sum) + path_sum(l, sum) + path_sum(r, sum);
    }
    None => return 0,
  }
}

/// https://leetcode.com/problems/subtree-of-another-tree/description/
/// 8. 子树
/// 572. Subtree of Another Tree (Easy)
/// 16 ms, faster than 7.14%
/// 2.2 MB, less than 100.00%
pub fn is_subtree_with_root(
  s: Option<Rc<RefCell<TreeNode>>>,
  t: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
  match (s, t) {
    (Some(node1), Some(node2)) => {
      let n1_l = node1.borrow().left.clone();
      let n2_l = node2.borrow().left.clone();
      let n1_r = node1.borrow().right.clone();
      let n2_r = node2.borrow().right.clone();
      return (node1.borrow().val == node2.borrow().val)
        && is_subtree_with_root(n1_l, n2_l)
        && is_subtree_with_root(n1_r, n2_r);
    }
    (Some(_), None) => return false,
    (None, Some(_)) => return false,
    (None, None) => return true,
  }
}

#[allow(dead_code)]
pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if let (Some(node1), Some(node2)) = (s, t) {
    let l = node1.borrow().left.clone();
    let r = node1.borrow().right.clone();
    return is_subtree_with_root(Some(node1), Some(node2.clone()))
      || is_subtree(l, Some(node2.clone()))
      || is_subtree(r, Some(node2.clone()));
  }
  false
}

/// https://leetcode.com/problems/symmetric-tree/description/
/// 9. 树的对称
/// 101. Symmetric Tree (Easy)

/// https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
/// 10. 最小路径
/// 111. Minimum Depth of Binary Tree (Easy)

/// https://leetcode.com/problems/sum-of-left-leaves/description/
/// 11. 统计左叶子节点的和
/// 404. Sum of Left Leaves (Easy)

/// https://leetcode.com/problems/longest-univalue-path/
/// 12. 相同节点值的最大路径长度
/// 687. Longest Univalue Path (Easy)

/// https://leetcode.com/problems/house-robber-iii/description/
/// 13. 间隔遍历
/// 337. House Robber III (Medium)

/// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/description/
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

  #[test]
  pub fn test_has_path_sum() {
    //        5
    //       / \
    //      4   8
    //     /   / \
    //    11  13  4
    //   /  \      \
    //  7    2      1
    let n1 = Rc::new(RefCell::new(TreeNode::new(5)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(8)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(11)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(13)));
    let n6 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let n8 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n9 = Rc::new(RefCell::new(TreeNode::new(1)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n2.borrow_mut().left = Some(n4.clone());
    n3.borrow_mut().left = Some(n5.clone());
    n3.borrow_mut().right = Some(n6.clone());
    n4.borrow_mut().left = Some(n7.clone());
    n4.borrow_mut().right = Some(n8.clone());
    n6.borrow_mut().right = Some(n9.clone());
    // [5,4,11,2]
    assert_eq!(super::has_path_sum(Some(n1), 22), true);
  }

  #[test]
  pub fn test_path_sum() {
    //       10
    //      /  \
    //     5   -3
    //    / \    \
    //   3   2   11
    //  / \   \
    // 3  -2   1
    let n1 = Rc::new(RefCell::new(TreeNode::new(10)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(5)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(-3)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n6 = Rc::new(RefCell::new(TreeNode::new(11)));
    let n7 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n8 = Rc::new(RefCell::new(TreeNode::new(-2)));
    let n9 = Rc::new(RefCell::new(TreeNode::new(1)));

    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n2.borrow_mut().left = Some(n4.clone());
    n2.borrow_mut().right = Some(n5.clone());
    n3.borrow_mut().right = Some(n6.clone());
    n4.borrow_mut().left = Some(n7.clone());
    n4.borrow_mut().right = Some(n8.clone());
    n5.borrow_mut().right = Some(n9.clone());
    // 存在3條路徑等於8
    // 路径不一定以 root 开头，也不一定以 leaf 结尾，但是必须连续。
    // 1.  5 -> 3
    // 2.  5 -> 2 -> 1
    // 3. -3 -> 11
    assert_eq!(super::path_sum(Some(n1), 8), 3);
  }

  #[test]
  pub fn test_is_subtree() {
    //      3           4
    //     / \         / \
    //    4   5       1   2
    //   / \
    //  1   2
    let n1 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n3 = Rc::new(RefCell::new(TreeNode::new(5)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(2)));
    n1.borrow_mut().left = Some(n2.clone());
    n1.borrow_mut().right = Some(n3.clone());
    n2.borrow_mut().left = Some(n4.clone());
    n2.borrow_mut().right = Some(n5.clone());

    let nr1 = Rc::new(RefCell::new(TreeNode::new(4)));
    let nr2 = Rc::new(RefCell::new(TreeNode::new(1)));
    let nr3 = Rc::new(RefCell::new(TreeNode::new(2)));
    nr1.borrow_mut().left = Some(nr2.clone());
    nr1.borrow_mut().right = Some(nr3.clone());
    // 應該返回true
    assert_eq!(super::is_subtree(Some(n1.clone()), Some(nr1.clone())), true);

    //      3          4
    //     / \        / \
    //    4   5      1   2
    //   / \
    //  1   2
    //     /
    //   0
    let n6 = Rc::new(RefCell::new(TreeNode::new(0)));
    n5.borrow_mut().left = Some(n6.clone());
    // 應該返回false
    assert_eq!(super::is_subtree(Some(n1), Some(nr1)), false);
  }
}
