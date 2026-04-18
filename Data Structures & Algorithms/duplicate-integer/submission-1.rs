impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i] == nums[i-1] {
                return true;
            }
        }
        return false;
    }
}
