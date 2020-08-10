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
    (None, None) => return true,
    _ => return false,
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
/// 0 ms, faster than 100.00%
/// 2 MB, less than 100.00%
pub fn is_symmetric_tree(
  node1: Option<Rc<RefCell<TreeNode>>>,
  node2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
  match (node1, node2) {
    (Some(left), Some(right)) => {
      let l = left.borrow().val;
      let r = right.borrow().val;
      if l != r {
        return false;
      }
      let l_l = left.borrow().left.clone();
      let l_r = left.borrow().right.clone();

      let r_l = right.borrow().left.clone();
      let r_r = right.borrow().right.clone();
      return is_symmetric_tree(l_l, r_r) && is_symmetric_tree(l_r, r_l);
    }
    (None, None) => return true,
    _ => return false,
  }
}

#[allow(dead_code)]
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if let Some(node) = root {
    let l = node.borrow().left.clone();
    let r = node.borrow().right.clone();
    return is_symmetric_tree(l, r);
  }
  true
}

/// https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
/// 10. 最小路径
/// 111. Minimum Depth of Binary Tree (Easy)
/// 树的根节点到叶子节点的最小路径长度
/// 0 ms, faster than 100.00%
/// 2.8 MB, less than 100.00%
#[allow(dead_code)]
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if let Some(node) = root {
    let l = min_depth(node.borrow().left.clone());
    let r = min_depth(node.borrow().right.clone());
    if l == 0 || r == 0 {
      return l + r + 1;
    }
    return l.min(r) + 1;
  }
  0
}

/// https://leetcode.com/problems/sum-of-left-leaves/description/
/// 11. 统计左叶子节点的和
/// 404. Sum of Left Leaves (Easy)
/// 0 ms, faster than 100.00%
/// 2.3 MB, less than 100.00%
pub fn is_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  if let Some(node) = root {
    let l = node.borrow().left.clone();
    let r = node.borrow().right.clone();
    // 说明: 叶子节点是指没有子节点的节点。
    return l.is_none() && r.is_none();
  }
  false
}
#[allow(dead_code)]
pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if let Some(node) = root {
    let l = node.borrow().left.clone();
    let r = node.borrow().right.clone();
    if is_leaves(l.clone()) {
      return l.unwrap().borrow().val + sum_of_left_leaves(r);
    }
    return sum_of_left_leaves(l) + sum_of_left_leaves(r);
  }
  0
}

/// https://leetcode.com/problems/longest-univalue-path/
/// 12. 相同节点值的最大路径长度
/// 687. Longest Univalue Path (Easy)
pub fn recursion_tree(path: &mut i32, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
  use std::cmp::max;
  if let Some(node) = root {
    let lv = recursion_tree(path, &node.borrow().left.clone());
    let rv = recursion_tree(path, &node.borrow().right.clone());
    let l_path = if node.borrow().left.is_some()
      && node.borrow().left.clone().unwrap().borrow().val == node.borrow().val
    {
      lv + 1
    } else {
      0
    };
    let r_path = if node.borrow().right.is_some()
      && node.borrow().right.clone().unwrap().borrow().val == node.borrow().val
    {
      rv + 1
    } else {
      0
    };
    println!(
      "node: {}, l_path: {}, r_path: {}",
      &node.borrow().val,
      l_path,
      r_path
    );
    *path = (*path).max(l_path + r_path);
    return max(l_path, r_path);
  }
  0
}
#[allow(dead_code)]
pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  let mut max = 0i32;
  recursion_tree(&mut max, &root);
  max
}

/// https://leetcode.com/problems/house-robber-iii/description/
/// 13. 间隔遍历
/// 337. House Robber III (Medium)
/// https://www.reddit.com/r/rust/comments/i02oxr/beginner_ask_for_help/
pub fn rob_old(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  use std::cmp::max;
  if let Some(node) = root {
    let mut v1 = node.borrow().val;
    let l_exist = node.borrow().left.is_some();
    let r_exist = node.borrow().right.is_some();
    if l_exist {
      let l1 = node.borrow().left.clone();
      let l2 = node.borrow().left.clone();
      v1 +=
        rob_old(l1.unwrap().borrow().left.clone()) + rob_old(l2.unwrap().borrow().right.clone());
    }
    if r_exist {
      let r1 = node.borrow().right.clone();
      let r2 = node.borrow().right.clone();
      v1 +=
        rob_old(r1.unwrap().borrow().left.clone()) + rob_old(r2.unwrap().borrow().right.clone());
    }
    let v2 = rob_old(node.borrow().left.clone()) + rob_old(node.borrow().right.clone());
    return max(v1, v2);
  }
  0
}
pub fn rob_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
  use std::cmp::max;
  if let Some(node) = root {
    let node = node.borrow();
    let mut v1 = node.val;
    if let Some(left) = &node.left {
      v1 += rob_ref(&left.borrow().left) + rob_ref(&left.borrow().right);
    }
    if let Some(right) = &node.right {
      v1 += rob_ref(&right.borrow().left) + rob_ref(&right.borrow().right);
    }
    let v2 = rob_ref(&node.left) + rob_ref(&node.right);
    return max(v1, v2);
  }
  0
}
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  return rob_ref(&root);
}

/// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/description/
/// 14. 找出二叉树中第二小的节点
/// 671. Second Minimum Node In a Binary Tree (Easy)
/// Your runtime beats 100 % of rust submissions
/// Your memory usage beats 100 % of rust submissions (2.1 MB)
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
      lv = find_second_minimum_value_ref(&node.left)
    }
    if rv == v {
      rv = find_second_minimum_value_ref(&node.right)
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
pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  return find_second_minimum_value_ref(&root);
}

#[cfg(test)]
mod tests {
  use super::TreeNode;
  use std::cell::RefCell;
  use std::rc::Rc;

  pub fn build_tree(array: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let tree: Vec<_> = array
      .iter()
      .map(|x| {
        if x.is_none() {
          None
        } else {
          Some(Rc::new(RefCell::new(TreeNode::new(x.unwrap()))))
        }
      })
      .collect();

    let mut iter = tree.iter();
    let _head = iter.next();

    let mut i = 1;
    let mut node = tree[0].as_ref();
    let mut next = 1;

    loop {
      if i == tree.len() {
        break;
      }
      println!("i: {}, next: {}", i, next);

      if node.is_none() {
        // 移動到下一個節點
        node = tree[i].as_ref();
        i = i + 1;
        continue;
      }
      if next >= tree.len() {
        println!(
          "set {} left={} i={} next={}",
          node.unwrap().borrow().val,
          "None",
          i,
          next
        );
        println!(
          "set {} right={} i={} next={}",
          node.unwrap().borrow().val,
          "None",
          i,
          next
        );
        node.unwrap().borrow_mut().left = None;
        node.unwrap().borrow_mut().right = None;
      } else {
        println!(
          "set {} left={} i={} next={}",
          node.unwrap().borrow().val,
          if tree[next].clone().is_some() {
            tree[next].clone().unwrap().borrow().val
          } else {
            -999
          },
          i,
          next
        );
        println!(
          "set {} right={} i={} next={}",
          node.unwrap().borrow().val,
          if tree[next + 1].clone().is_some() {
            tree[next + 1].clone().unwrap().borrow().val
          } else {
            -999
          },
          i,
          next
        );
        node.unwrap().borrow_mut().left = tree[next].clone();
        node.unwrap().borrow_mut().right = tree[next + 1].clone();
      }

      node = tree[i].as_ref();
      i = i + 1;
      next = next + 2;
    }
    return tree[0].clone();
  }

  #[test]
  pub fn test_max_depth() {
    //    3
    //   /  \
    //  9    20
    //      /  \
    //     15   7
    let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let t1 = build_tree(tree);
    assert_eq!(super::max_depth(t1), 3);
  }

  #[test]
  pub fn test_is_balanced() {
    //    3
    //   /  \
    //  9    20
    //      /  \
    //     15   7
    let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let t1 = build_tree(tree);
    assert_eq!(super::is_balanced(t1), true);
  }

  #[test]
  pub fn test_diameter_of_binary_tree() {
    //     1
    //    /  \
    //   2    3
    //  / \
    // 4   5
    let tree = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    let t1 = build_tree(tree);
    assert_eq!(super::diameter_of_binary_tree(t1), 3);
  }

  #[test]
  pub fn test_invert_tree() {
    //      4
    //    /   \
    //   2     7
    //  / \   / \
    // 1   3 6   9
    let t1 = vec![
      Some(4),
      Some(2),
      Some(7),
      Some(1),
      Some(3),
      Some(6),
      Some(9),
    ];
    let t11 = build_tree(t1);

    //      4
    //    /   \
    //   7     2
    //  / \   / \
    // 9   6 3   1
    let t2 = vec![
      Some(4),
      Some(7),
      Some(2),
      Some(9),
      Some(6),
      Some(3),
      Some(1),
    ];
    let t22 = build_tree(t2);

    assert_eq!(super::invert_tree(t11), t22);
  }

  #[test]
  pub fn test_merge_trees() {
    //      1
    //    /   \
    //   3     2
    //  /
    // 5
    let t1 = vec![Some(1), Some(3), Some(2), Some(5), None];
    let t11 = build_tree(t1);
    //      2
    //    /   \
    //   1     3
    //    \      \
    //     4      7
    let t2 = vec![Some(2), Some(1), Some(3), None, Some(4), None, Some(7)];
    let t22 = build_tree(t2);

    //      3
    //    /   \
    //   4     5
    //  / \     \
    // 5   4     7
    let t3 = vec![Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)];
    let t33 = build_tree(t3);

