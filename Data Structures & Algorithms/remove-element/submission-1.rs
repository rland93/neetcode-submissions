impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n_removed = 0;
        let mut i = 0;
        let l = nums.len();

        while (i < l - n_removed)  {
            if val == nums[i] {
                let q = nums[i];
                nums[i] = nums[l - n_removed - 1];
                nums[l - n_removed - 1] = q;
                n_removed += 1;
            } else {
                i += 1;
            }
        }

        // remove swapped elements from the back 
        let keptn = l - n_removed;
        println!("{}", keptn);
        nums.truncate(keptn);

        keptn as i32
    }
}
