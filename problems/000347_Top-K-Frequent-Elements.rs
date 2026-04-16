use std::{cmp::Ordering, collections::HashMap};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums.into_iter() {
            map.entry(n).and_modify(|n| *n += 1).or_insert(0);
        }

        let mut sorted: Vec<(i32, i32)> = map.into_iter().collect();
        sorted.sort_by_key(|v| -v.1);

        sorted.into_iter().map(|v| v.0).take(k as usize).collect()
    }
}

struct Num {
    pub n: i32,
    pub f: i32,
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.f.cmp(&other.f))
    }
    // fn cmp(&self, other: &Self) -> Ordering {
    //     self.f.cmp(&other.f)
    // }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        self.f.eq(&other.f)
    }
}
