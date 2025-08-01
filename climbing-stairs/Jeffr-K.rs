pub struct Solution;

impl Solution {
    /// TC: O(n)
    /// SC: O(n)
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = HashMap::new();
        memo.insert(0, 1);
        memo.insert(1, 1);

        for i in 2..=n {
            let val = memo.get(&(i - 1)).unwrap() + memo.get(&(i - 2)).unwrap();
            memo.insert(i, val);
        }

        *memo.get(&n).unwrap()
    }