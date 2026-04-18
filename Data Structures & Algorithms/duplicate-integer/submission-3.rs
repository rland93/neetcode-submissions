use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums.iter() {
            if !set.insert(n) {
                return true;
            }
        }
        false
    }
}