    assert_eq!(super::merge_trees(t11, t22), t33);
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
    let t1 = vec![
      Some(5),
      Some(4),
      Some(8),
      Some(11),
      None,
      Some(13),
      Some(4),
      Some(7),
      Some(2),
      None,
      None,
      None,
      Some(1),
    ];
    let t11 = build_tree(t1);
    // [5,4,11,2]
    assert_eq!(super::has_path_sum(t11.clone(), 22), true);

    //        5
    //       / \
    //      4   8
    //     /   / \
    //    11  13  4
    //   /  \      \
    //  7    2      1
    //        \      \
    //         1      6
    let t1 = vec![
      Some(5),
      Some(4),
      Some(8),
      Some(11),
      None,
      Some(13),
      Some(4),
      Some(7),
      Some(2),
      None,
      None,
      None,
      Some(1),
      None,
      None,
      None,
      Some(1),
      None,
      Some(6),
    ];
    let t11 = build_tree(t1);
    // [5,8,4,1,6]
    assert_eq!(super::has_path_sum(t11.clone(), 24), true);
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
    let t1 = vec![
      Some(10),
      Some(5),
      Some(-3),
      Some(3),
      Some(2),
      None,
      Some(11),
      Some(3),
      Some(-2),
      None,
      Some(1),
    ];
    let t11 = build_tree(t1);
    // 存在3條路徑等於8
    // 路径不一定以 root 开头，也不一定以 leaf 结尾，但是必须连续。
    // 1.  5 -> 3
    // 2.  5 -> 2 -> 1
    // 3. -3 -> 11
    assert_eq!(super::path_sum(t11, 8), 3);
  }

  #[test]
  pub fn test_is_subtree() {
    //      3           4
    //     / \         / \
    //    4   5       1   2
    //   / \
    //  1   2
    let t1 = vec![Some(3), Some(4), Some(5), Some(1), Some(2)];
    let t11 = build_tree(t1);

    let t2 = vec![Some(4), Some(1), Some(2)];
    let t22 = build_tree(t2);
    //
    assert_eq!(super::is_subtree(t11, t22.clone()), true);

    //      3          4
    //     / \        / \
    //    4   5      1   2
    //   / \
    //  1   2
    //     /
    //   0
    let t1 = vec![
      Some(3),
      Some(4),
      Some(5),
      Some(1),
      Some(2),
      None,
      None,
      None,
      None,
      Some(0),
      None,
    ];
    let t11 = build_tree(t1);
    //
    assert_eq!(super::is_subtree(t11, t22), false);
  }

  #[test]
  pub fn test_is_symmetric() {
    //       1
    //     /   \
    //    2     2
    //   / \   / \
    //  3   4 4   3
    let t1 = vec![
      Some(1),
      Some(2),
      Some(2),
      Some(3),
      Some(4),
      Some(4),
      Some(3),
    ];
    let t11 = build_tree(t1);
    assert_eq!(super::is_symmetric(t11), true);

    //    1
    //   / \
    //  2   2
    //   \   \
    //    3   3
    let t2 = vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)];
    let t22 = build_tree(t2);
    assert_eq!(super::is_symmetric(t22), false);
  }

  #[test]
  pub fn test_min_depth() {
    //      3
    //     / \
    //    9  20
    //      /  \
    //     15   7
    let t1 = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let t11 = build_tree(t1);
    assert_eq!(super::min_depth(t11), 2);
  }

  #[test]
  pub fn test_sum_of_left_leaves() {
    //      3
    //     / \
    //    9  20
    //      /  \
    //     15   7
    let t1 = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

    let t11 = build_tree(t1);

    assert_eq!(super::sum_of_left_leaves(t11), 24);
  }

  #[test]
  pub fn test_longest_univalue_path() {
    //      1
    //     / \
    //    4   5
    //   / \   \
    //  4   4   5
    let t1 = vec![Some(1), Some(4), Some(5), Some(4), Some(4), None, Some(5)];

    let t11 = build_tree(t1);

    assert_eq!(super::longest_univalue_path(t11), 2);
  }

  #[test]
  pub fn test_rob() {
    //    3
    //   / \
    //  2   3
    //   \   \
    //    3   1
    let t1 = vec![Some(3), Some(2), Some(3), None, Some(3), None, Some(1)];
    let t11 = build_tree(t1);
    assert_eq!(super::rob_old(t11.clone()), 7);
    assert_eq!(super::rob(t11), 7);
  }

  #[test]
  pub fn test_find_second_minimum_value() {
    //    2
    //   / \
    //  2   5
    //     / \
    //    5   7
    let t1 = vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)];
    let tree = build_tree(t1);
    assert_eq!(super::find_second_minimum_value(tree), 5);
    //    2
    //   / \
    //  2   2
    let t2 = vec![Some(2), Some(2), Some(2)];
    let tree2 = build_tree(t2);
    assert_eq!(super::find_second_minimum_value(tree2), -1);
  }
}
