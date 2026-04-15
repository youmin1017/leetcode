struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: usize = 0;
        while i < numbers.len() {
            let n = target - numbers[i];
            if let Ok(idx) = numbers.binary_search(&n)
                && i != idx
            {
                return vec![i as i32 + 1, idx as i32 + 1];
            }
            i += 1;
        }

        vec![-1, -1]
    }
}
