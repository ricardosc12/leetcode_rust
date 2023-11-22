struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut pars = 0;
        for i in 0..nums.len() - 1 {
            for j in (i+1)..nums.len() {
                if nums[i] == nums[j] {
                    pars += 1;
                }
            }
        }
        
        pars
    }
}