// `use` trait to get functionality

/// https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/description/
/// 1. 有序数组的 Two Sum
/// 167. Two Sum II - Input array is sorted (Easy)
#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
   assert!(!numbers.is_empty());
   let mut s = 0;
   let mut e = numbers.len() - 1;
   let mut result = Vec::new();

   while s < e {
      let sum = numbers[s] + numbers[e];
      if sum == target {
         result.push(s as i32 + 1);
         result.push(e as i32 + 1);
         return result;
      } else if sum > target {
         // 將末端(指向值比較大的)的指針向前(值比較小的方向)移動
         e = e - 1;
      } else {
         s = s + 1;
      }
   }
   result
}

/// https://leetcode-cn.com/problems/sum-of-square-numbers/description/
/// 2. 两数平方和
/// 在元素为 0~target 的有序数组中查找两个数，使得这两个数的平方和为 target，
/// 如果能找到，则返回 true，表示 target 是两个整数的平方和。
/// 633. Sum of Square Numbers (Easy)
#[allow(dead_code)]
pub fn judge_square_sum(target: i32) -> bool {
   let mut s = 0;
   // 右指针的初始化实现剪枝，使用平方根，只需要遍歷0~sqrt(target)
   let mut e = (target as f64).sqrt() as i32;
   while s <= e {
      let pow_sum = s * s + e * e;
      if pow_sum == target {
         return true;
      } else if pow_sum > target {
         e = e - 1;
      } else {
         s = s + 1;
      }
   }
   false
}

/// https://leetcode-cn.com/problems/reverse-vowels-of-a-string/description/
/// 3. 反转字符串中的元音字符
/// 345. Reverse Vowels of a String (Easy)
#[allow(dead_code)]
pub fn reverse_vowels(s: String) -> String {
   if s.is_empty() {
      return "".to_string();
   }

   use std::collections::HashSet;
   let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
      .into_iter()
      .collect();

   let mut i = 0;
   let mut j = s.len() - 1;
   let mut ss = s.into_bytes();
   while i < j {
      let l = ss[i];
      let r = ss[j];

      if !(vowels.contains(&(l as char))) {
         ss[i] = l;
         i = i + 1;
      } else if !(vowels.contains(&(r as char))) {
         ss[j] = r;
         j = j - 1;
      } else {
         ss[i] = r;
         ss[j] = l;
         i = i + 1;
         j = j - 1;
      }
   }
   return String::from_utf8(ss).unwrap();
}

/// https://leetcode-cn.com/problems/valid-palindrome-ii/description/
/// 4. 回文字符串
/// 680. Valid Palindrome II (Easy)
#[allow(dead_code)]
pub fn valid_palindrome(s: String) -> bool {
   let mut vc = s.chars();
   let mut rs = true;
   loop {
      let (lower, _) = vc.size_hint();
      if lower <= 0 {
         break;
      }
      let kk = vc.as_str();
      let l = vc.next();
      let r = vc.next_back();
      if l.is_some() && r.is_some() && l != r {
         rs = is_palindrome(&kk[0..kk.len() - 1]) || is_palindrome(&kk[1..]);
         break;
      }
   }
   pub fn is_palindrome(a: &str) -> bool {
      let mut s = 0;
      let mut e = a.len() - 1;
      let m = a.as_bytes();
      while s < e {
         if m[s] != m[e] {
            return false;
         }
         s = s + 1;
         e = e - 1;
      }
      return true;
   }
   return rs;
}

/// https://leetcode-cn.com/problems/merge-sorted-array/description/
/// 5. 归并两个有序数组
/// 88. Merge Sorted Array (Easy)
#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
   // num1最後一個元素的索引
   let mut n1_r = m - 1;
   // num2最後一個元素的索引
   let mut n2_r = n - 1;
   // num1末尾索引
   let mut r = m + n - 1;

   while n1_r >= 0 || n2_r >= 0 {
      if n1_r < 0 {
         // 說明nums1的執行完了，只需要移動剩餘的nums2
         nums1[r as usize] = nums2[n2_r as usize];
         r = r - 1;
         n2_r = n2_r - 1;
      } else if n2_r < 0 {
         // 說明nums2的執行完了，只需要移動剩餘的nums1
         nums1[r as usize] = nums1[n1_r as usize];
         r = r - 1;
         n1_r = n1_r - 1;
      } else if nums1[n1_r as usize] > nums2[n2_r as usize] {
         // 移動nums1的元素，並將索引移到下一個待判斷的元素上
         nums1[r as usize] = nums1[n1_r as usize];
         r = r - 1;
         n1_r = n1_r - 1;
      } else {
         // 移動nums2的元素，並將索引移到下一個待判斷的元素上
         nums1[r as usize] = nums2[n2_r as usize];
         r = r - 1;
         n2_r = n2_r - 1;
      }
   }
   println!("{:?}", nums1);
}

/// https://leetcode-cn.com/problems/linked-list-cycle/description/
/// 6. 判断链表是否存在环
/// 141. Linked List Cycle (Easy)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List<T> {
   head: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(PartialEq, Eq, Clone, Debug)]
