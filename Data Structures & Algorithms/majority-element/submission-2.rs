impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut candidate = nums[0];
        let mut count:i32 = 1;
        for i in 1..l {
            if candidate == nums[i] {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                candidate = nums[i];
                count = 1;
            }
        }
        return candidate;
    }
}
