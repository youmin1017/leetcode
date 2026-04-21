use std::collections::VecDeque;

struct Solution;

fn main() {
    let nums = vec![1, 2, 1, 0, 4, 2, 6];
    let k = 3;
    println!("{:?}", Solution::max_sliding_window(nums, k));

    let nums = vec![
        -6, -10, -7, -1, -9, 9, -8, -4, 10, -5, 2, 9, 0, -7, 7, 4, -2, -10, 8, 7,
    ];
    let k = 7;
    println!("{:?}", Solution::max_sliding_window(nums, k));
}

// TL in leetcode
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut current_max = i32::MIN;
        let mut max_index = 0;
        let k = k as usize;

        for (i, n) in nums[0..k].iter().enumerate() {
            if *n >= current_max {
                current_max = *n;
                max_index = i;
            }
        }

        let mut res = vec![current_max];
        for i in 1..=nums.len() - k {
            if i > max_index {
                current_max = i32::MIN;
                for j in i..i + k {
                    if nums[j] > current_max {
                        current_max = nums[j];
                        max_index = j;
                    }
                }
                res.push(current_max);
            } else if nums[i + k - 1] >= current_max {
                current_max = nums[i + k - 1];
                max_index = i + k - 1;
                res.push(current_max);
            } else {
                // current max in window and current max > nums[i]
                res.push(current_max);
            }
        }

        res
    }

    pub fn max_sliding_window_ac(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut deque: VecDeque<usize> = VecDeque::new(); // 存 index，值遞減
        let mut res = Vec::with_capacity(n - k + 1);

        for i in 0..n {
            // 1. 移除過期的 index（已不在窗口內）
            if let Some(&front) = deque.front() {
                if front + k <= i {
                    deque.pop_front();
                }
            }

            // 2. 從後端移除所有比當前值小的 index（它們永遠不會是答案）
            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            // 3. 窗口形成後才記錄答案
            if i + 1 >= k {
                res.push(nums[*deque.front().unwrap()]);
            }
        }

        res
    }
}
