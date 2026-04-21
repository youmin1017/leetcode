use std::collections::{HashMap, hash_map::Entry};

struct Solution;

fn main() {
    let mut s = String::from("ADOBECODEBANC");
    let mut t = String::from("ABC");

    println!("res = {}", Solution::min_window(s, t));

    s = String::from("OUZODYXAZV");
    t = String::from("XYZ");
    println!("res = {}", Solution::min_window(s, t));

    s = String::from("ab");
    t = String::from("a");
    println!("res = {}", Solution::min_window(s, t));

    s = String::from("aaaaaaaaaaaabbbbbcdd");
    t = String::from("abcdd");
    println!("res = {}", Solution::min_window(s, t));
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }
        if s == t {
            return s;
        }

        let mut target_map: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            target_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let s: Vec<char> = s.chars().collect();
        let mut p_next = None;
        let mut res = String::new();
        let mut l: usize = 0;
        while l <= s.len() - t.len() {
            if !target_map.contains_key(&s[l]) {
                l += 1;
                continue;
            }
            let mut r = l + 1;
            let mut map = target_map.clone();
            map.entry(s[l]).and_modify(|v| *v -= 1);

            if map.iter().all(|(_, v)| *v == 0) {
                return String::from(s[l]);
            }

            while r < s.len() {
                match map.entry(s[r]) {
                    Entry::Occupied(mut v) => {
                        if p_next.is_none() {
                            p_next = Some(r);
                        }
                        *v.get_mut() = 0.max(*v.get() - 1);
                        if map.iter().all(|(_, v)| *v == 0) {
                            break;
                        }
                    }
                    Entry::Vacant(_e) => {}
                }
                r += 1;
            }
            if r < s.len() && ((r - l + 1) < res.len() || res.is_empty()) {
                res = s[l..=r].iter().collect();
            }

            if let Some(p) = p_next {
                l = p;
                p_next = None;
            } else {
                l += 1;
            }
        }

        res
    }
}
