use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (j, n) in nums.iter().enumerate() {
            let companion = target - nums[j];
            match map.get(n) {
                Some(i) => {
                    return vec![*i as i32, j as i32];
                },
                None => {
                    map.insert(companion, j);
                }
            }
        }
        panic!("bad input.");
    }
}
