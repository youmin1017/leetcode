use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s1.chars() {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let s2: Vec<char> = s2.chars().collect();
        for w in s2.windows(s1.len()) {
            let mut mapw: HashMap<char, i32> = map.clone();
            let mut next_win = false;
            for c in w {
                mapw.entry(*c).and_modify(|v| *v -= 1).or_insert_with(|| {
                    next_win = true;
                    -1
                });
                if next_win {
                    break;
                }
            }
            if next_win {
                continue;
            }
            if mapw.into_iter().all(|(_, v)| v == 0) {
                return true;
            }
        }
        false
    }
}
