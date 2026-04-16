struct Solution;

fn main() {
    let height = vec![
        1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984, 983,
        982, 981, 980, 979, 978, 977, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        966, 966, 966, 966, 966, 966,
    ];

    println!("{}", Solution::trap(height));
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut total_area = 0;

        let mut l = 0;
        while l < height.len() - 1 {
            while height[l + 1] > height[l] {
                l += 1;
                if l == height.len() - 1 {
                    return total_area;
                }
                continue;
            }
            let mut r = l + 1;
            let mut area = 0;
            while r < height.len() && height[l] > height[r] {
                area -= height[r];
                r += 1;
            }

            // not found heigher than left till the end, count from right to left
            if r == height.len() {
                r -= 1; // set r to end
                while r > 0 && r > l + 1 {
                    while height[r - 1] > height[r] {
                        r -= 1;
                        if r == 0 {
                            return total_area;
                        }
                        continue;
                    }
                    let mut l2 = r - 1;
                    let mut area2 = 0;
                    while l2 > l && height[r] > height[l2] {
                        area2 -= height[l2];
                        l2 -= 1;
                    }
                    area2 += height[r] * (r - l2 - 1) as i32;
                    total_area += area2;
                    r = l2;
                }
                break;
            }

            area += height[l] * (r - l - 1) as i32;
            total_area += area;
            l = r;
        }
        total_area
    }
}
