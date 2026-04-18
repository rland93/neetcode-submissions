use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for j in 0..nums.len() {
            let complement = target - nums[j];
            match map.get(&nums[j]) {
                Some(i) => {return vec![*i as i32, j as i32 ];},
                None => {map.insert(complement, j);}
            }
        }
        unreachable!("Problem definition");
    }
}