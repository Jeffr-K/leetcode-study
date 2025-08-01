use std::collections::HashSet;

/// Contains Duplicate 
/// :💬Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
/// :💾Link: https://leetcode.com/problems/contains-duplicate/description/
/// :⏳TC:O(n)
/// :💾SC:O(n)
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    
    for num in nums.iter() {
        if let false = set.insert(num) {
            return true
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contains_duplicate() {
        let nums1 = vec![1,2,3,1];
        let nums2 = vec![1,2,3,4];
        let nums3 = vec![1,1,1,3,3,4,3,2,4,2];
        assert_eq!(contains_duplicate(nums1), true);
        assert_eq!(contains_duplicate(nums2), false);
        assert_eq!(contains_duplicate(nums3), true);
    }
}