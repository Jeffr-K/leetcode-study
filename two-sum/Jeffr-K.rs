
/// TwoSum
/// :💬Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// :💬You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// :💬You can return the answer in any order.
/// :💾Link:https://leetcode.com/problems/two-sum/description/
/// :⏳TC:O(n)
/// :💾SC:O(n)
pub fn twoSum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::new();

    for (index, num) in nums.iter().enumurate() {
        let diff = target - num;
        if let Some(&j) = m.get(&diff) {
          return vec![j as i32, i as i32];
        }
        
        m.insert(num, i)
    }

    vec![]
}

#[cfg(test)]
mod tests {
    struct Testcase {
        nums: Vec<i32>,
        target: i32,
        output: Vec<i32>,
    }
  
  #[test]
  fn testTwoSum() {
      let testcases = vec![
          Testcase { nums: vec![2, 7, 11, 15], target: 9, output: vec![0, 1] },
          Testcase { nums: vec![3, 2, 4], target: 6, output: vec![1, 2] },
          Testcase { nums: vec![3, 3], target: 6, output: vec![0, 1] },
      ];
      
      for case in cases {
          let mut test = twoSum(case.nums.clone(), case.target);
          let mut expected = case.output.clone();
          
          test.sort();
          expected.sort();

          assert_eq!(test, expected);
      }
  }
}