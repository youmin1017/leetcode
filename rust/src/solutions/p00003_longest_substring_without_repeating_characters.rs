use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut max_len = 1;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut l = 0;

        for (i, c) in s.chars().enumerate() {
            map.entry(c)
                .and_modify(|v| {
                    if *v < l {
                        *v = i;
                        return;
                    }
                    max_len = max_len.max(i - l);
                    l = *v + 1;
                    *v = i;
                })
                .or_insert_with(|| {
                    max_len = max_len.max(i - l + 1);
                    i
                });
        }
        max_len as i32
    }
}
