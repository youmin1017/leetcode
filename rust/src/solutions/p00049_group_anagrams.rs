use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut sorted: Vec<char> = s.chars().collect();
            sorted.sort();
            let sorted_str: String = sorted.into_iter().collect();
            map.entry(sorted_str)
                .and_modify(|v| v.push(s.to_owned()))
                .or_insert(vec![s.to_owned()]);
        }

        map.into_values().collect()
    }
}