struct Node<T> {
   elem: T,
   next: Link<T>,
}
#[allow(dead_code)]
impl<T> List<T> {
   pub fn new() -> Self {
      List { head: None }
   }
   pub fn push(&mut self, elem: T) {
      let new_node = Box::new(Node {
         elem: elem,
         next: self.head.take(),
      });
      self.head = Some(new_node);
   }
}
#[allow(dead_code)]
pub fn has_cycle(head: List<i32>, i: i32) -> bool {
   let head = head.head.as_ref();
   let mut slow = head;
   let mut fast = head;
   while fast.is_some() {
      println!(
         "start fast {} {}",
         fast.unwrap().next.is_none(),
         fast.unwrap().elem
      );
      let nx = fast.unwrap().next.as_ref();
      if nx.is_none() {
         return false;
      }
      println!("slow {} fast {}", slow.unwrap().elem, fast.unwrap().elem);
      slow = slow.unwrap().next.as_ref();
      fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
      // 如果是None，說明已經循環完了，此時需要將fast指向到i所在的位置。
      if fast.is_none() && i >= 0 {
         let mut tmp = head;
         for _ in 0..i {
            tmp = tmp.unwrap().next.as_ref();
         }
         fast = tmp;
      }
      if slow.is_none() && i >= 0 {
         let mut tmp = head;
         for _ in 0..i {
            tmp = tmp.unwrap().next.as_ref();
         }
         slow = tmp;
      }
      println!(
         "slow-next {} fast-next {}",
         slow.unwrap().elem,
         fast.unwrap().elem
      );
      if slow == fast {
         return true;
      }
   }
   return false;
}

/// https://leetcode-cn.com/problems/longest-word-in-dictionary-through-deleting/description/
/// 7. 最长子序列
/// 524. Longest Word in Dictionary through Deleting (Medium)
#[allow(dead_code)]
pub fn find_longest_word(s: String, d: Vec<String>) -> String {
   use std::cmp::Ordering;
   if s.is_empty() || d.is_empty() {
      return "".to_string();
   }
   let mut longest_word = String::new();
   for target in d {
      let l1 = longest_word.len();
      let l2 = target.len();
      // 1.如果當前longest_word比當前的target要長，則直接判斷下一個target
      // 2.或者當前longest_word和target一樣長，且longest_word是字典序的最小字符串，則直接判斷下一個target
      if l1 > l2 || (l1 == l2 && longest_word.cmp(&target) == Ordering::Less) {
         continue;
      }
      if is_substr(&s, &target) {
         longest_word = target;
      }
   }
   // 判斷s(s可以去掉一些沒有的字符)是否是target的子串
   fn is_substr(s: &str, target: &str) -> bool {
      let mut i = 0;
      let mut j = 0;
      let s1 = s.as_bytes();
      let t1 = target.as_bytes();
      while i < s.len() && j < target.len() {
         if s1[i] == t1[j] {
            j = j + 1;
         }
         i = i + 1;
      }
      return j == target.len();
   }
   return longest_word;
}

#[cfg(test)]
mod tests {
   use super::List;

   #[test]
   fn test_two_sum() {
      let mut numbers = Vec::new();
      numbers.push(2);
      numbers.push(7);
      numbers.push(11);
      numbers.push(15);

      assert_eq!(super::two_sum(numbers, 9), [1, 2]);
   }

   #[test]
   fn test_judge_square_sum() {
      assert!(super::judge_square_sum(9), true);
      assert!(super::judge_square_sum(2), false);
   }

   #[test]
   fn test_reverse_vowels() {
      assert_eq!(super::reverse_vowels("leetcode".to_string()), "leotcede");
      assert_eq!(super::reverse_vowels("hello".to_string()), "holle");
      assert_eq!(super::reverse_vowels("a.".to_string()), "a.");
   }

   #[test]
   fn test_valid_palindrome() {
      assert_eq!(super::valid_palindrome("aba".to_string()), true);
      assert_eq!(super::valid_palindrome("abc".to_string()), false);
      assert_eq!(super::valid_palindrome("abca".to_string()), true);
      assert_eq!(super::valid_palindrome("cbbcc".to_string()), true);
      assert_eq!(super::valid_palindrome("eccer".to_string()), true);
      assert_eq!(super::valid_palindrome("cxcaac".to_string()), false);
      assert_eq!(super::valid_palindrome("abcbfa".to_string()), true);
   }

   #[test]
   fn test_merge() {
      super::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);

      super::merge(&mut vec![2, 2, 2, 0, 0, 0], 3, &mut vec![2, 2, 2], 3);
   }

   #[test]
   fn test_has_cycle() {
      let mut list = List::new();
      list.push(-4);
      list.push(0);
      list.push(2);
      list.push(3);
      assert_eq!(super::has_cycle(list, 1), true);

      let mut list = List::new();
      list.push(1);
      assert_eq!(super::has_cycle(list, -1), false);

      let mut list = List::new();
      list.push(2);
      list.push(1);
      assert_eq!(super::has_cycle(list, 0), true);
   }

   #[test]
   fn test_find_longest_word() {
      let s = String::from("abpcplea");
      let d = vec![
         "ale".to_string(),
         "apple".to_string(),
         "monkey".to_string(),
         "plea".to_string(),
      ];
      assert_eq!(super::find_longest_word(s, d), "apple");

      let s = String::from("abpcplea");
      let d = vec!["a".to_string(), "b".to_string(), "c".to_string()];
      assert_eq!(super::find_longest_word(s, d), "a");
   }
}
