struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = heights.len() - 1;
        let mut max_area = 0;

        while l < r {
            let area = heights[l].min(heights[r]) * (r - l) as i32;
            max_area = max_area.max(area);
            if heights[l] < heights[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_area
    }
}

// Input: height = [1,7,2,5,4,7,3,6]
// 1 <-> 6 = 7 l < r, so l += 1
// 7 <-> 6 = 36 l > r, so r -= 1
// continue
