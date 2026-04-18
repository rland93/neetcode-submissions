impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut most_common = nums[0];
        for i in 1..nums.len() {
            if nums[i] == most_common {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                most_common = nums[i];
                count = 1;
            }
        }
        return most_common as i32;
    }
}
