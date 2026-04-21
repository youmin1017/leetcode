struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered_str: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let reversed: String = filtered_str.chars().rev().collect();

        let mut a = filtered_str.clone();
        a.push_str(&reversed);

        let mut b = reversed.clone();
        b.push_str(&filtered_str);

        a == b
    }
}
