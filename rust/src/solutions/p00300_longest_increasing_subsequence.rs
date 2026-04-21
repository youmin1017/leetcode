use std::collections::BTreeSet;
struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let set: BTreeSet<i32> = nums.into_iter().collect();

        let mut max_len = 1;
        let mut len = 1;
        println!("{:?}", set);
        for n in set.iter() {
            if !set.contains(&(n - 1)) {
                max_len = max_len.max(len);
                len = 1;
                continue;
            }
            len += 1;
        }
        max_len.max(len)
    }
}
