impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.extend_from_slice(&nums.clone());
        nums
    }
}
