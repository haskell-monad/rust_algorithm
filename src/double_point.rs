// `use` trait to get functionality

/// https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/description/
/// 1. 有序数组的 Two Sum
/// 167. Two Sum II - Input array is sorted (Easy)
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
pub fn reverse_vowels(s: String) -> String {
   if s.is_empty() {
      return "".to_string();
   }

   use std::collections::HashSet;
   let vowels: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].into_iter().collect();

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
           rs = is_palindrome(&kk[0..kk.len()-1]) || is_palindrome(&kk[1..]);
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

#[cfg(test)]
mod tests {

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
}
