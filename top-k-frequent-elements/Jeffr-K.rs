/// Top K Frequent Elements
/// :💬Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
/// :💾Link:https://leetcode.com/problems/top-k-frequent-elements/description/
/// :⏳TC:O(n)
/// :💾SC:O(n)
pub fn topKFrequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let m = HashMap::new();

    for num in nums {
       *m.entry(num).or_insert(0) += 1
    }

    let mut heap = BinaryHeap::new();
    for (num, freq) in m {
      heap.push(Reverse(freq, num)));
      if heap.len() > k as usize {
        heap.pop();
      }
    }
    
    heap.into_iter().map(|Reverse((_, num))| num).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let result = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert_eq!(result.len(), 2);
    }
}
