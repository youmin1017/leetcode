struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();

        for (i, n) in nums.iter().enumerate() {
            if *n > 0 {
                break;
            }

            if i > 0 && *n == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1
                    }
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        res
    }
}
