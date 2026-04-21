struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix: Vec<i32> = vec![1];
        let mut suffix: Vec<i32> = vec![1];
        let mut res = Vec::new();

        let mut sum: i32 = 1;
        for n in nums.iter() {
            sum *= n;
            prefix.push(sum);
        }

        sum = 1;
        for n in nums.iter().rev() {
            sum *= n;
            suffix.push(sum);
        }

        for i in 0..nums.len() {
            res.push(prefix[i] * suffix[nums.len() - i - 1]);
        }
        res
    }
}

// prefix: [1,1,2,6,24]
// suffix: [1,4,12,24,24]
