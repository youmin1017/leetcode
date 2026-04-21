struct Solution;

fn main() {
    // #1
    let strs = vec![String::from("#")];
    let encoded = Solution::encode(strs);
    println!("{}", encoded);
    let decoded = Solution::decode(encoded);
    println!("{:?}", decoded)
}

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();
        for s in strs.into_iter() {
            res.push_str(format!("{}#{}", s.len(), s).as_str());
        }

        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut n_str = String::new();
        let mut is_decoding = false;
        let mut res = Vec::new();
        let mut size: Option<i32> = None;
        let mut decoded = String::new();
        for c in s.chars() {
            if !is_decoding && c == '#' {
                is_decoding = true;
            }
            if !is_decoding {
                n_str.push(c);
                continue;
            }

            match size {
                Some(sz) => {
                    decoded.push(c);

                    let new_sz = sz - 1;
                    size = Some(new_sz);

                    if new_sz == 0 {
                        size = None;
                        is_decoding = false;
                        res.push(decoded.to_owned());
                        decoded.clear();
                    }
                }
                None => {
                    size = n_str.parse().ok();
                    n_str.clear();

                    // if 0 size push empty string
                    if size.unwrap() == 0 {
                        size = None;
                        res.push(String::new());
                        is_decoding = false;
                    }
                }
            }
        }

        if size == Some(0) {
            res.push(String::new());
        }

        res
    }
}
