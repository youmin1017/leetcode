struct Solution;

fn main() {
    let ts = vec![30, 38, 30, 36, 35, 40, 28];
    println!("{:?}", Solution::daily_temperatures(ts));
}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut s: Vec<usize> = Vec::new();
        let mut res: Vec<usize> = vec![0; temperatures.len()];
        for (i, t) in temperatures.iter().enumerate() {
            if s.is_empty() || *t <= temperatures[*s.last().unwrap()] {
                s.push(i);
                continue;
            } else {
                while !s.is_empty() && *t > temperatures[*s.last().unwrap()] {
                    let last = s.pop().unwrap();
                    res[last] = i - last;
                }
                s.push(i)
            }
        }
        res.into_iter().map(|v| v as i32).collect()
    }
}
